use std::fs::File;

fn main() {
    // panic!("Farewell");

    // let v = vec![1, 2, 3];
    // println!("{}", v[6]);

    // let fruits = vec!["banana", "apple", "pear"];

    // let first = fruits.get(0);
    // println!("{:?}", first);

    // let non_existent = fruits.get(10);
    // println!{"{:?}", non_existent};

    // let f = File::open("hello.txt").unwrap();

    let f = File::open("hello.txt").expect("Failed to open hello.txt");

    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => panic!("Can't open the file: {:?}", error),
    // };
}

