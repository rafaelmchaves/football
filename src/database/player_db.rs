use mongodb::{
    bson::{doc, Bson},
    error::Result,

    // Note that the sync version of the Client is in the `sync`
    // module rather than at the top-level of the driver’s namespace.
    sync::Client
};
use mongodb::sync::Collection;



pub fn get_player_by_the_name(name: String) -> Result<PlayerDB> {

    let coll = get_collection()?;

    let cursor = coll.find(doc! {"name": name}, None)?;

    let mut team_option: Option<PlayerDB> = None;

    for result in cursor {
        team_option = Some(bson::from_bson(Bson::Document(result?))?);
    }

    Ok(team_option.unwrap())
}

pub fn save(player_db: PlayerDB) -> Result<Bson> {
    let coll = get_collection()?;

    let doc2 = bson::to_bson(&player_db).unwrap();

    let doc = doc! {
        "name": &player_db.name
    };

    Ok(coll.insert_one(doc, None).unwrap().inserted_id)

}

fn get_collection() -> Result<Collection> {
    let client =
        Client::with_uri_str("mongodb://localhost:27017")?;

    // Get a handle to the collection being used.
    let coll = client
        .database("football")
        .collection("players");
    Ok(coll)
}

use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;
use crate::basic::team::Team;

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerDB {

    #[serde(rename = "_id")]
    pub id: String,
    pub name: String,
    pub age: u8,
    pub main_position: String,
    pub nation: String,
    pub team: String,

}


