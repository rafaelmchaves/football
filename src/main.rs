#![feature(proc_macro_hygiene, decl_macro)]

mod basic;


use crate::basic::team::{Team, Colors};

mod database;
mod web;

use crate::database::team_db;
use crate::web::player;
use rocket_contrib::json::{Json};
use rocket::request::Form;

#[macro_use] extern crate rocket;

fn main() {
  rocket::ignite().mount("/", routes![player::get_players, teams, create_team]).launch();
}



#[get("/teams?<shortname>", format = "json")]
fn teams(shortname: String) -> Json<Team> {
  let team_db = team_db::getTeamByShortName(shortname).unwrap();

  let team = Team {
    id: team_db.id.to_hex(),
    name: team_db.name,
    shortname: team_db.short_name,
    color_1: Colors::BLUE,
    color_2: Colors::WHITE,
    stadium: team_db.stadium,
    president: team_db.president,
    nation: team_db.nation
  };

  Json(team)
}

#[post("/teams", data = "<team_request>")]
fn create_team(team_request: Form<TeamRequest>) -> Json<Team> {

  let team_req = team_request.shortname.clone();
  let team_db = team_db::getTeamByShortName(team_req).unwrap();

  let team = Team {
    id: team_db.id.to_hex(),
    name: team_db.name,
    shortname: team_db.short_name,
    color_1: Colors::BLUE,
    color_2: Colors::WHITE,
    stadium: team_db.stadium,
    president: team_db.president,
    nation: team_db.nation
  };

  Json(team)
}

#[derive(FromForm)]
struct TeamRequest {
  pub name: String,
  pub shortname: String,
  pub color_1: String,
  pub color_2: String,
  pub stadium: String,
  pub president: String,
  pub nation: String,
}