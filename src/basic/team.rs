pub struct Team {
  pub id: String,
  pub name: String,
  pub color_1: Colors,
  pub color_2: Colors,
  pub stadium: String,
  pub president: String,
  pub nation: String,

}

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