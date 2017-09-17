pub fn defining_an_enum_6_1() {
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

pub fn match_control_flow_6_2() {
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

pub fn concise_control_flow_if_let_6_3() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }
    // shorthand for the above
    if let Some(0) = some_u8_value {
        println!("zero");
    }

    #[derive(Debug)] // So we can inspect the state in a minute
    enum UsState {
        Alabama,
    }

    enum Coin {
        Quarter(UsState),
        #[allow(dead_code)]
        Penny,
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("count is now {}", count);
}
