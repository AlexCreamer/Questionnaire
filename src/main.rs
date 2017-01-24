
extern crate clap;

use clap::{Arg, App, SubCommand};
use std::env;
use std::fs::File;
use std::io::prelude::*;
//use argparse::{ArgumentParser, StoreTrue, Store};

fn main() {
    let matches = App::new("Questionare")
        .version("0.1.0")
        .about("Tests to help users test their skillsets")
        .author("Alex Creamer")
        .arg(Arg::with_name("Section")
            .short("s")
            .long("section")
            .value_name("#")
            .help("Sets the section of the Questionaire to work on")
            .takes_value(true))
        .arg(Arg::with_name("v")
            .short("v")
            .help("Sets the verbosity flag")
            .multiple(true))
        .get_matches();

      let section = matches.value_of("Section").unwrap_or("1");
      println!("Value of section is: {}", section);
    match section {
        "1" => section1(),
        _ => println!("Invalid section given")
    }
}

fn section1() {
      let mut correct = 0;

      let message: &str = "Question?
        1. response
        2. response
        3. response
        4. response";
      let answer: bool = test(1, message);
      if answer == true {
          correct += 1;
      }

      let message: &str = "Question?
        1. response
        2. response
        3. response
        4. response";
      let answer: bool = test(4, message);
      if answer == true {
         correct += 1;
     }

     let message: &str = "Question?
       1. response
       2. response
       3. response
       4. response";
      let answer: bool = test(4, message);
      if answer == true {
          correct += 1;
      }

      println!("Section 1 questionaire ended. The amount you got correct was: {}", correct);
}

fn test(expected_answer: u8, message: &str) -> bool {
    println!("{}", message);

    let check_answer = |answer: u8| {
        if answer == expected_answer {
            println!("Correct answer!");
            return true;
        } else {
            println!("Incorrect answer!");
            return false;
        }
    };

    loop {
        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer)
            .expect("Failed to read line");
        match answer.trim().parse() {
            Ok(num) => {return check_answer(num)},
            Err(_) => println!("Please enter a number."),
        };
    }
}
