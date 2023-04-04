use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::errors::GroupError;
use crate::errors::ItemError;

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
    fn get_group(&self, name:String) -> Result<&Group, GroupError> {
        if self.groups.contains_key::<String>(&name) {
            let group = self.groups.get(&name);

            if let Some(group) = group{
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
    pub fn new_item<T: Serialize>(&mut self, group: String) -> Result<(), ItemError> {
                

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
    pub id: u64,
    pub path: String,
    pub data: String
}

impl Item {
    // pub fn new(id: u64) -> Item {

    // }
}

pub struct Relation {}