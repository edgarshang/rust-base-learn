

// fn hello_world() {
//     println!("Hello, hello_world!");
// }

// fn get_name() -> String {
//     return String::from("python Qt cv 开发");
// }

// fn get_name_2() -> String {
//     return String::from("python Qt tools 开发");
// }

// fn double_price(mut price:i32) {
//     price = price *2;
//     println!("内部的price是{}",price);
// }

// fn double_price2(price :&mut i32) {
//     *price = *price *3;
//     println!("内部的price{}", price);
// }

fn show_name(name:String) {
    println!("学习科目为:{}", name);
}

fn main() {
    // println!("Hello, world!");
    // hello_world();
    // println!("r1:{}",get_name());
    // println!("r2:{}",get_name_2());
    // let mut price = 32;
    // double_price(price);
    // double_price2(&mut price);
    // println!("外部的price是{}",price);

    let name = String::from("hello, world");
    show_name(name);
    // println!("name = {}",name);
}