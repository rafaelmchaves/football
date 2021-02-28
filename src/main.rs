mod basic;
use crate::basic::player::Player;
use crate::basic::position::Position;
use crate::basic::team::{Team, Colors};

fn main() {
  println!("Hello, world!");

  let player = Player {
    id: "12321421".parse().unwrap(),
    name: "Rafael".parse().unwrap(),
    age: 17,
    main_position: Position::GoalKeeper,
    nation: "String".parse().unwrap(),
  };

  println!("id player {}", player.id);
  println!("player name {}", player.name);

  let team = Team {
    id: "dsad".to_string(),
    name: "Cruzeiro".to_string(),
    color_1: Colors::WHITE,
    color_2: Colors::WHITE,
    stadium: "Mineirao".to_string(),
    president: "Crazy man".to_string(),
    nation: "Brazil".to_string()
  };

  println!("team name {}", team.name);
  println!("stadium name {}", team.stadium);

}
