use juniper::{GraphQLInputObject, GraphQLObject};
use ::serde::{Serialize, Deserialize};


#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct Flex {
    key: String, 
    value: String
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
pub struct InputFlex {
    key: String, 
    value: String
}


/// User
#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub user_id: String,
    pub fixed: Vec<String>,
    pub flex: Vec<Flex>
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
pub struct UserInput {
    pub fixed: Vec<String>,
    pub flex: Vec<InputFlex>
}