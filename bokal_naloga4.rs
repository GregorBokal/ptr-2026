use std::io;

fn osnovno(niz: &str) -> u16 {
    match niz {
        "tri" => 3,
        "štiri" => 4,
        "pet" => 5,
        "šest" => 6,
        "sedem" => 7,
        "osem" => 8,
        "devet" => 9,
        _ => {panic!()}
    }
}

fn desetice(niz: &str) -> u16 {
    match niz {
        "dvajset" => 20,
        ndeset => {
            assert!(ndeset.ends_with("deset"));
            10 * osnovno(&ndeset[..ndeset.len() - 5])
        }
    }
}

fn sestavljeno(niz: &str) -> u16 {
    let mut stevilo: u16 = 0;
    let en_de: Vec<&str> = niz.split("in").collect();
    assert_eq!(en_de.len(), 2);
    match en_de[0] {
        "ena" => {stevilo += 1}
        "dva" => {stevilo += 2}
        nekaj => {stevilo += osnovno(nekaj)}
    }
    stevilo + desetice(en_de[1])
}

fn nestotice(niz: &str) -> u16 {
    if niz.contains("in") {
        sestavljeno(niz)
    } else if niz.ends_with("jst") {
        match niz {
            "enajst" => 11,
            "dvanajst" => 12,
            nnajst => 10 + osnovno(&nnajst[..niz.len()-5])
        }
    } else if niz.ends_with("set") {
        desetice(niz)
    } else {
        match niz {
            "en" => 1,
            "ena" => 1,
            "eno" => 1,
            "dva" => 2,
            "dve" => 2,
            nekaj => osnovno(&nekaj)
        }
    }
}

fn stotice(niz: &str) -> u16 {
    assert!(niz.ends_with("sto"));
    if niz.len() == 3 {
        return 100
    } else if &niz[..niz.len()-3] == "dve" {
        return 200
    }
    100 * osnovno(&niz[..niz.len() - 3])
}

fn pretvori (vnos: &str) -> u16 {
    let mut stevilo: u16 = 0;
    let sto_de: Vec<&str> = vnos.split_whitespace().collect();
    let d: usize = sto_de.len();
    if sto_de[0].ends_with("sto") {
        stevilo += stotice(&sto_de[0]);
        if d == 1 {
            return stevilo
        }
    }
    assert!(d < 3);
    if sto_de[0] != "nič" {
        stevilo += nestotice(&sto_de[d - 1]);
    }
    stevilo
}

fn main() {
    loop {
        let mut vnos: String = String::new();
        io::stdin().read_line(&mut vnos).expect("Ne znam brati!");
        println!("{}", pretvori(&vnos));
    }
}