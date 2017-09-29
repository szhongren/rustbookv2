pub fn vectors_8_1() {
    let mut v: Vec<i32> = Vec::new();
    println!("v is {:?}", v);

    let v1 = vec![1, 2, 3, 4];
    println!("v1 is {:?}", v1);

    v.push(0);
    v.push(9);
    v.push(8);
    v.push(7);
    println!("v is now {:?}", v);

    let third: &i32 = &v[2]; // using references panics when the index is out of bounds
    let third_option: Option<&i32> = v.get(2); // Option always returns
    println!("third is {:?}", third);
    println!("third_option is {:?}", third_option);

    println!("vectors can only store items of the same type");
    println!("enums count as the same type no matter what their contents");
}

pub fn strings_8_2() {
    println!("there are 2 kinds of strings in Rust");
    println!("1. String - which is a Unicode string that is implemented as a vector of bytes representing Unicode code points");
    println!("2. str - which is a string, usually sliced and a reference to some UTF-8 encoded string data stored elsewhere");

    let s = String::from("this is a string made from String::from()");
    let s1 = "this is a string made from to_string()".to_string();

    println!("first string is: {}", s);
    println!("second string is: {}", s1);

    let mut s2 = String::from("I can push");
    println!("third string is: {}", s2);
    s2.push_str(" something onto the end");
    println!("third string is now: {}", s2);
    println!("push_str() takes a string slice because it does not want to take ownership");

    let s3 = String::from("you can concat ");
    let s4 = String::from("strings together ");
    let s5 = String::from("with +");
    println!("{}", s3 + &s4 + &s5);

    let s6 = String::from("for");
    let s7 = String::from("more");
    let s8 = String::from("complicated");
    let s9 = String::from("strings");
    println!("{}", format!("{} {} {} {} use format!", s6, s7, s8, s9));

    println!("indexing does not work on strings because internally they are represented as a Vec<u8>");
    println!("length of a String will thus be the number of bytes it takes to store it in Unicode");
    println!("raw bytes make up scalar unicode values make up graheme clusters");
    println!("slicing Strings does work but only on character boundaries");

    println!("for bytes, use .bytes()");
    println!("for scalar values, use .chars()");
}

pub fn hashmaps_8_3() {

}
