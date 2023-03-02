use std::{error::Error, io::stdin};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Welkom bij de Temperatuur Wisselaar!");


    let mut temp = String::new();
    let mut eenheid = String::new();

    println!("Voer een tempertuur in Celsius in!");
    stdin().read_line(&mut temp)?;
    let temp: f64 = temp.trim().parse()?;

    println!("Voer een eenheid in: Typ 'f' voor Fahrenheit of 'k' Kelvin! in");
    stdin().read_line(&mut eenheid)?;
    let eenheid = eenheid.trim();

    let fahr: f64 = 32.0 +  (temp * 1.8 );
    let kel: f64 = temp - 273.15;

    match eenheid == "f" {
        true => println!("De temperatuur in Fahrenheit is: {}", fahr),
        _ => ()
    };

    match eenheid == "k" {
        true => println!("De temperatuur in Kelvin is: {}", kel),
        _ => ()
    };

    // Of

    if eenheid == "f"{
        let fahr: f64 = 32.0 + (temp * 1.8 );
        println!("De temperatuur in Fahrenheit is: {}", fahr);
    } else if eenheid == "k"{
        let kel: f64 = temp -273.15;
        println!("De temperatuur in Kelvin is: {}", kel);
    }else{
        println!("Er is geen eenheid ingevoerd!");
    }

    Ok(())

}
