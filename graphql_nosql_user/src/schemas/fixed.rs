use juniper::{GraphQLInputObject, GraphQLObject};
use serde::{Serialize, Deserialize};

/// Product
#[derive(GraphQLObject, Serialize, Deserialize, Debug, Clone)]
pub struct Fixed {
    pub id: String,
    pub name : String
}

#[derive(GraphQLInputObject, Serialize, Deserialize, Debug, Clone)]
#[graphql(description = "Fixed Input")]
pub struct FixedInput {
    pub name: String
}
