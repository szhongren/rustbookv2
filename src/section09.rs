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
    // expect takes a string that is used as the panic message
    println!("{:?}", f5);

    use std::io;
    use std::io::Read;

    fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
        // propagates the error to the caller
        let f = File::open(filename);

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }

    let f6 = read_username_from_file("Cargo.lock");
    println!("{:?}", f6);

    fn read_username_from_file_1(filename: &str) -> Result<String, io::Error> {
        let mut s = String::new();
        File::open(filename)?.read_to_string(&mut s)?; // the ? is syntactical sugar for the match expressions above
        // ? syntax only works in functions that return a Result
        // if Ok(), returns the Ok to the statement
        // if Err(), returns the Err to the whole function's caller
        Ok(s)
    }

    let f7 = read_username_from_file_1("test.txt");
    println!("{:?}", f7);
}

pub fn panic_or_not_9_3() {
    println!("When should you panic or not?");
    println!("When in tests or prototyping, panicking is perfectly fine.");
    println!("When in a test, panicking is how the test is marked as a failure.");
    println!("Sometimes, when you have more information than the compiler, it is fine to use unwrap.");
    println!("Guidelines for when to panic:");
    println!("When you have bad state and one of the following:");
    println!("- The bad state is not something that’s expected to happen occasionally");
    println!("- Your code after this point needs to rely on not being in this bad state");
    println!("- There’s not a good way to encode this information in the types you use");

    println!("Generally speaking, we want to use types to enforce restrictions on values for a certain variable in our code, so that the function tha tuses it can have some guarantees about the value.");
}
