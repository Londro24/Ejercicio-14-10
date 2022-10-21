use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::f32;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines("./notas.txt") {
        for line in lines {
            if let Ok(ip) = line {
                let split = ip.split(":");
                let mut suma:f32 = 0.0;
                let mut contador:u32 = 0;

                for s in split {
                    if contador == 0 {
                        println!("{}:", s)
                    } else {
                        let num: f32 = match s.parse::<f32>() {
                            Err(_) => panic!("No es un numero compatible"),
                            Ok(s) => s
                        };

                        suma += num;
                    }

                    let promedio:f32 = suma/6.0;

                    if promedio < 4.0 && contador == 6{
                        println!("Reprobo, con un promedio {}", promedio);
                    } else if promedio >= 4.0 && contador == 6 {
                        println!("Aprobo, con un promedio {}", promedio);
                    }

                    contador += 1;
                }
            }
            println!("");
        }
    }
}