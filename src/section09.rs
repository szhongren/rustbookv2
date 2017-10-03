pub fn panic_9_1() {
    println!("{}", "to panic, use panic like: panic!(\"crash and burn\"))");
    println!("in powershell use $Env:RUST_BACKTRACE = 1 to set rust backtrace");
    println!("then use $Env:RUST_BACKTRACE = '' to unset it");
}

pub fn result_9_2() {
    use std::fs::File;
    use std::io::ErrorKind;

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

    let f3 = File::open(".gitignore");
    let f3 = match f3 {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            // match guard, extra condition on a match arm that further refines the pattern
            // ref so that the arm does not take ownership of the error
            // & matches a reference but ref matches a value and takes a reference to it
            match File::create(".gitignore") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was an error creating the file: {:?}",
                error
            )
        },
    };
    println!("{:?}", f3);

    let f4 = File::open(".vscode/tasks.json").unwrap();
    // unwrap returns the value if Ok or panics if Err
    println!("{:?}", f4);

    let f5 = File::open("Cargo.toml").expect("Failed to open Cargo.toml");
    println!("{:?}", f5);
}

pub fn panic_or_not_9_3() {

}
