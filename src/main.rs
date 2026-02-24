use std::io::{Write, stdin, stdout};

fn read_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    stdout().flush().unwrap();
    stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn read_f64(prompt: &str) -> Option<f64> {
    match read_input(prompt).parse() {
        Ok(n) => Some(n),
        Err(_) => {
            println!("Introdu un numar valid!");
            None
        }
    }
}

fn read_u32(prompt: &str) -> Option<u32> {
    match read_input(prompt).parse() {
        Ok(n) => Some(n),
        Err(_) => {
            println!("Introdu un numar valid!");
            None
        }
    }
}

fn menu_triunghi() {
    loop {
        println!("\n--- Triunghi dreptunghic - laturi ---");
        println!("0. Back");
        println!("1. Ipotenuza");
        println!("2. Cateta");
        let Some(opt) = read_u32("Alege: ") else {
            continue;
        };

        match opt {
            0 => break,
            1 => {
                let Some(c1) = read_f64("Cateta 1: ") else {
                    continue;
                };
                let Some(c2) = read_f64("Cateta 2: ") else {
                    continue;
                };
                let result = (c1.powi(2) + c2.powi(2)).sqrt();
                println!("Ipotenuza = {}", result);
            }
            2 => {
                let Some(ip) = read_f64("Ipotenuza: ") else {
                    continue;
                };
                let Some(c) = read_f64("Cateta: ") else {
                    continue;
                };
                if c >= ip {
                    println!("Cateta trebuie sa fie mai mica decat ipotenuza!");
                    continue;
                }
                let result = (ip.powi(2) - c.powi(2)).sqrt();
                println!("Cateta = {}", result);
            }
            _ => println!("Input invalid!"),
        }
    }
}

fn menu_unghiuri() {
    loop {
        println!("\n--- Functii trigonometrice ---");
        println!("0. Back");
        println!("1. sin");
        println!("2. cos");
        println!("3. tan");
        println!("4. cot");
        println!("5. sec");
        println!("6. csc");
        let Some(opt) = read_u32("Alege: ") else {
            continue;
        };

        if opt == 0 {
            break;
        }

        let name = match opt {
            1 => "sin",
            2 => "cos",
            3 => "tan",
            4 => "cot",
            5 => "sec",
            6 => "csc",
            _ => {
                println!("Input invalid!");
                continue;
            }
        };

        println!("\n--- {} ---", name);
        println!("1. Din unghi (grade) -> valoare");
        println!("2. Din laturi -> unghi + valoare");
        let Some(sub) = read_u32("Alege: ") else {
            continue;
        };

        match sub {
            1 => {
                let Some(unghi) = read_f64("Unghi (grade): ") else {
                    continue;
                };
                let rad = unghi.to_radians();
                let val = match opt {
                    1 => rad.sin(),
                    2 => rad.cos(),
                    3 => rad.tan(),
                    4 => 1.0 / rad.tan(),
                    5 => 1.0 / rad.cos(),
                    6 => 1.0 / rad.sin(),
                    _ => unreachable!(),
                };
                println!("{}({}) = {}", name, unghi, val);
            }
            2 => {
                let (l1_name, l2_name) = match opt {
                    1 => ("Latura opusa", "Ipotenuza"),
                    2 => ("Latura adiacenta", "Ipotenuza"),
                    3 => ("Latura opusa", "Latura adiacenta"),
                    4 => ("Latura adiacenta", "Latura opusa"),
                    5 => ("Ipotenuza", "Latura adiacenta"),
                    6 => ("Ipotenuza", "Latura opusa"),
                    _ => unreachable!(),
                };
                let Some(l1) = read_f64(&format!("{}: ", l1_name)) else {
                    continue;
                };
                let Some(l2) = read_f64(&format!("{}: ", l2_name)) else {
                    continue;
                };

                if l2 == 0.0 {
                    println!("Numitorul nu poate fi 0!");
                    continue;
                }

                let ratio = l1 / l2;
                let val = match opt {
                    1 => ratio, // sin = opus / ipotenuza
                    2 => ratio, // cos = adiacent / ipotenuza
                    3 => ratio, // tan = opus / adiacent
                    4 => ratio, // cot = adiacent / opus
                    5 => ratio, // sec = ipotenuza / adiacent
                    6 => ratio, // csc = ipotenuza / opus
                    _ => unreachable!(),
                };

                // calcul unghi din raport
                let unghi_rad = match opt {
                    1 => (l1 / l2).asin(),
                    2 => (l1 / l2).acos(),
                    3 => (l1 / l2).atan(),
                    4 => (l2 / l1).atan(), // cot = 1/tan -> unghi = atan(opus/adiacent)
                    5 => (l2 / l1).acos(), // sec = 1/cos -> unghi = acos(adiacent/ipotenuza)
                    6 => (l2 / l1).asin(), // csc = 1/sin -> unghi = asin(opus/ipotenuza)
                    _ => unreachable!(),
                };
                let unghi = unghi_rad.to_degrees();

                println!("{}  = {} / {} = {}", name, l1_name, l2_name, val);
                println!("Unghiul = {:.4} grade", unghi);

                // afiseaza si celelalte functii trig pentru acelasi unghi
                println!("\n-- Toate functiile pentru unghi = {:.4} grade --", unghi);
                println!("  sin = {}", unghi_rad.sin());
                println!("  cos = {}", unghi_rad.cos());
                println!("  tan = {}", unghi_rad.tan());
                println!("  cot = {}", 1.0 / unghi_rad.tan());
                println!("  sec = {}", 1.0 / unghi_rad.cos());
                println!("  csc = {}", 1.0 / unghi_rad.sin());
            }
            _ => println!("Input invalid!"),
        }
    }
}

fn main() {
    loop {
        println!("\n=== Calculator Triunghi Dreptunghic ===");
        println!("0. Exit");
        println!("1. Triunghi dreptunghic - laturi");
        println!("2. Unghiuri");
        let Some(opt) = read_u32("Alege: ") else {
            continue;
        };

        match opt {
            0 => break,
            1 => menu_triunghi(),
            2 => menu_unghiuri(),
            _ => println!("Input invalid!"),
        }
    }
}
