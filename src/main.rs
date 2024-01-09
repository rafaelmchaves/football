#![feature(proc_macro_hygiene, decl_macro)]

mod web;
mod basic;
mod database;

use crate::web::player_rest;
use crate::web::team_rest;

#[macro_use] extern crate rocket;

fn main() {
  rocket::ignite().mount("/", routes![player_rest::get_players, team_rest::teams, team_rest::create_team]).launch();
}
