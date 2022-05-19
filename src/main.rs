use opcua::server::prelude::*;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

fn main() {
    let config: ServerConfig = ServerConfig::load(&PathBuf::from("server.conf")).unwrap();
    let mut server: Server = Server::new(config.clone());

    let ns = {
        let address_space = server.address_space();
        let mut address_space = address_space.write().unwrap();
        address_space.register_namespace("urn:Test").unwrap();
        address_space.register_namespace("urn:RL").unwrap()
    };

    mock(&mut server, ns);

    let host = config.tcp_config.host;
    let port = config.tcp_config.port;
    println!("Server started on {}:{}", host, port);

    server.run();
}

fn mock(server: &mut Server, n: u16) {
    let sensor_names = [
        "\"+AL-BG2_Bandsensor_Rechts\"",
        "\"+AL-BG1_Bandsensor_Links\"",
        "\"+AL-BG4_Stopper_Rechts\"",
        "\"+AL-BG3_Stopper_Links\"",
        "\"+AO-BG1_PosLinks\"",
        "\"+AO-BG2_PosRechts\"",
        "\"+AO-BG3_StopperLinks\"",
        "\"+AO-BG4_StopperRechts\"",
        "\"+AO-BG5_VereinzelerOffen\"",
        "\"+AO-BG6_HubUnten\"",
        "\"+AO-BG7_HubMitte\"",
        "\"+AO-BG8_HubOben\"",
        "\"+AM-BG1_PosLinks\"",
        "\"+AM-BG2_PosRechts\"",
        "\"+AM-BG3_StopperLinks\"",
        "\"+AM-BG4_StopperRechts\"",
        "\"+AM-BG5_VereinzelerOffen\"",
        "\"+AM-BG6_HubUnten\"",
        "\"+AM-BG7_HubMitte\"",
        "\"+AM-BG8_HubOben\"",
        "\"+AN-BG1_InduktivUnten\"",
        "\"+AM-BL1_Füllstand\"",
        "\"+AO-BL2_Füllstand\"",
        "\"+AL-MA1.Rechtslauf\"",
        "\"+AL-MA1.Linkslauf\"",
        "\"+AL-MB2_Stopper_Rechts\"",
        "\"+AL-MB1_Stopper_Links\"",
        "\"+AL-MA1.Vmax\"",
        "\"+AO-MB1_StopperLinks\"",
        "\"+AO-MB2_StopperRechts\"",
        "\"+AO-MB3_Vereinzeler\"",
        "\"+AO-MB4_HubUnten\"",
        "\"+AO-MB5_HubOben\"",
        "\"+AM-MB1_StopperLinks\"",
        "\"+AM-MB2_StopperRechts\"",
        "\"+AM-MB3_Vereinzeler\"",
        "\"+AM-MB4_HubUnten\"",
        "\"+AM-MB5_HubOben\"",
    ];
    let sensor_list: Vec<NodeId> = sensor_names
        .iter()
        .map(|name| NodeId::new(n, name.clone()))
        .collect();

    let address_space = server.address_space();

    // The address space is guarded so obtain a lock to change it
    {
        let mut address_space = address_space.write().unwrap();

        // Create a sample folder under objects folder
        let sample_folder_id = address_space
            .add_folder("Sample", "Sample", &NodeId::objects_folder_id())
            .unwrap();

        let variables: Vec<Variable> = sensor_list
            .iter()
            .map(|e| Variable::new(&e, "", "", false))
            .collect();

        // Add some variables to our sample folder. Values will be overwritten by the timer
        let _ = address_space.add_variables(variables, &sample_folder_id);
    }

    {
        // Store a counter and a flag in a tuple
        let data = Arc::new(Mutex::new((0, true)));
        server.add_polling_action(300, move || {
            let mut data = data.lock().unwrap();
            data.1 = !data.1;
            let mut address_space = address_space.write().unwrap();
            let now = DateTime::now();
            sensor_list.iter().for_each(|e| {
                let _ = address_space.set_variable_value(e.clone(), data.1, &now, &now);
            });
        });
    }
}
