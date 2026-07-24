use std::io;

fn main() {
    let mut vhod: String = String::new();
    io::stdin().read_line(&mut vhod).expect("Ne znam brati!");

    for stevke in vhod.split_whitespace() {
        let stevilo: u32 = stevke.parse::<u32>().expect("Nekdo od naju ne zna šteti!");
        if stevilo % 2 == 0 {
            print!("Tik");
        } else {
            print!("Tak");
        }
    }
}