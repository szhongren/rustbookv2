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

}

pub fn hashmaps_8_3() {

}
