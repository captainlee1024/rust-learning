fn main() {
    // What is Ownership
    // The Stack and the Heap
    // Ownership Rules
    // 1. Each value in rust has a variable that's called tis owner
    // 2. There can only be one owner at a time
    // 3. When the owner goes out of scope, the value will be dropped

    // Variable Scope
    let s_1 = "hello";
    println!("{}", s_1);

    {
        // s_2 is not a valid here, it's not yet declared
        let s_2 = "hello"; // s_2 is valid from this point forward
        println!("{}", s_2);
    } // this scope is now over, and s_2 is no longer valid

    // The String Type
    // :: is an operator, that allows us to namespace this particular from function under the String
    // Type rather than using some sort of like string_name.
    let s_3 = String::from("hello");
    println!("{}", s_3);

    let mut s_4 = String::from("hello");
    s_4.push_str(", world!");
    println!("{}", s_4);

    // Memory and Allocation
    // the memory is automatically returned once the variable that owns it goes out of scope.
    // When a variable goes out of scope, Rust calls a special function for us.

    // Ways Variables and Data Interact: Move
    // bind the value 5 to x_1
    // then make a copy of the value in x_1 and bind to y_1
    // because integers are simple values with a known, fixed size, and these two 5 values are pushed
    // onto the stack
    let x_1 = 5;
    let y_1 = x_1;
    println!("x_1 {}, y_1 {}", x_1, y_1);

    // if Rust did this, the operation s_5 = s_6 could be very expensive in terms of runtime performance
    // if the data on the heap were large.
    // we copy the pointer, the length, and the capacity that are on the stack. we do not copy the
    // data on the heap that the pointer refers to. but, when a variable out of scope, Rust automatically
    // call the drop function and cleans up the heap memory for that variable. they will both try to free
    // the same memory. this is known as a double free error.
    // Instead of trying to copy the allocated memory, Rust considers s_5 to no longer be valid and,
    // therefore, Rust doesn't need to free anything when s_5 goes out of scope.
    let s_5 = String::from("hello");
    let s_6 = s_5;
    // println!("s_5 {}, s_6{}", s_5, s_6); // error
    println!("{}", s_6);
    // But because Rust also invalidates the first variables the first variable, instead of being
    // called a shallow copy, it's know as a move.

    // Ways Variables and Data Interact: Clone
    let s_7 = String::from("hello");
    let s_8 = s_7.clone();
    println!("s_7 {}, s_8 {}", s_7, s_8);

    // Stack-Only Data: Copy
    // Rust has a special annotation called the copy trait, that we can place on types like integers that
    // are stored on the stack. // TODO:完善英文注释
    // integer, boolean, floating, character, tuples(if they only types that also implement Copy)
    //
    let x_2 = 5;
    let y_2 = x_2;
    println!("x_2 {}, y_2 {}", x_2, y_2);

    // Ownership and Functions
    let s_8 = String::from("hello"); // s_8 comes into scope
    takes_ownership(s_8); // s_8's value move into the function ..
                          // ... and so is no longer valid herr
                          // println!("{}", s_8);

    let x_3 = 5; // x_3 comes into scope
    make_copy(x_3); // x_3 would move into the function
                    // but i32 is Copy, so it's okay to still use x_3 afterward
    println!("x_3 {}", x_3);

    // Return Values and Scope
    let s_9 = gives_ownership();
    println!("s_9 {}", s_9);

    let s_10 = takes_and_gives_back(s_9);
    // println!("s_9 {}", s_9);
    println!("s_10 {}", s_10);

    // It’s possible to return multiple values using a tuple
    let s_11 = String::from("hello");
    let (s_12, len) = calculate_length(s_11);
    println!("s_12 {}, len {}", s_12, len);

    // References and Borrowing
    //we want to let a function use a value but not take ownership
    // these & ampersands are references, and they allow you to refer to some value without taking
    // ownership of it.
    // Note: The opposite of referencing by using & is dereferencing, which is accomplished with the
    // dereference operator, *.
    // we call having references as function parameters borrowing
    // just as variables are immutable by default, so are references. We're not allowed to modify something
    // we have a reference to.
    let s_13 = String::from("hello");
    let x_4 = calculate_length_2(&s_13);
    println!("s_13 {}, x_4 {}", s_13, x_4);

    // Mutable Reference
    let mut s_14 = String::from("hello");
    change(&mut s_14);
    println!("s_14 {}", s_14);

    // You can have only mutable reference to a particular piece of data in a particular scope.
    // The benefit of having this restriction is that Rust can prevent data races at compile time.
    let mut s_15 = String::from("hello");
    let r1 = &mut s_15;
    println!("{}", r1);
    let r2 = &mut s_15;
    // println!("{}, {}", r1, r2); // error
    println!("{}", r2);

    // we also have a mutable reference while we have an immutable one.
    let mut s_16 = String::from("hello");
    let r3 = &s_16; // no problem
    let r4 = &s_16; // no problem
                    // let r5 = &mut s_16; // BIG PROBLEM
                    // note that a reference's scope from where it is introduced and continues through the last time
                    // that reference is used.
    println!("r3 {}, r4 {}", r3, r4);
    let r5 = &mut s_16; // no problem
    println!("{}", r5);

    // Dangling References
    // in languages with pointers, it's easy to erroneously create a dangling pointer, a pointer that
    // references a location in memory that may have been given to someone else, by freeing some memory
    // while preserving a pointer to that memory.
    // let reference_to_nothing = dangle(); // error

    // The Slice Type
    // Another data type that does not have ownership is the slice.
    let mut s_17 = String::from("hello, world");
    let word = first_word(&s_17); // word will get the value 6
    s_17.clear(); // this empties the String, making it equal to ""
                  // word still has the value 6 here, but there's not more String that we cloud meaningfully use the
                  // value 6 with. word is now totally invalid!
    println!("word {}", word);
    // We could use that values 6 with the variables s_17 to try to extract the first word out, but
    // this would be a bug because the contents of s_17 have changed since we saved 6 in word.

    // String Slices
    // A String Slice is a reference to part of a String and it looks like this:
    let s_18 = String::from("hello, world");
    // Rather than a reference to the entire String, it's a reference to portion of the String.
    let hello = &s_18[0..5];
    let world = &s_18[7..12];
    println!("hello world, {} {}", hello, world);

    // With Rust's .. range syntax, if you want to start at the first index(zero), you can drop the
    // value before the two periods.
    // In other words, there are equal
    let s19 = String::from("hello");
    println!("s19 {}", s19);
    let slice1 = &s19[0..2];
    let slice2 = &s19[..2];
    println!("slice1 {} slice2 {}", slice1, slice2);
    // there are equal
    let slice3 = &s19[3..len];
    let slice4 = &s19[3..];
    println!("slice3 {} slice4 {}", slice3, slice4);
    // there are equal
    let slice5 = &s19[0..len];
    let slice6 = &s19[..];
    println!("slice5 {} slice6 {}", slice5, slice6);
    // Note: String slice range indices must occur at valid UTF-8 character boundaries.
    // If you attempt to create a string slice in the middle of a multibyte character, your program
    // will exit with an error.

    // first_word BugFix with string slice
    // let mut s20 = String::from("hello, world");
    let s20 = String::from("hello, world");
    let word2 = first_word_slice(&s20);
    // s20.clear();
    // if we have an immutable reference to something, we cannot also take a mutable reference.
    // Because clear needs to truncate the String, it needs to get a mutable reference.
    // Rust disallows this, and compilation fails.
    // Rust eliminated an entire class of errors at compile time!
    println!("the first word is: {}", word2);

    // String Literals Are Slices
    let s21 = "hello, world!";

    // String Slices as Parameters
    // 在知道了能够过去字面值和 String 的 slice 后，我们对 first_word_slice 改进，让它能够对 String 值 和 &str 值
    // 使用相同的函数
    // 因为字符串字面值就是字符串 slice，所以直接传s21也可以
    let word3 = first_word_slice_pro(s21);
    println!("word3 {}", word3);
    let word4 = first_word_slice_pro(&s20[..]);
    println!("word4 {}", word4);

    // Other Slices
    let number_slice = [1, 2, 3, 4, 5];
    // Just as we might want to refer to a part of a String, we might want to refer to part of an array
    // We'd do so like this:
    let slice_part_num = &number_slice[1..3];
    println!("slice_part_num {}", slice_part_num[0]);
} // Here, x_3 goes out of scope, then s_8. But because s_8's value was moved, nothing special happens.

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_2(s: &String) -> usize {
    // s is a reference to a string
    s.len()
} // Here, s gone out of scope. But because it does not have ownership of what it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", hello!");
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//
//     &s
// }

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    // 因为需要逐个元素检查 String 中的值是否为空格，需要用 as_bytes 方法将 String 转化为字节数组
    let bytes = s.as_bytes();

    // 使用 iter 方法在字节数组上创建一个迭代器
    // iter is a method that returns each element in a collection
    // enumerate wraps the result of iter and returns each as part of a tuple instead.
    // The first element of the tuple returned from enumerate is the index, and the second element is
    // a reference to the element.
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_slice_pro(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b',' {
            return &s[0..i];
        }
    }

    &s[..]
}
