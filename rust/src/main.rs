use std::{collections::btree_map::Values, fs};
// use std::string::String;
use chrono::Local;

struct User {
    first_name: String,
    last_name: String,
}

impl User {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
}

// MAIN FUNC

fn main() {
    println!("{}", is_even(-11));
    println!("{}", fib(1));

    // string implementaion
    let my_string = String::from("hello, baby ~🥳");
    let length = get_string_length(&my_string);
    print!("THe number of characters in the string is:{}", length);

    // struct
    let user = User {
        first_name: String::from("Rahul"),
        last_name: String::from("Sain"),
    };

    println!("{}", user.first_name);
    println!("Full name: {}", user.full_name());

    // enums
    let r = Shape::Rectangle(1.1, 1.2);
    let c = Shape::Circle(1.2);

    println!("{}", calculate_area(r));
    println!("{}", calculate_area(c));

    // Result
    // let ans = greeting_from_file(file_path:String::from("hello.txt"));

    match greeting_from_file("hello.txt".to_string()) {
        Ok(content) => println!("File content: {}", content),
        Err(err) => println!("Error reading file: {}", err),
    }

    // package manager
    let now = Local::now();
    println!("current {}", now);

    // borrowing
    let s1 = String::from("Rahul");
    let s2 = &s1;

    println!("owner: {}", s1);
    println!("borrower: {}", s2);

    // vector
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);

    let ans = even_filter(&vec);

    println!("{:?}", ans);
    println!("{:?}", vec);
}

// FUNC -> ALL

fn is_even(num: i32) -> bool {
    num % 2 == 0
}

fn fib(num: i32) -> i32 {
    let mut first: i32 = 0;
    let mut second: i32 = 1;

    if num == 0 {
        return first;
    }
    if num == 1 {
        return second;
    }

    for _ in 0..(num - 1) {
        // Changed to include num with ..=
        let temp = first;
        first = second;
        second = temp + first;
    }

    second
}

fn get_string_length(s: &str) -> usize {
    s.chars().count()
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Rectangle(a, b) => a * b,
    }
}

fn greeting_from_file(file_path: String) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            new_vec.push(*val);
        }
    }
    new_vec
}
