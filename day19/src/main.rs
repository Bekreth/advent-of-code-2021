extern crate regex;
extern crate lazy_static;

mod spatial;
mod bit_map;
mod scanner;

use std::fs;
use std::env;
use std::string::String;

use lazy_static::lazy_static;
use regex::Regex;

use spatial::Point;
use scanner::Scanner;

enum Either {
    Left(Scanner),
    Right(Point),
}

fn main() {
    lazy_static! {
        static ref POINT_REGEX: Regex = Regex::new(r"^--- scanner (\d+) ---$")
            .expect("Failed to parse point regex");
    }

    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("missing filename");

    let content = fs::read_to_string(filename)
        .expect("failed to read file");
    let scanners = content.lines()
        .filter_map(|s| {
            match POINT_REGEX.captures(s) {
                Some(capture) => {
                    let id = capture[1].parse::<u8>().expect("Should have parse scanner");
                    Some(Either::Left(Scanner::new(id)))
                },
                None => {
                    if s != "" {
                        Some(Either::Right(Point::new(s)))
                    } else {
                        None
                    }
                },
            }
        })
        .fold(vec![], |mut acc: Vec<Scanner>, either| {
            match either {
                Either::Left(scanner) => acc.push(scanner),
                Either::Right(beacon) => {
                    let mut current_point = acc.pop()
                        .expect("Should have gotten a point");
                    current_point.append(beacon);
                    acc.push(current_point);
                },
            }
            acc
        });

}
