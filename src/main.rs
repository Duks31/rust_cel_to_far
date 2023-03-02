// converting from celsius to farenheit in rust 
use std::io;

fn cel_to_far() {
    // F = °C × (9/5) + 32
    println!("Cel to Far");
    println!("Enter Celcuis");
    
    let mut cel =  String::new();
    io::stdin().read_line(&mut cel) .expect("Failed to read line.");

    let cel: f64 = cel.trim().parse().expect("Please type a number. ");

    let far = cel * (9.0/5.0) + 32.0;
    println!("{cel} is {far} in Farenheit. ");


}

fn far_to_cel() {
    // °C = (°F - 32) × 5/9
    println!("Far to Cel");
    let mut far =  String::new();
    io::stdin().read_line(&mut far) .expect("Failed to read line.");

    let far: f64 = far.trim().parse().expect("Please type a number. ");

    let cel = (far - 32.0) * 5.0/9.0;
    println!("{far} is {cel} in Celcius. ");
}

fn main() {
    println!(" Convert between Farenheit to celsuis in Rust! ");
    
    println!("Press 1 for Celuis to Farenheit\nPress 2 for Farenheit to celcuis");
    
    let mut choice =  String::new();

    io::stdin().read_line(&mut choice) .expect("Failed to read line.");

    let choice: u64 = choice.trim().parse().expect("Please type a number. Either 1 or 2  ");

    if choice == 1 {
        cel_to_far();
    }else if choice == 2 {
        far_to_cel();
    } else {
        println!("Invalid Conversion prompt");
    }
}