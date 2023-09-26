use std::io::stdin;
use std::process;

fn main() {
    println!("To which temperature do you want to convert to?");
    println!("Enter `c` for Celcius, `f` for Farenheit.");

    let mut target = String::new();
    stdin()
        .read_line(&mut target)
        .expect("Something went wrong. Please try again.");
    // Type is str
    let mut target = target.trim();

    // Leave this since it was a good research
    if !vec!["f", "c"].contains(&target) {
        println!("You typed `{target}`.");
        println!("Please enter a valid temperature input, `c` or `f`.");
        process::exit(1);
    }

    let mut input_temperature = String::new();

    let source = if target == "f" {
        "Celcius"
    } else if target == "c" {
        "Farenheit"
    } else {
        "Undefined"
    };
    target = if target == "f" {
        "Farenheit"
    } else if target == "c" {
        "Celcius"
    } else {
        "Undefined"
    };

    if target == "Undefined" || source == "Undefined" {
        println!("Something went wrong. Please try again.");
        process::exit(1);
    }

    println!("Enter the temperature to be converted to {target}.");
    stdin()
        .read_line(&mut input_temperature)
        .expect("Something went wrong. Please try again.");

    let input_temperature = input_temperature.trim().parse::<f64>().unwrap();

    let converted_temperature: f64 = if target == "Farenheit" {
        convert_celcius_to_farenheit(input_temperature)
    } else {
        convert_farenheit_to_celcius(input_temperature)
    };

    println!("Converted {target} {input_temperature} to {source} {converted_temperature}");
}

fn convert_celcius_to_farenheit(celcius: f64) -> f64 {
    (celcius * 9.0 / 5.0) + 32.0
}

fn convert_farenheit_to_celcius(farenheit: f64) -> f64 {
    (farenheit - 32.0) / 9.0 / 5.0
}
