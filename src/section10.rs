pub fn generic_data_types_10_1() {
    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    use std::cmp::PartialOrd;

    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }

        fn y(&self) -> &T {
            &self.y
        }
    }

    let point0 = Point { x: 5, y: 10 };
    println!("point0.x = {}", point0.x());
    println!("point0.y = {}", point0.y());

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let point1 = Point { x: 2.8, y: 18.88 };
    println!("point1.x = {}", point1.x());
    println!("point1.y = {}", point1.y());
    println!("point1.distance_from_origin = {}",
             point1.distance_from_origin());

    struct MixedPoint<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> MixedPoint<T, U> {
        fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
            MixedPoint {
                x: self.x,
                y: other.y,
            }
        }
    }

    let point2 = MixedPoint { x: 5, y: 10.4 };
    let point3 = MixedPoint {
        x: "Hello",
        y: 'c',
    };

    let point4 = point2.mixup(point3);

    println!("point4.x = {}, point4.y = {}", point4.x, point4.y);

    println!("Generics in Rust are monomorphized at compile time.");
    println!("This means that versions of generic code are generated at comilation for each type \
              it is called with.");
}

pub fn traits_defining_shared_behavior_10_2() {
    println!("A trait is an abstraction over behavior that types can have in common.");
    println!("Similar to interfaces in other languages(like Go).");
    println!("A trait tells rustc about what functionality a type has.");
    println!("A trait bound can be used to specify at compile time that the generic type can be \
              any type that implements that trait.");
    println!("A trait is thus abstract over all types that implement it.");

    pub trait Summarizable {
        fn summary(&self) -> String {
            String::from("(Read more...)")
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summarizable for NewsArticle {
        fn summary(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summarizable for Tweet {
        fn summary(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    pub struct Comment {
        pub user: String,
        pub content: String,
    }

    impl Summarizable for Comment {}

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    let article = NewsArticle {
        headline: String::from("Lorem Ipsum"),
        location: String::from("Greece"),
        author: String::from("Dolores"),
        content: String::from("Lorem"),
    };

    let comment = Comment {
        user: String::from("hunter2"),
        content: String::from("this is the password"),
    };

    println!("1 new tweet: {}", tweet.summary());
    println!("1 new article: {}", article.summary());
    println!("1 new comment: {}", comment.summary());

    println!("You can implement your trait on an external type");
    println!("  or implement an external trait on an your type");
    println!("  but not an external trait on an external type");
    println!("  because of the orphan rule from type theory.");

    println!("When implementing default trait methods, we can call other methods from the same \
              trait.");
    println!("We can also conditionally implement methods with trait bounds.");

    use std::fmt::Display;

    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self { x: x, y: y }
        }
    }

    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest member is y = {}", self.y);
            }
        }
    }

    let p = Pair::new(1, 2);
    p.cmp_display();
}

pub fn validating_references_with_lifetimes_10_3() {
    println!("Lifetimes prevent dangling references");
    println!("Lifetimes are scope based");

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        //! all arguments passed in must have the same lifetime as the result
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    let string0 = String::from("abcd");
    let string1 = "xyz";

    let result = longest(string0.as_str(), string1);
    println!("The longest string is {}", result);

    println!("The result of a function must have the same lifetime as one of the arguments.");
    println!("Otherwise, the reference would be dropped at return.");

    println!("Structs can also hold references, but every reference must have a lifetime annotation");

    struct ImportantExcerpt<'a> {
        part: &'a str
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt{ part: first_sentence };
    println!("The excerpt is '{}'", i.part);

    println!("Lifetime elision is when the lifetime is deduced from three basic rules:");
    println!("1. Each parameter that is a reference gets its own lifetime parameter");
    println!("2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters");
    println!("3. If &self or &mut self is a parameter, then its lifetime is assigned to all output lifetime parameters");

    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    println!("Announcing: {}", i.announce_and_return_part("this"));

}
