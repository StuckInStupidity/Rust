struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };
    println!("The area of the rectangle is {} square pixels.", area(&rect1));

    let float: f32 = 4.82832;
    let floored_float = float.floor();
    let sqrt_of_four = floored_float.sqrt();
    let sinus_of_four = floored_float.sin();
    let exponential_of_four = floored_float.exp();
    println!("Floored test float {} to {}", float, floored_float);
    println!("The square root of {} is {}", floored_float, sqrt_of_four);
    println!("The sinus of {} is {}", floored_float, sinus_of_four);
    println!("The exponential of {} to the base e is {}", floored_float, exponential_of_four)
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}