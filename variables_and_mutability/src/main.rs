fn main() {
    fn reassignment() {
        let x = 5;
        println!("value of x: {x}");
        
        // x = 6; // -> needs to be mutable
        println!("value of x: {x}");        
    }

    fn mutability() {
        let mut x = 5;
        println!("value of x: {x}");

        x = 6;
        println!("value of x: {x}");
    }

    fn scope() {
        let x = 5;
        let x = x + 1;

        {
            let x = x * 2;
            println!("value of x in inner scope: {x}");
        }

        println!("value of x in outer scope: {x}");
    }

    fn shadowing() {
        let spaces = "     ";
        let spaces = spaces.len(); // shadowing allows type changing!
        println!("spaces: {spaces}");

        let mut dashes = "----";
        // dashes = dashes.len(); // -> results in compile-time error
        dashes = "some string"; // reassignment within same type is okay
        println!("dashes: {dashes}");
    }
}
