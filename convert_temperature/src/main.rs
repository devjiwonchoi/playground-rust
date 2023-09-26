use std::io::stdin;

fn main() {
    println!("To which temperature do you want to convert to?");
    println!("Enter `c` for Celcius, `f` for Farenheit.");

    let mut target = String::new();
    stdin()
        .read_line(&mut target)
        .expect("Something went wrong. Please try again.");
    target = target.trim().to_owned();

    // Leave this since it was a good research
    if !vec!["f", "c"].contains(&target.as_str()) {
        println!("You typed {target}.");
        println!("Please enter a valid temperature input, `c` or `f`.")
    }

    let mut temperature: f64 = 0.0;
    let mut converted_temperature: f64 = 0.0;
    let mut convert_target = "Undefined".to_string();

    if target == "f".to_string() {
        println!("Enter the temperature to be converted to Farenheit.");
        let mut celcius = String::new();
        stdin()
            .read_line(&mut celcius)
            .expect("Something went wrong. Please try again.");

        let celcius: f64 = celcius.trim().parse::<f64>().unwrap();

        temperature = celcius;
        converted_temperature = convert_celcius_to_farenheit(celcius);
        target = "Celcius".to_string();
        convert_target = "Farenheit".to_string();
    }

    if target == "c".to_string() {
        println!("Enter the temperature to be converted to Celcius.");
        let mut farenheit = String::new();
        stdin()
            .read_line(&mut farenheit)
            .expect("Something went wrong. Please try again.");

        let farenheit: f64 = farenheit.trim().parse::<f64>().unwrap();

        temperature = farenheit;
        converted_temperature = convert_farenheit_to_celcius(farenheit);
        target = "Farenheit".to_string();
        convert_target = "Celcius".to_string();
    }

    println!("Converted {target} {temperature} to {convert_target} {converted_temperature}");
}

fn convert_celcius_to_farenheit(celcius: f64) -> f64 {
    (celcius * 9.0/5.0) + 32.0
}

fn convert_farenheit_to_celcius(farenheit: f64) -> f64 {
    (farenheit - 32.0) / 9.0 / 5.0
}
