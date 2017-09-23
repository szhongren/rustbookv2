mod section01;
mod section02;
mod section03;
mod section04;
mod section05;
mod section06;
mod section07;
mod section08;

fn main() {
    section01::hello_1_2();
    div();
    // section02::guessing_game_2();
    section03::variables_3_1();
    div();
    section03::data_types_3_2();
    div();
    section03::functions_3_3();
    div();
    section03::control_flow_3_5();
    div();
    section04::ownership_4_1();
    div();
    section04::references_and_borrowing_4_2();
    div();
    section04::slices_4_3();
    div();
    section05::structs_define_and_instantiate_5_1();
    div();
    section05::using_structs_5_2();
    div();
    section05::method_syntax_5_3();
    div();
    section06::defining_an_enum_6_1();
    div();
    section06::match_control_flow_6_2();
    div();
    section06::concise_control_flow_if_let_6_3();
    div();
    section07::mod_and_the_filesystem_7_1();
    div();
    section07::importing_names_with_use_7_3();
    div();
    section08::vectors_8_1();
    div();
    section08::strings_8_2();
    div();
    section08::hashmaps_8_3();
}

fn div() {
    println!("--------------------------------------------------------------------------------");
}
