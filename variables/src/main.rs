use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is {}.", x);
    x = 6;

    // Shadowing
    // declear a new variable whti the same as previous variable
    // and the new varibale shadows the pervious variable
    println!("The value of x is {}.", x);
    let y = 1;
    let y = y + 2;
    let y = y * 2;
    println!("The value of y is {}.", y);

    // The other difference between mut and shadowing.
    // shadowing(use the let keyword): we can change the type of the value but reuse the same name.
    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    // mus: we'll get a complie-time error: we're not allowed to mutate a variable's type
    //let mut spaces2 = "    ";
    //spaces2 = spaces2.len();

    // Data Types

    // scalar
    // Rust has fourprimary salar tyeps: integers, floating-point, numbers, Booleans and characters

    // intagers: i8, i16, i32, i64, i128, isize
    // the isize type depend on the kind of computer your program is running on

    // floating-point: f32, f64(default)
    let f_default = 1.0; // f64
    let y_f32: f32 = 2.0; // f32
    println!("default flot(f64): {}, f32: {}", f_default, y_f32);

    // Numberic Operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multipication
    let produc = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
    println!(
        "result: add: {}, sub: {}, mul: {}, div: {}, rem: {}",
        sum, difference, produc, quotient, remainder
    );

    // The Boolean Type
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("Boolean: t: {}, f: {}", t, f);

    // The Character Type
    // Rust's char type is four bytes in size and represents a Unicode Scalar Value
    let c = 'c';
    let c_chinese = '中';
    let c_x = '😻';
    println!("char: c: {}, c_chinese: {}, c_x: {}", c, c_chinese, c_x);

    // Compound Types
    // The Tuple Type
    let tup1: (i32, f64, u8) = (500, 6.4, 1);

    let tup2 = (500, 6.2, 1);
    let (x, y, z) = tup2;
    println!("The value of x is: {}, y: {}, z: {}", x, y, z);

    let five_hundred = tup1.0;
    let six_point_four = tup1.1;
    let two = tup1.2;
    println!(
        "five_hundred: {}, six_point_four: {}, two: {}",
        five_hundred, six_point_four, two
    );

    // The Array Type
    let arr_1 = [1, 2, 3, 4, 5];
    println!("arr_1: {}", arr_1[0]);

    let months = ["January", "February", "..."];
    println!("1: {}", months[1]);

    let arr_i32: [i32; 3] = [1, 2, 3];
    println!("arr_i32(len 3): {}", arr_i32[1]);

    // This is the same as writing let a = [3, 3, 3, 3, 3];
    let arr_i32_plus = [3; 5];
    println!("arr_i32_plus: {}", arr_i32_plus[4]);

    // Invalid Array Element Access
    let arr_invalid = [1, 2, 3, 4, 5];

    let mut index = String::new();

    println!("Please enter an array index.");
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to rean line");

    let index: usize = index.trim().parse().expect("Index enterd was not a number");

    let element = arr_invalid[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );


    // Functions
    // Rust code uses snake case as the conventional style for function and variable names.
    another_function();

    // Function Parameters
    another_function_with_param(5);
    fun_with_2_params(5, 6);

    // Function Bodies Contain Statements and Expressions

    // Statements
    // let y = 6; is a statement
    // Statements do not return values. you can't assign a let statement to another variable
    // eg: let x = (let y = 6);

    // Expressions
    // the 6 in the statement let y = 6; is an expression that evaluater to the value 6
    // consider a simple math operation, such as 5 + 6
    // Calling a function si an expression.
    // Calling a macro is an expression.
    // The block that we use to create new scops, {}, is an expression, for example:
    let x_1 = 5;
    println!("{}", x_1);

    // Note the x + 1 line without a semicolon at the nd,
    // whtch is unlike most of the lines you've seen so far. Expressions do not include ending semicolons.
    // If you add a semicolon to the end of an expression,
    // you turn it into a statement, which will then not return a value
    let y = {
        let x_1 = 3;
        x_1 + 1
    };
    println!("y: {}", y);

    // Function with Reture Values
    let return_x = fun_with_reture_values();
    println!("The value of return_x is: {}", return_x);

    let plus_5 = plus_one(5);
    println!("The value of plus_5 is: {}", plus_5);

    // Comments
    // Control Flow
    // if Expressions
    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // The is condition evalutes to a value of 3 this time, and Rust throws an error
    // if number {
    //     println!("number was xxx");
    // }
    // Unlike languages such as JS ... Rust will not automatically try to convert non-Boolean types
    // to Boolean.

    // Handling Multiple Conditions with else if
    let number_two = 6;
    if number_two % 4 == 0 {
        println!("number is divisible by 4");
    } else if number_two % 3 == 0 {
        println!("number is divisible by 3");
    } else if number_two % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 2");
    }

    // Using if in a let Statement
    let condition = true;
    let number_3 = if condition {
        5
    } else {
        6
    };
    println!("The value of number is: {}", number_3);

    // if the types are mismatched, as in the following example
    // let condition_2 = true;
    // let number_4 = if condition_2 { 5 } else { "six" };

    // Repetition with Loops
    // repeating Code with loop
    // loop {
    //     println!("again!");
    // }

    // Returning Values from Loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // Conditional Loops with while
    let mut number_4 = 3;
    while number_4 != 0 {
        println!("{}!", number_4);

        number_4 = number_4 - 1;
    }
    println!("LIFTOFF!");

    // Looping Through a Collection with for
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // rev range:
    for number in (1..5).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_param(x: i32) {
    println!("The value of x is: {}", x);
}

fn fun_with_2_params(x: i32, y: i32) {
    println!("The value of x is: {}, y: {}", x, y);
}

fn fun_with_reture_values() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
