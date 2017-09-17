pub fn structs_define_and_instantiate_5_1() {
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

pub fn using_structs_5_2() {
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

pub fn method_syntax_5_3() {
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
