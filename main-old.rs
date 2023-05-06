// some comment!
// fn decrement_my_value() {
//     let mut x = 1000000;

//     while x > 0 {
//         x = x - 1;
//         println!("{x:?}");
//     }
// }

enum MyColors {
    Purple,
    Rose,
    Orange,
    Black,
}

fn match_color(color: MyColors) {
    match color {
        MyColors::Black => println!("Color is black!"),
        MyColors::Purple => println!("Color is Purple!"),
        MyColors::Rose => println!("Color is Rose!"),
        MyColors::Orange => println!("Color is Orange!"),
    }
}

fn main() {
    // decrement_my_value();
    match_color(MyColors::Purple)
}

// rust tutorial, 2 hrs done!

// 23 April 2023
// - match
// - loop
// - enum
