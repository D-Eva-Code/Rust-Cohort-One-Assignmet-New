use std::io;
use std::f64;
fn main() {
    let integer:u32;
    let mut log_value:f64= 0.0;

    loop{
    println!("1. Addition\n2. Subtraction\n3. Multiplication\n4. Division\n5. Gpa calculation\n6. Log(x)\n7. ln(x)");
    println!("Select Funtionality");
    let mut function= String::new();//creates a string variable to store user input
    io::stdin().read_line(&mut function).expect("failed to read input");//reads user input and stores in function
    match function.trim().parse::<u32>(){//convert user input to signed integer
        Ok(num)=>{
            integer= num;
            break;
        }
        Err(err)=>{
            println!("Invalid integer {:?}",err);//handling error incase user enters something other than integer
            continue;
        }
      }
    }
    let mut add_array:Vec<i32>= Vec::new();//creates an empty array to store user input
    fn array(add_array:&mut Vec<i32>){//created function to avoid repitition of code since it was used in 4 functionalities
        let mut add= String::new();//creates a string variable to store user input
        loop{
                io::stdin().read_line(&mut add).expect("failed to read input");//takes input from user
                if add.trim().is_empty(){//using trim to remove extra white space from inputed value and .is_empty to check if user did not input anything
                    break;// end the array when user presses enter(did not input anything)
                }
                for a in add.split(|c| c=='+'||c=='-'||c=='/'||c=='*'){//check for any of these seperator symbols used to split the user input and extract the individual values
                match a.trim().parse::<i32>(){
                    Ok(num)=>
                        add_array.push(num),//puts the user input in the empty array
                    Err(err)=>
                        println!("Invalid integer {:?}",err),
                  }
                }break;
            }
        }
        if integer==1{
            println!("Go ahead and add: ");
            array(&mut add_array);//used the array function to avoid repeating code
            let sum:i32= add_array.iter().sum();//loop through the array and add all the elements
            println!("the sum of the numbers is {}!",sum);
            }
        else if integer==2{
            println!("Go ahead and subtract: ");
            array(&mut add_array);
            let mut result= add_array[0];//result is equal to the first element in the array
            for a in &add_array[1..]{//loop through the array and subtract all the elements
                result-=a;
            }println!("Answer is {}!",result);
            } 

        else if integer==3{
            println!("Go ahead and multiply: ");
            array(&mut add_array);
            let mut result= add_array[0];
            for a in &add_array[1..]{//loop through the array and multiply all the elements
                result*=a;
            }
            println!("Answer is {}!",result);
            }

        else if integer==4{
            println!("Go ahead and Divide: ");
            array(&mut add_array);
            let mut result= add_array[0] as f32;
            for a in &add_array[1..]{//loop through the array and divide all the elements
                result/=*a as f32;//had to dereference the values in a first before converting i32 to f32
            }
            println!("Answer is {}!",result);
            }
        else if integer == 5 {
            let mut new_array: Vec<String> = Vec::new();//create new array to store grades
            println!("Enter grades (Separate with commas): ");
    
            loop {
                let mut function2 = String::new();
                io::stdin().read_line(&mut function2).expect("failed to read input");
                if function2.trim().is_empty() {//end the array when user presses enter(no more input)
                    break;
                }
                for a in function2.split(',') {//check for the , seperator used to split the user input to extract the individual values
                    new_array.push(a.trim().to_uppercase());//convert the user input to uppercase and push to the empty array and remove empty space
                }
    
                break; 
            }
    
            println!("Enter corresponding credit load");
            let mut new_array2: Vec<u64> = Vec::new();//create a new empty array
            loop {
                let mut function3 = String::new();//create a variable to store user input
                io::stdin().read_line(&mut function3).expect("failed to read input");//read user input
    
                if function3.trim().is_empty() {
                    break;
                }
                for a in function3.split(',') {
                    match a.trim().parse::<u64>() {
                        Ok(num) => new_array2.push(num),//push the input values to the array
                        Err(err) => println!("Invalid integer {:?}", err),
                    }
                }
                break;
            }
        
        let A=5;//assign credits for each grade
        let B=4;
        let C=3;
        let D=2;
        let E=1;
        let F=0;
        let mut total:u64=0;
        for (i,a) in new_array.iter().enumerate(){//loop through the grade array and keeps track of the values and indexes
            let grade= match a.as_str(){//converting the string to str 
                "A"=> A,
                "B"=> B,
                "C"=> C,
                "D"=> D,
                "E"=> E,
                "F"=> F,
                _ => {// for when the user enters an invalid grade 
                    println!("Invalid grade: {}", a);
                    continue;
                }
            };
            total+= grade*new_array2[i];//adds the total grade multiplied by the coursecredit load
        }
        let credit_sum:u64= new_array2.iter().sum();
        let gpa:f64= total as f64/credit_sum as f64;//calculates the final gpa
        println!("Your gpa is {}",gpa);
        if gpa <3.5{
            println!("Weldone, you can do better!");
        }
        else{
            println!("Great job. you're doing great!");
        }
    }
        else if integer==6{
            println!("calculate value of log: ");
            // let mut new_array3:Vec<f64>= Vec::new();
            let mut log_x=String::new();
            loop{
            log_x.clear();
            io::stdin().read_line(&mut log_x).expect("failed to readline");
            if log_x.trim().is_empty(){
                break;
            }
            if log_x.trim().starts_with("log(")&& log_x.trim().ends_with(")"){//check if the input is in the correct format starting with 'ln(' and ending with ')'
            if let Some(start)= log_x.find("log("){//checks the input for ln( and if its found it returns the index of each character
                if let Some(end)=log_x.find(")"){//checks the input for the closing bracket and if its found it returns the index of each character
                    let number= &log_x[start+4..end];//extracts only the number inside the bracket "()". The `+4` skips the "log(" part, and `end` makes sure the user stops before the closing ")".This is string slicing. And we also borrowed the value using the '&'
                match number.trim().parse::<f64>(){//converts the input which is a string to an f64 value
                    Ok(num)=>{log_value=num;
                        break;
                    }   
                    Err(err)=>{
                        println!("error, invalid integer {}",err);
                        continue;
                    } 
                }
                }else {
                    println!("Add closing bracket ')'");
                }
                }else{
                    println!("invalid format, use 'log(x)'");
                }
            } 
            }
                let log_x_answer= log_value.log(10.0);//uses std::f64 library to calculate log base 10 of a number
                println!("log({}) is {}",log_value, log_x_answer);
            } 

        else if integer==7{
            println!("calculate value of ln: ");
            // let mut new_array3:Vec<f64>= Vec::new();
            let mut log_x=String::new();
            loop{
            log_x.clear();
            io::stdin().read_line(&mut log_x).expect("failed to readline");
            if log_x.trim().is_empty(){
                break;
            }
            if log_x.trim().starts_with("ln(")&& log_x.trim().ends_with(")"){//check if the input is in the correct format starting with 'ln(' and ending with ')'
            if let Some(start)= log_x.find("ln("){//checks the input for ln( and if its found it returns the index of each character
                if let Some(end)=log_x.find(")"){//checks the input for the closing bracket and if its found it returns the index of each character
                    let number= &log_x[start+3..end];//extracts only the number inside the bracket "()". The `+3` skips the "ln(" part, and `end` makes sure the user stops before the closing ")".This is string slicing. And we also borrowed the value using the '&'
                match number.trim().parse::<f64>(){//converts the input which is a string to an f64 value
                    Ok(num)=>{log_value=num;
                        break;
                    }   
                    Err(err)=>{
                        println!("error, invalid integer {}",err);
                        continue;
                    } 
                }
                }else {
                    println!("Add closing bracket ')'");
                }
                }else{
                    println!("invalid format, use 'ln(x)'");
                }
            } 
            }
                let log_x_answer= log_value.ln();//uses std::f64 library to calculate ln of a number
                println!("ln({}) is {}",log_value, log_x_answer);
            } 
            
        }
            
        