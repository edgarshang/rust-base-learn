fn test_vector(){
    let mut v = Vec::new();
    v.push("the book of Go language");
    v.push("22 lessons of the go language server");
    v.push ("to be a teacher of the go language server");
    println!("{:?}", v);
    println!("{}", v.len());
    println!("Hello, world!");

    let mut v2 = vec!["the book of Go language", "22 lessons of the go language","to be a teacher of the go language server"];
    println!("{:?}", v2);

    let x = v2.remove(0);
    println!("{:?}", x);
    println!("{:?}",v2);

    if v2.contains(&"the book of Go language") {
        println!("yes, it's here");
    }else
    {
        println!("no")
    }

    let y = v[0];
    println!("{:?}",y);

    for item in v{
        println!("the prgorm is {:?}",item);
    }
}

use std::collections::HashMap;

fn test_hash_map(){
    let mut process = HashMap::new();
    process.insert("the book of Go language", 100);
    process.insert("22 lessons of the go language",2);
    process.insert("to be a teacher of the go language server",3);

    println!("{:?}",process);
    println!("{:?}",process.len());

    match process.get(&"to be a teacher of the go language server"){
        Some(v)=>{
            println!("HashMap   {:?}",v);
        }
        None=>{
            println!("HashMap None");
        }
    }

    for (k,v) in process.iter(){ 
        println!("k:{:?} v :{:?}",k, v);
    }

    if process.contains_key(&"22 lessons of the go language")
    {
        println!("yes, it's here");
    }

    let x = process.remove(&"22 lessons of the go language");
    println!("{:?}",x);
    println!("{:?}",process);

}

use std::collections::HashSet;
fn hash_set_test() {
    let mut studySet = HashSet::new();
    studySet.insert("the book of Go language");
    studySet.insert("22 lessons of the go language");
    studySet.insert("to be a teacher of the go language server");
    println!("{:?}",studySet);

    studySet.insert("to be a teacher of the go language server");
    println!("{:?}",studySet);

    println!("len:{:?}", studySet.len());

    for item in studySet.iter() {
        println!("hash set {:?}", item);
    }

    // match studySet.get(&"to be a teacher of the go language server") {
    //     None => {
    //         println!("not be found");
    //     }

    //     Some(data) => {
    //         println!("study Set find:{:?}", data);
    //     }
    // }
}

fn main() {
    let switch = 3;
    if switch == 1 {
        test_vector();
    }else if switch == 2 {
        test_hash_map();
    }else if switch == 3 {
        hash_set_test();
    }else{
        println!("hello, world!");
    }
}

