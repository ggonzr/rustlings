// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.

fn main() {
    // Rust automatically doesn't recast
    // For instance, the next type doesn't
    // allow the comparison
    // let x: u8 = 10;

    let x: i32 = 512;
    if x == 512 {
        println!("x has a 512 value!");
    } else {
        println!("x has not a 512 value!");
    }
}
