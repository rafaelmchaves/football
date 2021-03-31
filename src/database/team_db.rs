use mongodb::{
    bson::{doc, Bson},
    error::Result,

    // Note that the sync version of the Client is in the `sync`
    // module rather than at the top-level of the driver’s namespace.
    sync::Client
};
use mongodb::sync::Collection;

pub fn getTeamByShortName(shortname: String) -> Result<TeamDb> {

    let coll = getCollection()?;

    let mut cursor = coll.find(doc! {"short_name": shortname}, None)?;

    let mut team: TeamDb = TeamDb {
        id: Default::default(),
        name: "".to_string(),
        short_name: "".to_string(),
        color_1: "".to_string(),
        color_2: "".to_string(),
        stadium: "".to_string(),
        president: "".to_string(),
        nation: "".to_string()
    };

    for result in cursor {
        team = bson::from_bson(Bson::Document(result?))?;
        println!("result {}", team.name);
    }

    Ok(team)
}

fn getCollection() -> Result<Collection> {
    let client =
        Client::with_uri_str("mongodb://localhost:27017")?;

    // Get a handle to the collection being used.
    let coll = client
        .database("football")
        .collection("teams");
    Ok(coll)
}

use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamDb {

    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub short_name: String,
    pub color_1: String,
    pub color_2: String,
    pub stadium: String,
    pub president: String,
    pub nation: String,

}


