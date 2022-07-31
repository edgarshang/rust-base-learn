#[derive(Debug)]
enum RoadMap{
    Go_learnteasy,
    Go_22,
    from0toteacher,
}

// enum Option<T>{
//     Some(T),
//     None
// }

// fn getDiscount(price:i32)->Option<bool>{
//     if price > 100{
//         Some(true)
//     }else{
//         None
//     }
// }
fn print_road_map(r:RoadMap){
    match r{
        RoadMap::Go_learnteasy=>{
            println!("first");
        },
        RoadMap::Go_22=>{
            println!("second");
        },
        RoadMap::from0toteacher=>{
            println!("third");
        }
    }
}

enum StudyRoadMap{
    Name(String),
}

fn main() {
    let level = RoadMap::Go_learnteasy;
    println!("level---{:?}", level);

    // let p = 666;
    // let result = getDiscount(p);
    // println!("{:?}", result);
    println!("Hello, world!");

    print_road_map(RoadMap::Go_learnteasy);

    let level3 = StudyRoadMap::Name(String::from("from 0 to teacher of language server"));
    match level3 {
        StudyRoadMap::Name(val)=>{
            println!("{:?}", val);d
        }
    }
}
