use std::io;

use lazy_static::lazy_static;
use regex::{Captures, Regex};

fn main() {
    println!("Hi. This tool converts Fahrenheit to Celsius and vice versa. Please enter the temperature followed by its unit (e.g. 69f or -30c):");

    let parsed_input = loop {
        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line!");

        let temperature = temperature.trim(); // remove whitespace/line breaks

        match parse_input(&temperature) {
            Ok(res) => {
                break res;
            }
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
    };

    let converted_temp: TemperatureWithUnit = match parsed_input.unit {
        TemperatureUnit::F => TemperatureWithUnit { temp: f_to_c(parsed_input.temp), unit: TemperatureUnit::C },
        TemperatureUnit::C => TemperatureWithUnit { temp: c_to_f(parsed_input.temp), unit: TemperatureUnit::F }
    };

    println!("Converted temperature: {}{:?}", converted_temp.temp, converted_temp.unit)
}

fn f_to_c(f: i32) -> i32 {
    (f - 32) * 5 / 9
}

fn c_to_f(c: i32) -> i32 {
    (c * 9 / 5) + 32
}

#[derive(Debug)]
enum TemperatureUnit {
    F,
    C,
}

struct TemperatureWithUnit {
    temp: i32,
    unit: TemperatureUnit,
}

fn parse_input(input: &str) -> Result<TemperatureWithUnit, &str> {
    lazy_static! {
        static ref TEMPERATE_INPUT_REGEX: Regex = Regex::new(
            r"^(\-?\d+)(F|C)$"
            ).unwrap();
    }

    let maybe_caps: Option<Captures> = TEMPERATE_INPUT_REGEX.captures(input);

    match maybe_caps {
        None => Err("Invalid input. Please be sure to enter temps in the following format: [-][digits][F/C]. Examples: -10C, 70F"),
        Some(caps) => {
            let temperature: i32 = match caps.get(1).unwrap().as_str().parse() {
                Ok(temp) => temp,
                // should I just panic here? Regex validation should already account for this
                Err(_) => return Err("Failed to parse the temperature component.")
            };
            let unit: char = match caps.get(2).unwrap().as_str().parse() {
                Ok(u) => u,
                Err(_) => return Err("Failed to parse the unit component.")
            };

            let converted_unit = match unit {
                'F' => TemperatureUnit::F,
                'C' => TemperatureUnit::C,
                _ => return Err("Invalid temperature unit.")
            };

            Ok(TemperatureWithUnit {
                temp: temperature,
                unit: converted_unit,
            })
        }
    }
}
