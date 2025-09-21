const FREEZING_POINTF: f64 =32.0;

fn fahrenheit_to_celsius(f: f64) -> f64{
    (f-FREEZING_POINTF)*5.0/9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64{
    (c*9.0/5.0)+FREEZING_POINTF
}

fn main() {
    let mut temp_f:f64 =32.0;

    println!("{}°F = {}°C", temp_f, fahrenheit_to_celsius(temp_f));

    for _ in 0..5{
        temp_f+=1.0;
        println!("{}°F = {}°C", temp_f, fahrenheit_to_celsius(temp_f));

    }

    let mut temp_c:f64 =0.0;

    println!("{}°C = {}°F", temp_c, celsius_to_fahrenheit(temp_c));

    for _ in 0..5{
        temp_c+=1.0;
        println!("{}°C = {}°F", temp_c, celsius_to_fahrenheit(temp_c));

    }

}