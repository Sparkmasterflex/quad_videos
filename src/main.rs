extern crate termion;
extern crate chrono;

use std::io;
use std::env;
use std::path::Path;
use std::fs::{self, DirEntry};
use termion::color;
use chrono::prelude::*;

fn main() {
  println!("{cyan}=============================={reset}",
    cyan  = color::Fg(color::Cyan),
    reset = color::Fg(color::Reset)
  );
  println!("{cyan}|     Rename FPV Footage     |{reset}",
    cyan  = color::Fg(color::Cyan),
    reset = color::Fg(color::Reset)
  );
  println!("{cyan}=============================={reset}",
    cyan  = color::Fg(color::Cyan),
    reset = color::Fg(color::Reset)
  );

  let mut quad_name         = String::new();

  let current_directory = env::current_dir().unwrap().display().to_string();
  let mut new_directory = String::new();

  // let today           = Local::now();
  // let mut flight_date = String::new();

  // println!("\r");
  // println!("{green}Which quad?{reset}",
  //   green  = color::Fg(color::Green),
  //   reset = color::Fg(color::Reset)
  // );

  // io::stdin().read_line(&mut quad_name)
  //   .expect("Failed to read line");


  println!("\r");
  println!("{green}Where?{magenta}      ({dir}){reset}",
    green    = color::Fg(color::Green),
    magenta  = color::Fg(color::Magenta),
    reset    = color::Fg(color::Reset),
    dir      = current_directory
  );

  io::stdin().read_line(&mut new_directory)
    .expect("Failed to read line");


  let dir = if new_directory == "\n" {
    current_directory
  } else {
    new_directory
  };

  // println!("\r");
  // println!("{green}When?{magenta}      ({now}){reset}",
  //   green    = color::Fg(color::Green),
  //   magenta  = color::Fg(color::Magenta),
  //   reset    = color::Fg(color::Reset),
  //   now      = today.format("%m/%d/%Y").to_string()
  // );

  // io::stdin().read_line(&mut flight_date)
  //   .expect("Failed to read line");

  // let date_of_flight: DateTime<Local> = match Local::datetime_from_str(&Local, &flight_date, "%m/%d/%Y") {
  //   Ok(dt) => dt,
  //   Err(_) => today
  // };

  let paths = fs::read_dir(dir)

  for path in paths {
    println!("Name: {}", path.unwrap().path().display());
  }
}
