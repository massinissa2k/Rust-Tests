extern crate rand;

use rand::Rng;
use std::{
    fs,
    io::{stdin, Write},
};

pub fn tests() {
    test_tab();
    println!();

    test_vect();
    println!();

    test_majeur(17);
    println!();

    print_retour();
    println!();

    test_some();
    println!();

    test_boucles();
    println!();

    test_file();
    println!();

    test_rand();
    println!();

    test_print_n_type();
    println!();

    test_stdin();
    println!();
}

fn test_tab() -> () {
    let tab = [8, 12, 20, 30, 50];
    let tab2 = &tab[2..];

    println!("test_tab {:?}", tab);
    println!("test_tab Sliced {:?}", tab2);
}

fn test_vect() -> () {
    let mut vect: Vec<u8> = Vec::new();
    vect.push(8);
    vect.push(80);

    let vect2 = &vect[2..];

    println!("test_vect {:?}", vect);
    println!("test_vect 2 {:?}", vect2);
}

fn test_majeur(age: u8) -> () {
    println!("test_majeur: ");
    match age {
        val if val < 18 => {
            println!("  Tu es mineur: {:?} ans", val);
        }
        18.. => {
            println!("  Tu as 18 ans ou plus");
        }
        _ => {
            println!("  Tu es trop jeune :)");
        }
    };
}

fn print_retour() -> () {
    let a = String::from(test_retour(""));
    println!("  print_retour: {}", a);
}

fn test_retour(_: &str) -> &str {
    "result test_retour"
}

fn test_some() {
    let mut v2 = vec![1, 2, 3];

    if let Some(p) = v2.pop() {
        println!("test_some {:?}", p);
    };
}

fn test_boucles() {
    println!("test_boucles: ");
    let vec: Vec<u8> = vec![47, 85, 21];

    for val in &vec {
        println!("  for res: {}", val);
    }

    for (index, val) in vec.iter().enumerate() {
        println!("  for res index: {}, value: {}", index, val);
    }

    let vec2 = 0..4;

    for val in vec2 {
        println!("  range res: {}", val);
    }
}

fn test_file() -> () {
    println!("test_file: ");
    let file_path = "./created-txt-file.txt";

    let file_result = fs::OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .append(true)
        .open(file_path);

    let mut file = match file_result {
        Ok(f) => {
            println!("File opened successfully");
            f
        }
        Err(e) => {
            println!("File does not opened successfully {}", e);
            return;
        }
    };

    file.write_all(b" Opened file 20 ")
        .expect("Can't write in opened file");
    file.write_all(b" Opened 2 file ")
        .expect("Can't write in opened file");
    file.sync_all().expect("Can't sync  in opened file");
    //fs::remove_file(file_path);
}

fn test_rand() -> () {
    println!("test_rang");
    let rng = rand::thread_rng().gen_range(0..=100);
    println!("  test_rang {}", rng);
}

fn test_stdin() -> () {
    let mut entree = String::new();

    println!("test_stdin");
    println!("  Saisir un mot et entrer!");

    let result = stdin().read_line(&mut entree);

    match result {
        Ok(_) => {
            println!("  {}", entree);
        }
        Err(e) => {
            println!("  Problème lors de la saisie ! {}", e);
        }
    }
}

fn test_print_n_type() {
    println!("test_print_n_type");
    let nombre: i32 = 6;
    let nombre2: i32 = 16;

    println!("  affiche le nombre en binaire: {:b}", nombre);
    println!("  affiche le nombre en octal (base 8): {:o}", nombre);
    println!(
        "  affiche le nombre en 'petit' hexadecimal (base 16): {:x}",
        nombre
    );
    println!(
        "  affiche le nombre en 'grand' hexadecimal (base 16): {:X}",
        nombre
    );
    println!("  affiche '00000006': {:08}", nombre);
    println!("  affiche '00000016': {:08}", nombre2);
}
