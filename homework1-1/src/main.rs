const freezing_point_f: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - freezing_point_f) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + freezing_point_f
}

fn main() {
    let mut temperature_f: f64 = 32.0;

    let temperature_c = fahrenheit_to_celsius(temperature_f);
    println!("{} F = {} C", temperature_f, temperature_c);

    for _i in 1..=5 {
        temperature_f += 1.0;
        let temp_c = fahrenheit_to_celsius(temperature_f);
        println!("{} F = {:.2} C", temperature_f, temp_c);
    }

}