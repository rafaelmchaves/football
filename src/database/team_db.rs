use mongodb::{
    bson::{doc, Bson},
    error::Result,

    // Note that the sync version of the Client is in the `sync`
    // module rather than at the top-level of the driver’s namespace.
    sync::Client
};
use mongodb::sync::Collection;

pub fn get_team_by_short_name(shortname: String) -> Result<TeamDb> {

    let coll = get_collection()?;

    let cursor = coll.find(doc! {"short_name": shortname}, None)?;

    let mut team_option: Option<TeamDb> = None;

    for result in cursor {
        team_option = Some(bson::from_bson(Bson::Document(result?))?);
    }

    Ok(team_option.unwrap())
}

fn get_collection() -> Result<Collection> {
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
use crate::basic::team::Team;

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


