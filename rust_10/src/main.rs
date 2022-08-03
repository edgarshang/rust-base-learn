// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest_i32(&number_list);
//     println!("The largest number is {}", result);

//     let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

//     let result = largest_i32(&number_list);
//     println!("The largest number is {}", result);

//     let workPoint = point { x: 5, y: "hell" };
// }

// // fn largest<T>(list: &[T]) -> T {
// //     let mut largest_value = list[0];

// //     for &item in list {
// //         if item > largest_value {
// //             largest_value = item;
// //         }
// //     }

// //     return largest_value;
// // }

// struct point<T, Q> {
//     x: T,
//     y: Q,
// }

// fn largest_i32(list: &[i32]) -> i32 {
//     let mut largest_value = list[0];

//     for &item in list {
//         if item > largest_value {
//             largest_value = item;
//         }
//     }

//     return largest_value;
// }

// fn largest_char(list: &[char]) -> char {
//     let mut largest_value = list[0];

//     for &item in list {
//         if item > largest_value {
//             largest_value = item;
//         }
//     }

//     return largest_value;
// }

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
