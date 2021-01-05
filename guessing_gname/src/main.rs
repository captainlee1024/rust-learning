// 引入 io (输入输出库) 到当前作用域 io 来自标准库也被称作 std.
use rand::Rng;
use std::cmp::Ordering;
use std::io;

// main 函数是程序入口
// fn 声明了一个函数，() 表示没有参数
fn main() {
    // println! 是在终端打印字符串的宏.
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Pless input your guess.");

        // let 创建一个变量，mut 并把它的值绑定到 String::new()上
        // String::new 这个函数则返回一个 String 的新实例(String 是标准库提供的字符串类型，他是
        // UTF-8编码的可增长文本块)
        // ::new 中的 :: 表明 new 是 String 类型的一个关联函数(associated
        // function)．关联函数是针对类型实现的，在这个例子中是 String ,而不是 String 的某个特定实例．
        // 一些语言把它称为静态方法．
        // new 函数创建了一个新的空字符串
        // 这一行创建了一个可变变量，当前它绑定到一个新的 String 空实例上。
        let mut guess = String::new();

        // 如果不在开头引入标准库也可以这样写 std::io::stdin
        // 返回一个 std::io::stdin 的实例，代表终端标准输入句柄的类型
        // .read_line(&mut guess)调用read_line 方法从标准库输入句柄获取用户输入
        // read_line 的工作是，无论用户在标准输入中键入什么内容，都将其存入一个字符串中，
        // 因此它需要字符串作为参数。这个字符串参数应该是可变的，以便 read_line 将用户输入附加上去．
        // & 表示这个参数是一个引用（reference），它允许多出代码访问同一处数据，而无需在内存中多次拷贝
        // 应用是一个复杂的特性，Rust 的一个主要优势就是安全而简单的操纵引用．
        // 现在我们只需知道，他就像变量一样默认是不可变的，因此需要协程 &mut guess 来使其可变，而不是
        // &guess
        // Result(枚举) 类型来处理潜在的错误，这里返回的是 Result子模块中的具体版本 io::Result
        // Result 的成员是 Ok 和 Err，Ok 成员表示操作成功，内部包含成功时产生的值．
        // Err 成员则意味着操作失败，并且包含失败的前因后果．
        // 如果不调用 expect，程序也能编译，不过会出现一个警告：
        // warning: unused `std::result::Result` which must be used
        io::stdin()
            .read_line(&mut guess)
            .expect("Field to read line");

        //let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // {} 是预留在特定位置的占位符。使用 {} 也可以打印多个值
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
