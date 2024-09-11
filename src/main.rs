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