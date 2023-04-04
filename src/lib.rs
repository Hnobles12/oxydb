
// use serde::{Deserialize, Serialize};
//! Docs for the entire crate.

/// Module db
mod db;
mod errors;
mod utils;

use db::Db;



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imports() {
        let db = Db::new("testdb".to_string());
    }

    // #[test]
    // fn group_not_found() {
    //     let db = Db::new("test".to_string());
    //     match db.get_group("group".to_string()) {
    //         Ok(group) => panic!("Group found, when no group should be present."),
    //         Err(e) => assert!(true)
    //     }
    // }
}
