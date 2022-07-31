
#[derive(Debug)]
struct Study{
    name: String,
    target:String,
    spend:i32,
}

fn show(s:Study)
{
    println!("name is {} target is {} spend is {}", s.name, s.target, s.spend);
}

fn main() {
    let mut s = Study{
        name:String::from("from 0 to teacher of mirco of Go language"),
        target:String::from("learn the system of server and fenbushi"),
        spend:3,
    };
    s.spend = 8;
    println!("{:?}", s);
    println!("{:?}", s.name);
    println!("{:?}", s.spend);
    println!("Hello, world!");
    show(s)
}
