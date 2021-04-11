fn main() {
    // Defining and Instantiating Structs
    let user1 = User {
        email: String::from("someone@example"),
        username: String::from("someusername123"),
        active: true,
        sign_in: 1,
    };
    println!("{:#?}", user1);

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123x"),
        active: true,
        sign_in: 1,
    };

    user2.email = String::from("anotheremail@example");

    // Creating Instances From other Instance With Struct Update Syntax
    let _user3 = User {
        email: String::from("another@example"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in: user1.sign_in,
    };

    // we can achieve the same effect with less code (..)
    let _user4 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername789"),
        ..user1
    };

    // Using Tuple Structs without Named Fields to Create Different Types
    // tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // Unit-Like Structs Without Any Fields
    // You can also define structs that don't have any fields! These are called unit-like structs
    // because they behave similarly to (), the unit type.
    // Unit-like structs be useful in situations in which you need to implement a trait on some type
    // but don't have any data that you want to store in the type itself.

    // Ownership of struct Data
    // Let's say you try to store a reference in a struct without specifying lifetimes, like this
    // which won't work:
    /*struct User2 {
        username: &str,
        email: &str,
    };

    error[E0106]: missing lifetime specifier
        --> src/main.rs:53:16
        |
     53 |         email: &str,
        |                ^ expected named lifetime parameter
    */

    // An Example Program Using Structs
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // Refactoring with Tuples
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1)
    );

    // Refactoring with Structs: Adding More Meaning
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area_struct(&rect2)
    );

    // Adding Useful Functionality with Derived Traits
    // println!("{}", rect2);

    // error[E0277]: `Rectangle` doesn't implement `std::fmt::Display`
    // we'll find this helpful note:
    // help: the trait `std::fmt::Display` is not implemented for `Rectangle`
    // = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    // println!("{:?}", rect2);

    // we still get an error:
    // error[E0277]: `Rectangle` doesn't implement `Debug`
    // But again, the compiler gives us a helpful note:
    //    = help: the trait `Debug` is not implemented for `Rectangle`
    //    = note: add `#[derive(Debug)]` or manually implement `Debug`

    println!("{:?}", rect2);
    println!("{:#?}", rect2);

    // Method Syntax
    // Defining Methods
    println!("{:#?}", rect2.area());

    /*
    -> 运算符到哪去了？
    在 C/C++ 语言中，有两个不同的运算符来调用方法：. 直接在对象上调用方法，而 -> 在一个对象的指针上调用方法，这时需要先解引用（dereference）指针。
    换句话说，如果 object 是一个指针，那么 object->something() 就像 (*object).something() 一样。

    Rust 并没有一个与 -> 等效的运算符；相反，Rust 有一个叫 自动引用和解引用（automatic referencing and dereferencing）的功能。
    方法调用是 Rust 中少数几个拥有这种行为的地方。

    他是这样工作的：当使用 object.something() 调用方法时，Rust 会自动为 object 添加 &、&mut 或 * 以便使 object 与方法签名匹配。也就是说，这些代码是等价的：

    p1.distance(&p2);
    (&p1).distance(&p2);
    第一行看起来简洁的多。这种自动引用的行为之所以有效，是因为方法有一个明确的接收者———— self 的类型。在给出接收者和方法名的前提下，
    Rust 可以明确地计算出方法是仅仅读取（&self），做出修改（&mut self）或者是获取所有权（self）。事实上，Rust 对方法接收者的隐式借用让所有权在实践中更友好。
    */

    // Methods with More Parameters
    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect4 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect5 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect3 hold rect4? {}", rect3.can_hold(&rect4));
    println!("Can rect3 hold rect5? {}", rect3.can_hold(&rect5));

    // Associated Functions
    // Another useful feature of impl blocks is that we're allowed to define functions within impl blocks that don't take self as parameter.
    let square1 = Rectangle::square(100);
    println!("square1: {:#?}", square1);

    // Multiple impl Blocks
    Rectangle::another_impl_func();
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in: u64,
    active: bool,
}

fn _build_user(email: String, username: String) -> User {
    /*User {
        email: email,
        username: username,
        active: true,
        sign_in: 1,
    }*/

    // Using the Field Init Shorthand when Variable and Fields Have the Same Name
    User {
        email,
        username,
        active: true,
        sign_in: 1,
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Methods
    // rectangle: &Rectangle -> &self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated Functions
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Multiple impl Blocks
impl Rectangle {
    fn another_impl_func() {
        println!("Multiple impl Blocks");
    }
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
