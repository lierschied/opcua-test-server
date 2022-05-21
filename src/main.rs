use opcua::server::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

mod sensor;
use sensor::SensorConfig;

fn main() {
    let config: ServerConfig = ServerConfig::load(&PathBuf::from("server.conf")).unwrap();
    let mut server: Server = Server::new(config.clone());

    let _ns = {
        let address_space = server.address_space();
        let mut address_space = address_space.write().unwrap();
        address_space.register_namespace("urn:Test").unwrap();
        address_space.register_namespace("urn:RL").unwrap()
    };

    mock(&mut server);

    let host = config.tcp_config.host;
    let port = config.tcp_config.port;
    println!("Server started on {}:{}", host, port);

    server.run();
}

fn mock(server: &mut Server) {
    let sensor_config: SensorConfig = SensorConfig::load();
    let address_space = server.address_space();

    // The address space is guarded so obtain a lock to change it
    {
        let mut address_space = address_space.write().unwrap();

        // Create a sample folder under objects folder
        let sample_folder_id = address_space
            .add_folder("Sample", "Sample", &NodeId::objects_folder_id())
            .unwrap();

        let variables: Vec<Variable> = sensor_config.create_variables();
        // Add some variables to our sample folder. Values will be overwritten by the timer
        let _ = address_space.add_variables(variables, &sample_folder_id);
    }

    {
        // Store a counter and a flag in a tuple
        let data = Arc::new(Mutex::new((0, true)));
        server.add_polling_action(300, move || {
            let mut data = data.lock().unwrap();
            data.0 += 1;
            data.1 = !data.1;
            let mut address_space = address_space.write().unwrap();
            let now = DateTime::now();

            let nodes = sensor_config.node_list();

            sensor_config.sensors.iter().enumerate().for_each(|(i, s)| {
                let to_set = match s.value.as_str() {
                    "int" => data.0.to_string(),
                    "bool" => data.1.to_string(),
                    _ => panic!("No value set for {}", s.id),
                };
                let _ = address_space.set_variable_value(nodes[i].clone(), to_set, &now, &now);
            });
        });
    }
}
