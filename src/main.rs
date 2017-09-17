extern crate rand;

use std::io;

fn main() {
    hello_1_2();
    div();
    // guessing_game_2();
    variables_3_1();
    div();
    data_types_3_2();
    div();
    functions_3_3();
    div();
    control_flow_3_5();
    div();
    ownership_4_1();
    div();
    references_and_borrowing_4_2();
    div();
    slices_4_3();
    div();
    structs_define_and_instantiate_5_1();
    div();
    using_structs_5_2();
    div();
    method_syntax_5_3();
    div();
    defining_an_enum_6_1();
}

fn div() {
    println!("--------------------------------------------------------------------------------");
}

fn hello_1_2() {
    println!("Hello, world!") // macro and implicit return
}

#[allow(dead_code)]
fn guessing_game_2() {
    use rand::Rng;
    use std::cmp::Ordering;

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_number);
    loop {
        println!("Please input your guess:");

        let mut guess = String::new(); // static method

        io::stdin() // static method
            .read_line(&mut guess) // read into mut ref
            .expect("Failed to read line"); // expect a Result, print string on error
        // a Result is an enum that can be either a Ok<type> or Err<type>

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input!");
                continue;
            }
        }; // match on the result of parse

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

fn variables_3_1() {
    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is now {}", x);
}

#[allow(unused_variables)]
fn data_types_3_2() {
    // scalar types
    let decimal: u32= 98_222;
    let hexadecimal: u64 = 0xdeadbeef;
    let octal: u8= 0o77;
    let binary: u8 = 0b1010_1010;
    let byte: u8 = b'B';
    let floating_point: f64 = 4.2;

    // booleans
    let a = true;
    let b: bool = false;

    let heart_eyed_cat = '😻'; // unicode!!!
    println!("{}", heart_eyed_cat);

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    println!("The value of x is: {}", tup.0);

    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    println!("The value of first is: {}", first);
}

#[allow(unused_variables)]
fn functions_3_3() -> u32{
    let x = 8; // statement, performs an action and does not return a value
    let y = {
        let x = 3;
        x + 2
    }; // expression, returns a value
    y
}

fn control_flow_3_5() {
    let number = 3;
    if number > 5 {
        println!("{} was bigger than 5", number);
    } else {
        println!("{} was smaller than 5", number);
    }

    let test = if true {
        88
    } else {
        0
    };
    println!("test was assigned a value of {}", test);

    let mut i = 0;

    loop {
        i += 1;
        if i >= 10 {
            break;
        }
        println!("loop: {}", i);
    }

    while i >= -5 {
        i -= 1;
        println!("while: {}", i);
    }

    let a = [10, 20, 30, 40, 50];
    for (k, v) in a.iter().rev().enumerate() {
        println!("The value of element {} is {}", k, v);
    }

}

fn ownership_4_1() {
    println!("1. Each value in Rust has a variable that's called its owner.");
    println!("2. There can only be one owner at a time.");
    println!("3. When the owner goes out of scope, the value will be dropped.");
    println!("Generally, stack allocated variables are deep copied (implement Copy trait) and heap allocated variables are moved(implement Drop trait) as opposed to shallow copy.");
    println!(".clone is used for a deep copy.");
}

fn references_and_borrowing_4_2() {
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    fn mutate_string(s: &mut String) {
        s.push_str("ADDED");
    }
    let s1 = String::from("Hello there!");
    let len_s1 = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len_s1);

    let mut s2 = String::from("I am a string");
    mutate_string(&mut s2);
    println!("The string was mutated to '{}'.", s2);

    println!("You can only have one mutable ref to a piece of data at a time in the same scope to prevent data races.");
    println!("Data races happen when:");
    println!("1. Two or more pointers access the same data at the same time.");
    println!("2. At least one of the pointers is used to write to the data.");
    println!("3. There's no mechanism used to synchronize access to the data.");

    println!("You cannot return a dangling reference because the memory it points to will be deallocated when the function returns.");
}

fn slices_4_3() {
    fn first_word_naive(s: &String) -> usize {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }

        s.len()
    }

    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }

        &s[..]
    }

    fn first_word_best(s: &str) -> &str {
        // can work with both String and &str
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }

        &s[..]
    }

    let s1 = String::from("test of the first word fn");
    println!("string is {}", s1);
    println!("return value of first_word_naive is {}", first_word_naive(&s1));
    println!("return value of first_word is {}", first_word(&s1));
    println!("return value of first_word_best is {}", first_word_best(&s1[..]));

    println!("string literals are string slices into the compiled binary");
    println!("strings are not the only thing you can slice");
}

fn structs_define_and_instantiate_5_1() {
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // dot notation for access
    user1.email = String::from("anotheremail@example.com");

    println!("{:?}", user1);

    // shorthand for making struct with fields that have the same name as variables in scope
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    println!("{:?}", build_user(String::from("e@mail.com"), String::from("this is my name.")));

    // copy from another instance with the same struct type
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println!("{:?}", user2);

    // tuple structs, fields have no names
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // unit-like struct
    struct Unit();

    let _ = Color(0, 0, 0);
    let _ = Point(0, 0, 0);
    let _ = Unit();

}

fn using_structs_5_2() {
    #[derive(Debug)]
    struct Rectangle {
        length: u32,
        width: u32,
    }

    let rect1 = Rectangle { length: 50, width: 30 };
    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.length * rectangle.width
    }

    // nicer debug print
    println!("rect1 is {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn method_syntax_5_3() {
    #[derive(Debug)]
    struct Rectangle {
        length: u32,
        width: u32,
    }

    impl Rectangle {
        // methods have to follow ownership rules too when referencing self
        fn area(&self) -> u32 {
            self.length * self.width
        }
        // method that has arguments other than self
        fn can_hold(&self, other: &Rectangle) -> bool {
            self.length > other.length && self.width > other.width
        }
        // associated method, called with :: on the struct instead of . on an instance
        fn square(size: u32) -> Rectangle {
            Rectangle { length: size, width: size }
        }
    }

    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle::square(25);

    // nicer debug print
    println!("rect1 is {:#?}", rect1);
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("The rectangle can hold {:?}: {}.", rect2, rect1.can_hold(&rect2));
}

fn defining_an_enum_6_1() {
    {
        // enums are like algebraic types
        #[derive(Debug)]
        enum IpAddrKind {
            V4, // variant of the enum
            V6,
        }
        // we can now use type signatures for args that will accept either type of IpAddrKind

        #[derive(Debug)]
        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }

        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
        println!("{:#?}", home);
        println!("{:#?}", loopback);
    }

    {
        #[derive(Debug)]
        enum IpAddr {
            V4(String), // attach a String to each enum variant
            V6(String),
        }

        let home = IpAddr::V4(String::from("127.0.0.1"));
        let loopback = IpAddr::V6(String::from("::1"));

        println!("{:#?}", home);
        println!("{:#?}", loopback);
    }

    {
        #[derive(Debug)]
        enum IpAddr {
            V4(u8, u8, u8, u8), // allows different variants to have different value types attached
            V6(String),
        }

        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));

        println!("{:#?}", home);
        println!("{:#?}", loopback);
    }
}
