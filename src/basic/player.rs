use crate::basic::position::Position;
use crate::basic::team::Team;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
  pub(crate) id: String,
  pub(crate) name: String,
  pub(crate) age: u8,
  pub(crate) main_position: Position,
  pub(crate) nation: String,
  pub(crate) team: String,
}