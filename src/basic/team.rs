use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
  pub id: String,
  pub name: String,
  pub color_1: Colors,
  pub color_2: Colors,
  pub stadium: String,
  pub president: String,
  pub nation: String,

}

#[derive(Serialize, Deserialize, Debug)]
pub enum Colors {
  WHITE,
  RED,
  BLUE,
  GREEN,
  ORANGE,
  YELLOW,
  PINK,
  BLACK
}