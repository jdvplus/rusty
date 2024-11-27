fn main() {
    fn exercise1() {
        let x = 5;
        println!("value of x: {x}");
        
        // x = 6; // -> needs to be mutable
        println!("value of x: {x}");        
    }

    fn exercise2() {
        let mut x = 5;
        println!("value of x: {x}");

        x = 6;
        println!("value of x: {x}");
    }

    fn exercise3() {
        let x = 5;
        let x = x + 1;

        {
            let x = x * 2;
            println!("value of x in inner scope: {x}");
        }

        println!("value of x in outer scope: {x}")
    }

    fn exercise4() {
        let spaces = "     ";
        let spaces = spaces.len(); // shadowing allows type changing!
        println!("spaces: {spaces}")

        // below 
        let mut dashes = "----";
        // dashes = dashes.len(); // -> results in compile-time error
        dashes = "some string"; // reassignment within same type is okay
        println!("dashes: {dashes}");
    }
}
