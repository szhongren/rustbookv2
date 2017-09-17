mod section01;
mod section02;
mod section03;

fn main() {
    section01::hello_1_2();
    div();
    // section02::guessing_game_2();
    section03::variables_3_1();
    div();
    section03::data_types_3_2();
    div();
    section03::functions_3_3();
    div();
    section03::control_flow_3_5();
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
    div();
    match_control_flow_6_2();
}

fn div() {
    println!("--------------------------------------------------------------------------------");
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

    #[allow(dead_code)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
}

fn match_control_flow_6_2() {
    {
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }

        fn value_in_cents(coin: Coin) -> u32 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }

        println!("The value of a penny is {}", value_in_cents(Coin::Penny));
        println!("The value of a nickel is {}", value_in_cents(Coin::Nickel));
        println!("The value of a dime is {}", value_in_cents(Coin::Dime));
        println!("The value of a quarter is {}", value_in_cents(Coin::Quarter));
    }

    {
        // binding in match arms
        #[derive(Debug)] // So we can inspect the state in a minute
        enum UsState {
            Alabama,
            Alaska,
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }

        fn value_in_cents(coin: Coin) -> u32 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                },
            }
        }

        println!("The value of a penny is {}", value_in_cents(Coin::Penny));
        println!("The value of a nickel is {}", value_in_cents(Coin::Nickel));
        println!("The value of a dime is {}", value_in_cents(Coin::Dime));
        println!("The value of a quarter is {}", value_in_cents(Coin::Quarter(UsState::Alaska)));
        println!("The value of a quarter is {}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    }

    // using options in functions that can chain and implement error handling
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);
}
