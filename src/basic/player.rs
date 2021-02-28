use crate::basic::position::Position;

pub struct Player {
  pub(crate) id: String,
  pub(crate) name: String,
  pub(crate) age: u8,
  pub(crate) main_position: Position,
  pub(crate) nation: String,
}