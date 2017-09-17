pub fn variables_3_1() {
    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of x is now {}", x);
}

#[allow(unused_variables)]
pub fn data_types_3_2() {
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
pub fn functions_3_3() -> u32{
    let x = 8; // statement, performs an action and does not return a value
    let y = {
        let x = 3;
        x + 2
    }; // expression, returns a value
    y
}

pub fn control_flow_3_5() {
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
