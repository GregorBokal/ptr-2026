use std::io;

/*
Rešitev temelji na ideji, da je zaradi merjenja dolžine z manhatansko razdaljo
popolnoma vseeno, kako najkrajša ograda "vijuga" — pomembno je le, da se razteza
čez vso širino in dolžino območja z bakterijami.

Če torej vse bakterije le ujamemo v najmanjši možni pravokotnik, bo njegov obseg
predstavljal najkrajšo možno dolžino ograde (ki bi jo v praksi lahko še uvihali
navznoter, da bi bila čim manjša tudi ploščina ... ampak naloga tega ne zahteva).
*/

fn main() {
    let mut vnos: String = String::new();
    io::stdin().read_line(&mut vnos).expect("Ne znam brati!");
    let mut y_x = vnos.trim().split_whitespace();

    let mut min_y: u32 = y_x.next().unwrap().parse().expect("Nekdo od naju ne zna šteti!");
    let mut max_y: u32 = 0;
    let mut min_x: u32 = y_x.next().unwrap().parse().expect("Nekdo od naju ne zna šteti!");
    let mut max_x: u32 = 0;

    for v in 0..min_y.clone() {
        vnos.clear();
        io::stdin().read_line(&mut vnos).expect("Ne znam brati!");

        if let (Some(p), Some(z)) = (vnos.find('#'), vnos.rfind('#')) {
            let (p, z) = (p as u32, z as u32);

            if v < min_y {min_y = v} else if v > max_y {max_y = v}
            if p < min_x {min_x = p} else if z > max_x {max_x = z}
        }
    }
    println!("{}", 2 * (1 + max_y - min_y) + 2 * (1 + max_x - min_x))
}