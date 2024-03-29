fn main() {
    println!("Hello, world!");

    let x = 4;
    println!("x is: {}", x);
     
// above is how to embed a variable in an out put line
    
    {
        let x = 2;
        println!("x is: {}", x);
    }
// the above code is in a different scope from the indented curly braces {}
// this will allow the code to run normally as well as only run inside this scope

    let x = x + 1;
    println!("x is: {}", x);
    
}
