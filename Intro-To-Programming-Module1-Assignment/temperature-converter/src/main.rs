
fn fahrenheit_to_celsius(f:f64) -> f64 {
    return (f - 32.0) * (5.0 / 9.0);
}

fn celsius_to_fahrenheit(c:f64) -> f64 {
    return (c * (9.0 / 5.0)) + 32.0;
}


fn main(){
    let mut temp_farenheit:f64 = 50.0;
    let temp_celsius:f64 = fahrenheit_to_celsius(temp_farenheit);
    println!("{:.2} degrees Farenheit is {:.2} degrees Celsius", temp_farenheit, temp_celsius);

    let mut i = 0;
    loop {
        temp_farenheit += 1.0;
        let temp_c:f64 = fahrenheit_to_celsius(temp_farenheit);
        println!("{:.2} degrees Farenheit is {:.2} degrees Celsius", temp_farenheit, temp_c);
        i += 1;
        if i >= 5 {
            break;
        }
    }
}