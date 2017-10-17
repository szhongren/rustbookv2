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
    println!("point1.distance_from_origin = {}", point1.distance_from_origin());

    struct MixedPoint<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> MixedPoint<T, U> {
        fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
            MixedPoint{
                x: self.x,
                y: other.y,
            }
        }
    }

    let point2 = MixedPoint { x: 5, y: 10.4 };
    let point3 = MixedPoint { x: "Hello", y: 'c'};

    let point4 = point2.mixup(point3);

    println!("point4.x = {}, point4.y = {}", point4.x, point4.y);

    println!("Generics in rust are monomorphized at compile time.");
    println!("This means that versions of generic code are generated at comilation for each type it is called with.");
}

pub fn traits_defining_shared_behavior_10_2() {

}
