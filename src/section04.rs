pub fn ownership_4_1() {
    println!("1. Each value in Rust has a variable that's called its owner.");
    println!("2. There can only be one owner at a time.");
    println!("3. When the owner goes out of scope, the value will be dropped.");
    println!("Generally, stack allocated variables are deep copied (implement Copy trait) and heap allocated variables are moved(implement Drop trait) as opposed to shallow copy.");
    println!(".clone is used for a deep copy.");
}

pub fn references_and_borrowing_4_2() {
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

pub fn slices_4_3() {
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
