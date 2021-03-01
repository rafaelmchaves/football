use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum Position {
  GoalKeeper,
  RightBack,
  LeftBack,
  CentreBack,
  DefensiveMidfield,
  Midfield,
  AttackingMidfield,
  RightAttackingMidfield,
  LeftAttackingMidfield,
  Forward,
  Striker,
  LeftWinger,
  RightWinger
}