use std::{collections::HashMap, fs::read_to_string, sync::mpsc, thread};
use chrono::{Local, Utc};

fn main() {
    let ans = is_even(2100000000);
    println!("{}", ans);
    let x: i32 = fib(10);
    println!("{}", x);
    let my_string = String::from("Hello, world!");
    let length = str_len(&my_string);
    println!("The number of characters in the string is: {}", length);
    let user = create_user(&String::from("John"), &String::from("Doe"), 22);
    println!("User age is {}", user.age);

    let rect1 = Rect {
        width: 10,
        height: 20,
    };
    println!("Area of rect1 is {}", rect1.area());
    println!("Perimeter of rect1 is {}", rect1.perimeter());

    let my_direction = Direction::North(0.0, 1.0);
    let new_direction = my_direction;
    move_around(new_direction);

    let index = find_first_a(String::from("John Doe"));
    match  index {
       Some(value) => println!("Index is {}", value),
       None => println!("Index not found"), 
    }

    read_from_file();
    using_chrono();
    using_vectors();
    using_hashmaps();
    using_iterators();
    using_strings_and_slices();
    use_traits();
    use_lifetimes();
    using_threads();
    using_channels();
}

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    }
    return false;
}

fn fib(num: i32) -> i32 {
    let mut first = 0;
    let mut second = 1;
    if num == 0 {
        return first;
    }
    if num == 1 {
        return second;
    }

    for _i in 0..num-1 {
        let temp = second;
        second = second + first;
        first = temp;
    }
    return second;
}

fn str_len(s: &str) -> usize {
    s.chars().count()
}

struct User {
    first_name: String,
    last_name: String,
    age: i32,
}

fn create_user(first_name: &str, last_name: &str, age: i32) -> User {
    let user = User {
        first_name: String::from(first_name),
        last_name: String::from(last_name),
        age: age,
    };
    println!("{} {} is {}", user.first_name, user.last_name, user.age);
    return user;
}

struct Rect {
    width: i32,
    height: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn perimeter(&self) -> i32 {
        2 * (self.width + self.height)
    }
}

enum Direction {
    North(f32, f32),
    East(f32, f32),
    South(f32, f32),
    West(f32, f32),
}

fn move_around(direction: Direction) -> f32 {
    let xy = match direction {
        Direction::North(a,b) => a+b,
        Direction::East(a, b ) => a+b,
        Direction::South(a, b) => a+b,
        Direction::West(a, b) => a+b,
    };
    return xy;
}

fn find_first_a(s: String) -> Option<i32> {
    for (index, char) in s.chars().enumerate() {
        if char == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

fn read_from_file() {
    let greetings_file_result = read_to_string("he.txt");

    match greetings_file_result {
        Ok(file_content) => {
            println!("File read successfully: {:?}", file_content);
        }
        Err(error) => {
            println!("Failed to read file: {:?}", error);
        }
    }
}


fn using_chrono() {
    let now = Utc::now();
    println!("Current date and time in UTC: {}", now);

    let formatted = now.format("%Y-%m-%d %H:%M:%S");
    println!("Formatted date and time is: {}", formatted);

    let local = Local::now();
    println!("Current local time is: {}", local);
}

fn using_vectors() {
    let vec = vec![1,2,3,4]; //Vec::new();
    //vec.push(1);
    //vec.push(2);

    println!("{:?}", even_filter(&vec));
    println!("{:?}", vec);
}

fn even_filter(vec: &Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in vec {
        if val % 2 == 0 {
            new_vec.push(*val);
        }
    }
    return new_vec;
}

fn using_hashmaps() {
    let mut users: HashMap<String, u32> = HashMap::new();

    users.insert(String::from("John"), 22);
    users.insert(String::from("Doe"), 15);

    let first_user_age = users.get("John");
    match first_user_age {
        Some(age) => println!("First user age is: {}", age),
        None => println!("User not found!"),
    };
    let input_vec = vec![(String::from("John"), 22), (String::from("Doe"), 32)];
    let hm = group_value_by_keys(input_vec);
    println!("{:?}", hm);
}

fn group_value_by_keys(vec: Vec<(String, u32)>) -> HashMap<String, u32> {
    let mut hm = HashMap::new();
    for (key, val) in vec {
        hm.insert(key, val);
    }

    return hm;
}

fn using_iterators() {
    let mut v1 = vec![1, 2, 3, 4, 5];
    let v1_iter = v1.iter();

    // for val in v1_iter {
    //     println!("{}", val);

    // }
    let sum: i32 = v1_iter.sum();
    println!("{:?}, Sum: {}", v1, sum);

    let mut v1_iter_mut = v1.iter_mut();

    // for val in v1_iter_mut {
    //     *val = *val + 1;
    // }
    // println!("{:?}", v1);

    while let Some(val) = v1_iter_mut.next() {
        println!("{}", val);
    }

    let v2 = vec![1,2,3,4,5];
    let v2_iter = v2.iter();

    let v2_iter2 = v2_iter.map(|x| x+1);
    for i in v2_iter2 {
        println!("{i}");
    }
    let v3 = vec![1,2,3,4,5,6];
    let v3_iter = v3.iter();
    let v3_iter3 = v3_iter.filter(|x| *x % 2 == 0);
    for i in v3_iter3 {
        println!("{i}");
    }

    println!("{:?}", v2);
}

fn using_strings_and_slices() {
    let mut name = String::from("John Doe");
    name.push_str("dove");
    println!("Name is {}", name);
    name.replace_range(3..name.len(), "");
    println!("name is {}", name);
    let mut new_name = String::from("John Doe");
    let ans = &new_name[0..4];
    println!("slice is {}", ans);
}

pub trait Summary {
    fn summarize(&self) -> String;
}

struct Profile {
    name: String,
    age: u32,
}

impl Summary for Profile {
    fn summarize(&self) -> String {
        return format!("The name is {}. Age is {}.", self.name, self.age);
    }
}

fn use_traits() {
    let profile = Profile {
        name: String::from("John Doe"),
        age: 21,
    };

    println!("{}", profile.summarize());
    notify(&profile);
}

fn notify(profile: &impl Summary) {
    println!("{}", profile.summarize());
}

struct User2<'a> {
    name: &'a str,
}

fn use_lifetimes() {
    let name = String::from("Joh Wick");
    let user2 = User2 {
        name: &name
    };
    print!("{}", user2.name);
}


fn using_threads() {
    thread::spawn(|| {
        for i in 0..5 {
            println!("Hi from spawned thread {}", i);
        }
    });

    for i in 0..20 {
        println!("Hi from main thread {} ", i);
    }
}

fn using_channels() {
    let (tx, rx) = mpsc::channel();

    let prod = tx.clone();
    thread::spawn(move || {
        let val = String::from("Hi");
        prod.send(val).unwrap();
    });
    drop(tx);

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}