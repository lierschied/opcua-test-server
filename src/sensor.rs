use opcua::server::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SensorConfig {
    pub sensors: Vec<Sensor>,
}

impl SensorConfig {
    pub fn create_variables(&self) -> Vec<Variable> {
        self.sensors
            .iter()
            .map(|s| {
                let node = NodeId::new(s.node_id.clone(), s.id.clone());
                Variable::new(&node, &s.browse_name, &s.display_name, false)
            })
            .collect()
    }

    pub fn node_list(&self) -> Vec<NodeId> {
        self.sensors
            .iter()
            .map(|s| NodeId::new(s.node_id.clone(), s.id.clone()))
            .collect()
    }

    pub fn load() -> SensorConfig {
        let sensors_config_file =
            std::fs::File::open("SensorsConfig.yaml").expect("Could not read SensorsConfig.yaml");

        serde_yaml::from_reader(sensors_config_file).expect("Unable to parse SensorConfig")
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Sensor {
    pub id: String,
    pub node_id: u16,
    pub browse_name: String,
    pub display_name: String,
    pub value: String,
}
