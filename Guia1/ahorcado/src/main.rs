use rand::Rng;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

const MAX_INTENTOS: usize = 5;
const PATHFILE: &str = "./docs/palabras.txt";
const EMPTY_STRING: String = String::new();

fn main() {
    println!("Bienvenido al ahorcado de FIUBA!");
    let palabras = leer_archivo(PATHFILE);
    let target = elegir_palabra(palabras);
    let mut historial: [String; MAX_INTENTOS] = [EMPTY_STRING; MAX_INTENTOS];
    let mut intentos_disponibles = MAX_INTENTOS;
    let mut palabra = inicializar_ahorcado(&target);
    let mut win = false;

    while intentos_disponibles != 0 && !win {
        imprimir_resultado_parcial(&palabra);
        imprimir_historial(&historial);
        println!(
            "Te quedan {} intentos.\nIngresa una letra: ",
            intentos_disponibles
        );

        let guess = match leer_nuevo_caracter(&historial) {
            Ok(n) => n,
            Err(_) => continue,
        };

        match reemplazar_coincidencias(&mut palabra, &guess, &target) {
            true => win = target.eq(&palabra),
            false => {
                historial[5 - intentos_disponibles] = guess;
                intentos_disponibles -= 1;
            }
        }
    }

    match win {
        true => println!("Correcto, es {}! Felicitaciones", target),
        false => println!("Se acabaron los intentos. La palabra era {}", target),
    }
}

fn inicializar_ahorcado(objetivo: &String) -> String {
    let mut palabra = String::new();
    for _c in 0..objetivo.len() {
        palabra.push('_');
    }
    palabra
}

fn reemplazar_coincidencias(resultado_parcial: &mut String, letra: &str, objetivo: &str) -> bool {
    let mut coincidencia = false;
    for (_i, _c) in objetivo.chars().enumerate() {
        match letra.trim().to_uppercase().chars().next() {
            Some(x) => {
                if x == _c {
                    resultado_parcial.replace_range(_i.._i + 1, &x.to_string());
                    coincidencia = true;
                }
            }
            _ => continue,
        }
    }
    coincidencia
}

fn leer_nuevo_caracter(historial: &[String; MAX_INTENTOS]) -> Result<String, ()> {
    let guess = leer_caracter()?;
    match historial.iter().position(|p| *p == guess) {
        None => Ok(guess),
        Some(_) => Err(()),
    }
}

fn leer_caracter() -> Result<String, ()> {
    let mut letra = String::new();
    match io::stdin().read_line(&mut letra) {
        Ok(n) => {
            if n != 2 {
                println!("Por favor, ingresa una letra");
                return Err(());
            }
            Ok(letra.trim().to_string().to_uppercase())
        }
        Err(_) => {
            println!("Error al leer la linea.");
            Err(())
        }
    }
}

fn imprimir_resultado_parcial(resultado: &str) {
    print!("\nLa palabra hasta el momento es: ");
    for _c in resultado.chars() {
        print!(" {}", _c);
    }
}

fn imprimir_historial(historial: &[String; MAX_INTENTOS]) {
    print!("\nLetras equivocadas: ");
    for _c in historial.iter() {
        print!("{} ", _c);
    }
    println!();
}

fn leer_archivo(path: &str) -> Vec<String> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut palabras = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        palabras.push(line);
    }
    palabras
}

fn elegir_palabra(palabras: Vec<String>) -> String {
    let indice = rand::thread_rng().gen_range(0..palabras.len());
    palabras[indice].to_uppercase()
}
