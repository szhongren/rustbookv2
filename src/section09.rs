pub fn panic_9_1() {
    println!("{}", "to panic, use panic like: panic!(\"crash and burn\"))");
    println!("in powershell use $Env:RUST_BACKTRACE = 1 to set rust backtrace");
    println!("then use $Env:RUST_BACKTRACE = '' to unset it");
}

pub fn result_9_2() {
    use std::fs::File;
    let f = File::open("hello.txt");
    println!("{:?}", f);
    println!("type of f is a Result<Err>");

    let f1 = File::open("Cargo.lock");
    println!("{:?}", f1);
    println!("type of f1 is a Result<Ok<File>>");

    let f2 = File::open("Cargo.toml");
    match f2 {
        Ok(file) => println!("got a file: {:?}", file),
        Err(error) => println!("got an error: {:?}", error),
    };

}

pub fn panic_or_not_9_3() {

}
