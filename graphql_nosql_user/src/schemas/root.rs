use futures::TryStreamExt;
use juniper::{
    graphql_object, EmptySubscription, FieldResult, RootNode,
};
use mongodb::{Client, bson::doc};

use crate::schemas::{user::{User, UserInput}, fixed::{Fixed, FixedInput}};

pub struct Context {
    pub client: Client,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[graphql_object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "List of all users")]
    async fn users(context: &Context) -> FieldResult<Vec<User>> {
        let client = context.client.clone();
        let collection = client.database(&"test").collection(&"users");
        let cursors = collection.find(None, None).await.ok().expect("Error");   
        let users = cursors.try_collect().await;
        println!("{:?}", users);
        Ok(users.unwrap())
    }

    #[graphql(description = "Get Single user reference by user ID")]
    async fn user(context: &Context, id: String) -> FieldResult<User> {
        let client = context.client.clone();
        let collection = client.database("test").collection("users");
        let filter = doc! {"user_id": id};
        let user = collection.find_one(filter, None).await.ok().expect("Error");
        Ok(user.unwrap())
    }

    #[graphql(description = "Get Single user reference by user ID")]
    async fn fixeds(context: &Context) -> FieldResult<Vec<Fixed>> {
        let client = context.client.clone();
        let collection = client.database("test").collection("fixeds");
        let cursors = collection.find(None, None).await.ok().expect("Error");
        let fixeds = cursors.try_collect().await;
        Ok(fixeds.unwrap())
    }
}

pub struct MutationRoot;

#[graphql_object(Context = Context)]
impl MutationRoot {
    async fn create_user(context: &Context, user: UserInput) -> FieldResult<String> {
        let client = context.client.clone();
        let collection = client.database("test").collection("users");
        let new_id = uuid::Uuid::new_v4().simple().to_string();
        let new_doc = doc!{
            "user_id" : new_id,
            "fixed": user.fixed,
            "flex" : bson::to_bson(&user.flex).unwrap()
        };

        let id: String = collection.insert_one(new_doc, None).await.expect("Error").inserted_id.to_string();
        Ok(id)    
    }

    async fn create_fixed(context: &Context, fixed: FixedInput) -> FieldResult<String> {
        let client = context.client.clone();
        let collection = client.database("test").collection("fixeds");
        let new_id = uuid::Uuid::new_v4().simple().to_string();
        let new_doc = doc!{
            "id": new_id,
            "name": fixed.name
        };

        let id: String = collection.insert_one(new_doc, None).await.expect("Error").inserted_id.to_string();
        Ok(id)    
    }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<Context>>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot, EmptySubscription::new())
}