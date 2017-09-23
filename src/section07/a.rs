pub mod series {
    pub mod of {
        pub fn nested_modules() {
            println!("I'm in a::series::of::nested_modules()");
            use super::super::Single::{Only}; // super imports go up one level in the module hierarchy
            let only = Only;
            println!("only is {:?}", only);

        }
    }
}

#[derive(Debug)]
pub enum Single {
    Only
}
