use std::io;

fn main() {
    fn explicit_typing() {
        let guess: u32 = "42".parse().expect("not a number!");
        println!("guess: {guess}");

        // let guess = "42".parse().expect("not a number!");
        // ^results in a compile-time error; needs specific type
    }

    fn char_type() {
        let c = 'z'; // single character only
        println!("char literal: {c}");
    }

    fn tuple_type() {
        let tup: (i32, f64, u8) = (-666, -38.31940, 88);
        // println!("{tup}"); // -> results in compile-time error
        // [`(i32, f64, u8)` cannot be formatted with the default formatter]

        let (x, y, z) = tup; // destructure
        println!("{x}, {y}, and {z}");

        // can also access tuple elements directly with index
        let a = tup.0;
        let b = tup.1;
        let c = tup.2;
        println!("{a}, {b}, and {c}");
    }

    fn arrays() {
        let a: [i32; 5] = [1, 2, 3, 4, 5]; // array: [type; amount]
        let b = ["hi", "hey", "sup", "hello"]; // don't need explicit typing
        
        let c = [44; 5]; // c = [44, 44, 44]
        let d = c[4];
        println!("d: {d}");
    }

    fn invalid_array_el_access() {
        let a = [1, 2, 3, 4, 5];

        println!("enter an array index");
    
        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("failed to read line");
        // (out of bounds index will cause thread to "panic" and result in runtime error)
    
        let index: usize = index
            .trim()
            .parse()
            .expect("index must be a number");
        let element = a[index];
        println!("i: {index} | el: {element}");
    }
}
