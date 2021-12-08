use std::io::Read;
use std::ops::Deref;
use std::sync::Arc;
use crate::udp::Task;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Data {
    pub tcp_address: String,
    pub udp_address: String,
    pub seed: u64,
    pub init_flag: u64,
    pub final_flag: u64,
    #[serde(skip)] tasks: Option<Vec<Task>>,
}

impl Data {
    pub fn get_tasks(&self) -> &Vec<Task> {
        self.tasks.as_ref().unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    data: Arc<Data>,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let buffer = match std::fs::File::open("config.json") {
            Ok(mut file) => {
                let mut buffer = Vec::new();
                file.read_to_end(&mut buffer).unwrap();
                buffer
            }
            Err(error) => {
                return Err(error.to_string());
            }
        };

        let data = match serde_json::from_slice::<Data>(&buffer) {
            Ok(mut data) => {
                data.tasks = Some(Task::gen_tasks(data.seed));
                Arc::new(data)
            },
            Err(error) => {
                return Err(error.to_string());
            }
        };


        Ok(Self {
            data
        })
    }
}

impl Deref for Config {
    type Target = Data;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}
