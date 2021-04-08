#![feature(proc_macro_hygiene, decl_macro)]

mod basic;

use crate::basic::player::Player;
use crate::basic::position::Position;
use crate::basic::team::{Team, Colors};

mod database;
use crate::database::team_db;

use rocket_contrib::json::{Json};
use rocket::request::Form;

#[macro_use] extern crate rocket;

fn main() {
  rocket::ignite().mount("/", routes![index, teams, create_team]).launch();
}

#[get("/players", format = "json")]
fn index() -> Json<Player> {

  let player = Player {
    id: "12321421".parse().unwrap(),
    name: "Rafael".parse().unwrap(),
    age: 17,
    main_position: Position::GoalKeeper,
    nation: "String".parse().unwrap(),
    team: "".parse().unwrap()
  };

  Json(player)
}

#[get("/teams?<shortname>", format = "json")]
fn teams(shortname: String) -> Json<Team> {
  let team_db = team_db::get_team_by_short_name(shortname).unwrap();

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
  let team_db = team_db::get_team_by_short_name(team_req).unwrap();

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