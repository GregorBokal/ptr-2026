use std::io;

fn main() {
    let mut vhod: String = String::new();
    let mut samoglasniki: i32 = 0;
    io::stdin().read_line(&mut vhod).expect("Ne znam brati!");
    for znak in vhod.chars() {
        if "AaEeIiOoUu".contains(znak) {
            samoglasniki += 1;
        }
    }
    println!("{samoglasniki}")
}