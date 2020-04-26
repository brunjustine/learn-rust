//Training 1 _
//Convert temperatures between Fahrenheit and Celsius.
//just.brun@hotmail.fr ~ Brun Justine

use std::io;


fn read_number () ->f32 {
    println!("=>");
    let mut number = String::new();

    //read number
    io::stdin().read_line(&mut number)
            .expect("Failed to read line");

    //convert string to float
    let number: f32 = number.trim().parse()
            .expect("Failed to read number");
    number
}

fn fahrenheit_to_celsius() {
    let number = read_number();
    let res : f32 = (number-32.0)/1.8;
    println!(" T°({}) in celsius \n
               T°({}) in fahrenheit",number,res);
}

fn celsius_to_fahrenheit() {
    let number = read_number();
    let res : f32 = number*1.8+32.0;
    println!("  T°({}) in fahrenheit \n
                T°({}) in celsius ",number,res);
}

fn main() {
    println!("1. Convert to Fahrenheit");
    println!("2. Convert to Celsius");
    println!("What do you want ?");
    
    let choice = read_number();

    //select the connvertion 
    if choice == 1.0 {
        celsius_to_fahrenheit();
    } else if choice == 2.0 {
        fahrenheit_to_celsius();
    }else {
        println!("your number of choice is incorrect!");
    }
    
}
