// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let x: u8 = 20; // remember varible must be initialized
    let y: String = String::from("x is not ten!");
    if x == 10 {
        println!("x is ten!");
    } else {
        println!("{}", y);
    }
}
