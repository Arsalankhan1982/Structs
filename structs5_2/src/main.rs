#[derive(Debug)]
struct Rectangle {
    width1: u32,
    height1: u32,
}

fn main() {
    let width = 30;
    let height = 50;
    let rect1 = Rectangle {
        width1: 20,
        height1: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        area1(&rect1)
    );
    println!("rect1 is {:?}", rect1);
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}
fn area1(rectangle: &Rectangle) -> u32 {
    rectangle.width1 * rectangle.height1
}
