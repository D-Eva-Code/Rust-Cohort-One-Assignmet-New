fn main(){
    let mut num= 1;
    while num<13{
    println!("On the {} day of christmas my true love gave to me..", num);//prints for all the days
    days(num);//uses the days function created
    println!("\n");//seperates each part of the song with a new line
    num +=1;//increment num by 1 after each 
    }
   
}

fn days(num: usize) {
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for i in (0..num).rev() {// to loop from num-1 down to 0 ie from index 11 to 0 
        if i == 0 && num > 1 {
            println!("And {}", gifts[i]); // adds "And" only for the last line 
        } else {
            println!("{}", gifts[i]);
        }
    }
}