fn main(){
    let mut num= 1;
    while num<13{
    println!("On the {} day of christmas my true love gave to me..", num);
    days(num);
    println!("\n");
    num +=1;
    }
   
}
// fn days(mut num:i32){
//     loop{
//         if num ==1{
//             println!("A partrich in a pear tree");
//             break;
//         }
//         if num==2{
//             println!("Two turtle doves");
//             num -=1;
    
//     }
// }
// }

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

    for i in (0..num).rev() {// to loop from num-1 down to 0
        if i == 0 && num > 1 {
            println!("And {}", gifts[i]); // "And" only for the last line 
        } else {
            println!("{}", gifts[i]);
        }
    }
}