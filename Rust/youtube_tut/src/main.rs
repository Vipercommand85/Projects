fn main() {
    println!("Hello, world!");

    let x = 4;
    println!("x is: {}", x);
    // above is how to embed a variable in an out put line
    // x = 5;
    // println!("x is: {}", x);
    // the above code can not be used as is, as x is an immutable variable
    // would need to explicitly declare x to be a mutable variable with mut after let
    let x = 5;
    println!("x is: {}", x);
    // this is allowed as this is defining "new" variable with the same name a new value
    let x = "hello";
    println!("x is: {}", x);
    // because this is allowed, you can also redefine the type of a variable within the same scope
    
    {
        let x = 2;
        println!("x is: {}", x);
    }
    // the above code is in a different scope from the indented curly braces {}
    // this will allow the code to run normally as well as only run inside this scope

    let x = x + 1;
    println!("x is: {}", x);
    // interger values can be of type i8, i16, i32, i64, & i128 which are all signed intergers
    // default type is i32
    // can also use unsigned intergers, i.e. u32

    let b: u8 = 3;
    let y = x;
    println!("{}, {}", x, y);
    // y will inherit the data type of b in the above

    let floating_point: f32 = 10.92;
    println!("floating point number is: {}", floating_point);
    // there are 2 options with floating point numbers, f32 & f64
    // f64 is the default

    let true_or_false: bool = false; // or true
    println!("bool value is: {}", true_or_false);

    let letter: char = 'a';
    println!("letter is: {}", letter);
    // allows you represent a character and must used single quotes

    let mut tup: (i32, bool, char) = (1, true, 's');
    // can explicitly store different data types in the tuple
    // tuple are immutable by default but can be explicitly set to mutable
    // cannot add any more elements once the tuple has been defined
    println!("the second element in the tuple is:{}", tup.1);
    // use the .(indecies of the tuple) to print individual characters
    // can change individual indicies when the mut key word is used when defining the tuple
    let tup.0 = 5;
    println!("the first element in the tuple is:{}", tup.0);
    // change all of the elements in a tuple by explicitly stating the new types in an assingment statement
    // let tup = (24, false, 'b');
    //println!("the first element in the tuple is:{}", tup.0);

    let arr = [1, 2, 3, 4, 5];
    println!("the third element in the arr array is:{}", arr[2]);
    // arrays have to have the same data type for all elements
    // can access the individual elements of an array with [] and the index
    // you have to initialize all elements in an array and none can be empty or cannot create an empty array
    let mut arr2: [i64, 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("the fourth element in the arr2 array is:{}", arr[3]);
    arr2[3] = 27;
    println!("the fourth element in the arr2 array is:{}", arr[3]);
    // can also explicitly define the type and amount to exact number of elements in the array
    // can also add the mut key word to allow elements to be changed
    

    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);
    // have to define the type when defining a constant
    // u32 is an unsigned interger
    // cannot be redefined once this has been set
    
}
