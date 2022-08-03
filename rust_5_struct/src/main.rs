#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn perimeter(&self) -> u32 {
        return 2 * (self.width + self.height);
    }
}

fn main() {
    let ding = User {
        active: true,
        username: String::from("DuDing"),
        email: String::from("www@163.com"),
        sign_in_count: 64,
    };
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 50,
        height: 60,
    };

    println!("user.name is {}", ding.username);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("the perimeter is {} pixels", rect2.perimeter())
}
