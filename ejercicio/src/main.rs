use std::fs::File;
use std::io::{self, BufRead};
use std::io::Read;
use std::path::Path;
use std::fs::OpenOptions;
use std::f32;
use std::io::Write;

fn read_file(mut f: &File) {
    let mut text = String::new();
    f.read_to_string(&mut text).unwrap();
    println!("{}", &text);
}

fn create_blank_file(p: &Path) {
    let _file = File::create(p).expect("El archivo no pudo crearse");
    println!("El archivo fue creado");
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn open_file_to_append(p: &Path) -> File{
    let mut binding = OpenOptions::new();
    let binding = binding.append(true);
    let file = match binding.open(p){
        Err(_why) => panic!("No se puede abrir el archivo"),
        Ok(file) => file,
    };

    return file
}

fn open_file(p: &Path) {
    if Path::new(p).exists(){
        let _file = match File::open(&p){
            Err(_why) => panic!("El archivo no se puede abrir..."),
            Ok(file) => file,
        };
    } else {
        create_blank_file(p);
        panic!("reinicie, porfavor")
    }
}

fn main() {
    let path = Path::new("reporte.txt");
    open_file(path);
    let mut file = open_file_to_append(path);

    if let Ok(lines) = read_lines("./notas.txt") {
        let mut nombres:u32 = 0;
        for line in lines {
            if let Ok(ip) = line {
                let split = ip.split(":");
                let mut suma:f32 = 0.0;
                let mut contador:u32 = 0;

                for s in split {
                    if contador == 0 && nombres == 0 {
                        println!("{}:", s);
                        file.write_all(b"Pepito ");
                    } else if contador == 0 && nombres == 1 {
                        println!("{}:", s);
                        file.write_all(b"Juanito ");
                    } else if contador == 0 && nombres == 2 {
                        println!("{}:", s);
                        file.write_all(b"Maria ");
                    } else if  contador == 0 && nombres == 3 {
                        println!("{}:", s);
                        file.write_all(b"Joselito ");
                    }else {
                        let num: f32 = match s.parse::<f32>() {
                            Err(_) => panic!("No es un numero compatible"),
                            Ok(s) => s
                        };

                        suma += num;
                    }

                    let promedio:f32 = suma/6.0;

                    if promedio < 4.0 && contador == 6{
                        println!("Reprobo, con un promedio {}", promedio);
                        file.write_all(b"reprobo\n");
                    } else if promedio >= 4.0 && contador == 6 {
                        println!("Aprobo, con un promedio {}", promedio);
                        file.write_all(b"aprobo\n");
                    }

                    contador += 1;
                }
            }
            println!("");
            nombres += 1;
        }
    }
    
    file.write_all(b"\n");
    println!("");
    println!("");
    println!("");
    read_file(&file)
}