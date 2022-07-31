fn main() {
    let mut v = Vec::new();
    v.push("Go 语言极简一本通");
    v.push("GO language from miro 22");
    v.push("from 0 to teacher");
    println!("len:{:?}", v.len());
    let s1 = &v[0..2];
    println!("s1:{:?}", s1);
    shwo_slices(s1);

    modify_slice(&mut v[1..3]);
    println!("the after modify_slice v={:?}",v);
}

fn shwo_slices(s:&[&str]){
    println!("shwo_slices function:{:?}",s);
}

fn modify_slice(s:&mut [&str]){
    s[0] = "the function you have learn";
    println!("modify_slice:{:?}",s);
}
