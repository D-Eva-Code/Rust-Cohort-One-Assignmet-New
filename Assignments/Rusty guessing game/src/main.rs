use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Hello, I'm back!");
    let random:i32= rand::thread_rng().gen_range(1..=100);
    println!("how's school?, enter a number.");
    
    loop{
    let mut number= String::new();
    io::stdin().read_line(&mut number).expect("invalid number");

    let new_number:i32= match number.trim().parse(){
        Ok(num)=>{num}
        Err(_)=>{
            println!("Invalid");
            continue;
        }
    };
    if new_number==random{
        println!("you guessed it!");
        break;
    }
    else{
        match new_number.cmp(&random){
            Ordering::Less=>println!("Oops, value too low"),
            Ordering::Greater=>println!("value too high"),
            Ordering::Equal=>println!("Nice!"),
        }
        println!("Wrong guess, try again");
    }
}
    
    println!("the actual number is {}", random);
}
