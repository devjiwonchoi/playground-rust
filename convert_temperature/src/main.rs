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
        process::exit(1)
    }

    let mut input_temperature = String::new();
    let mut temperature: f64 = 0.0;
    let mut converted_temperature: f64 = 0.0;
    let mut convert_target = "Undefined";

    if target == "f" {
        println!("Enter the temperature to be converted to Farenheit.");
        stdin()
            .read_line(&mut input_temperature)
            .expect("Something went wrong. Please try again.");

        let celcius: f64 = input_temperature.trim().parse::<f64>().unwrap();

        temperature = celcius;
        converted_temperature = convert_celcius_to_farenheit(celcius);
        target = "Celcius";
        convert_target = "Farenheit";
    }

    if target == "c" {
        println!("Enter the temperature to be converted to Celcius.");
        stdin()
            .read_line(&mut input_temperature)
            .expect("Something went wrong. Please try again.");

        let farenheit: f64 = input_temperature.trim().parse::<f64>().unwrap();

        temperature = farenheit;
        converted_temperature = convert_farenheit_to_celcius(farenheit);
        target = "Farenheit";
        convert_target = "Celcius";
    }

    let convert_target = convert_target.to_string();

    println!("Converted {target} {temperature} to {convert_target} {converted_temperature}");
}

fn convert_celcius_to_farenheit(celcius: f64) -> f64 {
    (celcius * 9.0/5.0) + 32.0
}

fn convert_farenheit_to_celcius(farenheit: f64) -> f64 {
    (farenheit - 32.0) / 9.0 / 5.0
}
