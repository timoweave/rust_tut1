#![allow(unused)]

use rand::Rng;
use std::cmp::Ordering;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::num::ParseIntError;

fn main() {
    // ask_your_name();
    // another_example();
    // max_numbers();
    // boolean_stuff();
    // random_number();
    // ternay_op(10);
    // match_this();
    // match_that();
    // array_example()
    // array_loop();
    // tuple_stuff();
    // string_stuff();
    guess_number(1, 100);
}

fn guess_number(first: i32, last: i32) {

    let answer: i32 = rand::thread_rng().gen_range(first..=last);
    let mut guess: String = String::new();
    loop {
        println!("enter a guess number between {} to {}: ", first, last);
		guess.clear();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let number: i32 = match (guess
			.trim()
			.parse::<i32>()) {
				Ok(num) => num,
				Err(e) => {
					println!("Could not parse string: '{}', {}", guess.trim(), e);
        			continue;
				}
		};
		match number.cmp(&answer) {
			Ordering::Less => println!("too small"),
			Ordering::Greater => println!("too large"),
			Ordering::Equal => {
				println!("you win! ({} == {})", number, answer);
				break;
			},
		}
    }
}

fn string_stuff() {
    let mut str1: String = String::new();
    str1.push('A');
    str1.push(' ');
    str1.push_str("word");
    let str2: String = str1.replace('A', "Hello");
    println!("str1 = {str1}\nstr2 = {str2}");
}

fn tuple_stuff() {
    let tup: (i32, f32, String) = (160, 6.7, "Timothy".to_string());
    let (weight, height, name) = tup;
    println!("weigth = {}, height = {}, name = {}", weight, height, name);
    // println!("tup.0 = {}, tup.1 = {}, tup.2 = {}", tup.0, tup.1, tup.2.clone().to_string());
    println!();
}

fn array_loop() {
    let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut idx: usize = 0;
    loop {
        let val: i32 = arr[idx];
        if val % 2 == 0 {
            idx += 1;
            continue;
        }

        if val == 9 {
            break;
        }
        println!("value = {}", val);
        idx += 1;
    }
    println!();

    idx = 0;
    while idx < arr.len() {
        println!("value = {}", arr[idx]);
        idx += 1;
    }
    println!();

    for val in arr.iter() {
        if (val % 2) == 1 {
            continue;
        }
        println!("value = {}", val);
    }
    println!();
}

fn array_example() {
    let arr: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("arr[0] = {}", arr[0]);
    println!("arr.len() = {}", arr.len());
}

fn match_that() {
    let age: i32 = 1_000_000;
    let voting_age: i32 = 18;
    match age.cmp(&voting_age) {
        Ordering::Less => println!("You are not old enough to vote"),
        Ordering::Greater => println!("You are old enough to vote"),
        Ordering::Equal => println!("You are just old enough to vote"),
    }
}

fn match_this() {
    let age: i32 = 21;
    match age {
        1..=18 => println!("You are a child"),
        19..=20 => println!("You are a young adult"),
        21 | 50 => println!("You are a special age"),
        65..=i32::MAX => println!("You are a senior"),
        _ => println!("You are not a child"),
    };
}

fn ternay_op(age: i32) {
    let mut can_vote: bool = false;
	can_vote = age > 18;;
    println!("Can vote: {}", can_vote);
}

fn random_number() {
    let rnd: i32 = rand::thread_rng().gen_range(1..101);
    println!("Random number: {}", rnd);
}

fn boolean_stuff() {
    let x: bool = true;
    let y: bool = false;
    println!("x = {}\ny = {}\nx == y is {}", x, y, x == y);
    println!();
}

fn max_numbers() {
    println!("max u32 = {}", u32::MAX);
    println!("max u64 = {}", u64::MAX);
    println!("max f32 = {}", f32::MAX);
    println!("max f64 = {}", f64::MAX);
    println!();
}

fn another_example() {
    const ONE_MILLION: u32 = 1_000_000;
    const FLOATING_NUMBER: f64 = 2.1415;
    const AGE: &str = "40";
    let mut age: u32 = AGE.trim().parse().expect("age is not a number");
    age += 1;
    println!(
        "AGE = {}\nage = {}\nPI = {}\n1_MILLION = {}",
        AGE, age, FLOATING_NUMBER, ONE_MILLION
    );
    println!();
}

fn ask_your_name() {
    println!("Enter your name? ");
    let mut name: String = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!("Hello, {}!", name.trim_end());
    println!();
}
