
use rocket_contrib::json::{Json};
use crate::basic::player::Player;
use crate::basic::position::Position;

#[get("/players", format = "json")]
pub fn get_players() -> Json<Player> {

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