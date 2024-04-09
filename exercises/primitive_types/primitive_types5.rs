// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);

    let arr = [1, 2, 3, 4, 5, 6];
    let (num1, num2) = (arr[0], arr[1]);
    println!("{}, {}", num1, num2);
}
