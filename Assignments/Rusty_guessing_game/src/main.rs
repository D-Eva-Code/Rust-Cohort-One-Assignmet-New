use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let mut parsed_number:i32;
    println!("Welcome!");
    let random:i32= rand::thread_rng().gen_range(1..=100);
    println!("Guess a number.");
    
    loop{
    let mut number= String::new();
    io::stdin().read_line(&mut number).expect("invalid number");
    match number.trim().parse::<i32>(){
        Ok(num)=>{parsed_number = num;}
        Err(_)=>{
            println!("Invalid");
            continue;
        }
    }
    if parsed_number==random{
        println!("you guessed it!");
        break;
    }
    else{
        match parsed_number.cmp(&random){
            Ordering::Less=>println!("Oops, value too low"),
            Ordering::Greater=>println!("value too high"),
            Ordering::Equal=>println!("Nice!"),
        }
        println!("Wrong guess, try again");
    }
}
    
    println!("the actual number is {}", random);
}
