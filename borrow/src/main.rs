fn show(v:&Vec<&str>){
    println!("v:{:?}",v)
}

fn show2(v:&mut Vec<&str>){
    v[0]="第一个充电项目已完成";
    println!("v:{:?}",v);
}

fn main() {
    let studyList = vec!["C","C++", "python"];
    let studyList2 = studyList;
    let mut studyList3 = vec!["C","C++", "python"];
    show(&studyList2);
    show2(&mut studyList3);
    println!("studyList2:{:?}",studyList2);
    println!("studyList3:{:?}",studyList3);
}
