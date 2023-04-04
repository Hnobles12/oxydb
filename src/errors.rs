use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum GroupError {
    #[snafu(display("Group `{name}` could not be found"))]
    GroupNotFound { name: String},

    #[snafu(display("Group `{name}` already exists."))]
    GroupExists {name: String}
}

#[derive(Debug, Snafu)]
pub enum ItemError {
    #[snafu(display("Item with id `{id}` could not be found"))]
    ItemNotFound { id: String},

    #[snafu(display("Item with id `{id}` already exists."))]
    ItemExists {id: String}
}
// #[derive(Debug, Snafu)]
// pub enum DbError {
//     #[snafu(display("Group `{name}` could not be found"))]
//     GroupNotFound { name: String},

//     #[snafu(display("Group `{name}` already exists."))]
//     GroupExists {name: String},

//     #[snafu(display("Item with id `{id}` could not be found"))]
//     ItemNotFound { id: String},

//     #[snafu(display("Item with id `{id}` already exists."))]
//     ItemExists {id: String}
// }