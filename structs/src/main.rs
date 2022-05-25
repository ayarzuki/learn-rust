struct Person {
    name: String,
    age: u8,
    likes_oranges: bool
}

struct Point2D(u32, u32);

fn main() {
    let person = Person {
        name:String::from("John"),
        likes_oranges: true,
        age: 30,
    };

    println!("Person name is: {}", person.name);

    let origin = Point2D(100, 200);

    println!("Point contain {:?} and {:?}", origin.0, origin.1);

    let Point2D(x, y) = origin;

    println!("Point contain {:?} and {:?}", x, y);
}
