use futures::stream::StreamExt;
use mongodb::{
    bson::{doc, from_bson, to_bson},
    Client,
};
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
struct User {gender: String,
    name: Name,
    email: String,
    dob: Dob,
}

#[derive(Debug, Serialize, Deserialize)]
struct Name {
first: String,
last: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Dob {
date: String,
    age: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   
let client = Client::with_uri_str("mongodb://localhost:27017/").await?;
  let db = client.database("users");
    let coll = db.collection("random");
    let response = reqwest::get("https://randomuser.me/api/?results=10").await?;
    let data = response.json::<ApiResponse>().await?;
    let users = data.results;

    
    for user in &users {
        let doc = to_bson(&user)?;
        coll.insert_one(doc.as_document().unwrap().to_owned(), None).await?;
    }

    
    let mut cursor = coll.find(doc! {}, None).await?;
    while let Some(result) = cursor.next().await {
        let doc = result?;
        let user = from_bson::<User>(mongodb::bson::Bson::Document(doc))?;
        println!("{:?}", user);
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct ApiResponse {
    results: Vec<User>,
}

