#[derive(Debug)]
struct Rectangle{
    length: u32,
    breadth: u32,

}
fn area(rectangle: &Rectangle)->u32{
    rectangle.length* rectangle.breadth
}

#[derive(Debug)]
#[allow(dead_code)]
enum Direction{
    Up,
    Down,
    Left,
    Right,
}

enum Shape{
    Circle(f64),
    Rectanglee(f)
}
fn main() {
    let rect1= Rectangle{
        length: 30,
        breadth: 40,
    };
    println!("The area of the rectangle is {}", area(&rect1));
    let move_down= Direction::Down;
    println!("Moving {:?}", move_down);
}
