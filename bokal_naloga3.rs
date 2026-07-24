use std::io;

fn preberi() -> Vec<u32> {
    let mut stanje: Vec<u32> = Vec::new();
    let mut vnos: String = String::new();
    io::stdin().read_line(&mut vnos).expect("Ne znam brati!");
    for usi in vnos.split_whitespace() {
        stanje.push(usi.parse().expect("Nekdo od naju ne zna šteti!"));
    }
    stanje
}

fn preseli(stanje: &mut Vec<u32>) {
    let mut za_dodati: u32 = 0;
    let konec = stanje.len() - 1;
    for i in 0..stanje.len() {
        let usi: u32 = stanje[i];
        stanje[i] += za_dodati;
        if 5 <= usi {
            za_dodati = (usi + 1) / 2;
            if i == konec {
                stanje[0] += za_dodati - 1;
            }
            stanje[i] -= za_dodati;
        } else {
            za_dodati = 0;
            if (1..=3).contains(&usi) {
                stanje[i] += 1;
            }
        }
    }
}

fn main() {
    let mut stanje: Vec<u32> = preberi();
    let mut ponovitve: String = String::new();
    io::stdin().read_line(&mut ponovitve).expect("Ne znam brati!");

    let n = ponovitve.trim().parse::<u32>().expect("Neko od naju ne zna šteti!");
    for _ in 0..n {
        preseli(&mut stanje);
        // println!("{:?}", stanje);
    }
    for (i, steblo) in stanje.iter().enumerate() {
        print!("{}", steblo);
        if i < stanje.len() - 1 {
            print!(" ");
        }
    }
}