mod client;
mod network;
mod a; // include

pub fn mod_and_the_filesystem_7_1() {
    client::connect();
    network::connect();
    network::server::connect();
}


pub fn importing_names_with_use_7_3() {
    use self::a::series::of::nested_modules; // namespacing
    nested_modules();
}
