use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Serializer;

use crate::errors::GroupError;
use crate::errors::ItemError;
use crate::utils::Path;
use crate::utils::to_json;

/// Database struct housing groups and relations between items.
pub struct Db {
    name: String,
    groups: HashMap<String, Group>,
    relations:HashMap<String, Relation>,
    
}

impl Db {
    pub fn new(name: String) -> Db {
        return Db {
            name,
            groups: HashMap::<String, Group>::new(),
            relations: HashMap::<String, Relation>::new(),
        };
    }

    /// Finds and returns group with item of type `I`, or creates it if it is not present.
    fn get_group(&mut self, name:String) -> Result<&mut Group, GroupError> {
        if self.groups.contains_key::<String>(&name) {
            let mut group = self.groups.get_mut(&name);

            if let Some(mut group) = group{
                return Ok(group);
            }
            else {
                return Err(GroupError::GroupNotFound { name });
            }
        }
        else {
            return Err(GroupError::GroupNotFound { name })
        }
    }

    pub fn new_group(&mut self, name: String) -> Result<(), GroupError> {
        if self.groups.contains_key::<String>(&name) {
            Err(GroupError::GroupExists { name: name })
        }
        else {

            let group = Group::new(name.clone());
            self.groups.insert(name, group.clone());

        return Ok(());
        }
    }

    // Inserts new item into group.
    pub fn insert<T: Serialize>(&mut self, path: String, data: T) -> Result<(), ItemError> {
        let bytes = serde_json::to_string(&data).unwrap();

        let path = Path::from_string(path.clone());
        let json = to_json::<T>(data);

        let item = Item{key: path.item_name, path: path.path, data: json};

        let mut group: &mut Group = self.get_group(path.group_name.clone()).unwrap();

        group.items.insert(item.key.clone(), item);

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    pub items: HashMap<String, Item>,
    pub name: String,    
}

impl Group {
    fn new(name: String) -> Group {
        Group { items: HashMap::<String, Item>::new(), name}
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item {
    pub key: String,
    pub path: String,
    pub data: String
}

// impl Item {
//     pub fn new(path: String) -> Item {
        
//     }
// }

pub struct Relation {}