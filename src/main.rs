#![feature(proc_macro_hygiene, decl_macro)]

mod basic;
use crate::basic::player::Player;
use crate::basic::position::Position;
use crate::basic::team::{Team, Colors};

use rocket_contrib::json::{Json};

use serde::{Serialize, Deserialize};

#[macro_use] extern crate rocket;

fn main() {
  rocket::ignite().mount("/", routes![index]).launch();
}


#[get("/players", format = "json")]
fn index() -> Json<Player> {

  let team = Team {
    id: "dsad".to_string(),
    name: "Cruzeiro".to_string(),
    color_1: Colors::WHITE,
    color_2: Colors::WHITE,
    stadium: "Mineirao".to_string(),
    president: "Crazy man".to_string(),
    nation: "Brazil".to_string()
  };

  let player = Player {
    id: "12321421".parse().unwrap(),
    name: "Rafael".parse().unwrap(),
    age: 17,
    main_position: Position::GoalKeeper,
    nation: "String".parse().unwrap(),
    team,
  };

  println!("id player {}", player.id);
  println!("player name {}", player.name);
  println!("team name {}", player.team.name);

  Json(player)
}
