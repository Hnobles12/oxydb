use serde::Serialize;
use serde_json;

pub struct Path {
    pub db_name: String,
    pub group_name: String,
    pub item_name: String,
    pub path: String
}

impl Path {
    pub fn from_string(path: String) -> Path {
        let parts: Vec<&str> = path.as_str().split(":").collect();
        match parts.len() {
            1 => {
                return Path {db_name: parts[0].to_string(), group_name: "".to_string(), item_name: "".to_string(), path}
            }
            2 => {
                
                return Path {db_name: parts[0].to_string(), group_name: parts[1].to_string(), item_name: "".to_string(), path}
            }
            3 => {

                return Path {db_name: parts[0].to_string(), group_name: parts[1].to_string(), item_name: parts[2].to_string(), path}
            }
            _ => {

                return Path {db_name: "".to_string(), group_name: "".to_string(), item_name: "".to_string(), path}
            }
        }

    }
}


pub fn to_json<T: Serialize>(data: T) -> String {
    return serde_json::to_string(&data).unwrap();
}