extern crate termion;
extern crate chrono;
extern crate regex;

use std::io;
use std::env;
use std::fs;
use termion::color;
use chrono::prelude::*;
use regex::Regex;


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

  let today           = Local::now();
  let mut flight_date = String::new();

  let avi_regex  = Regex::new(r".AVI$").unwrap();

  println!("\r");
  println!("{green}Which quad?{reset}",
    green  = color::Fg(color::Green),
    reset = color::Fg(color::Reset)
  );

  io::stdin().read_line(&mut quad_name)
    .expect("Failed to read line");


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
    new_directory.trim().to_string()
  };

  println!("\r");
  println!("{green}When?{magenta}      ({now}){reset}",
    green    = color::Fg(color::Green),
    magenta  = color::Fg(color::Magenta),
    reset    = color::Fg(color::Reset),
    now      = today.format("%m/%d/%Y").to_string()
  );

  io::stdin().read_line(&mut flight_date)
    .expect("Failed to read line");

  let date_of_flight: DateTime<Local> = match Local::datetime_from_str(&Local, &flight_date, "%m/%d/%Y") {
    Ok(dt) => dt,
    Err(_) => today
  };

  let paths = fs::read_dir(&dir).unwrap();

  let mut i = 1;
  for path in paths {
    let p       = path.unwrap().path();
    let display = p.display().to_string();

    if p.is_file() && avi_regex.is_match(&display) {
      let content_type = match p.extension() {
        None => "",
        Some(os_str) => {
          if os_str == "avi" {
            "avi"
          } else if os_str == "AVI" {
            "AVI"
          } else {
            "suckit"
          }
        }
      };
      let new_filename = format!("{}-{}-{}.{}",
        quad_name.trim(),
        date_of_flight.format("%m%d%Y").to_string(),
        i,
        content_type
      );
      let with_path = format!("{}/{}", dir, new_filename);
      fs::rename(&display, &with_path);
      println!("{} ==> {}", display, with_path);

      i += 1;
    }
  }
}
