use std::fmt::Display;

// 生命周期于有效引用

// 生命周期避免了悬垂引用
// 如下：外部作用域声明了一个没有初始值的变量r, 在内部声明了一个初始值为5的变量x, 在内部作用域中,
// 我们尝试将r的值设置为一个x的引用。
// 但是, 变量x并没有处在的足够久，x在内部作用域结束 '}' 那一行就离开了作用域, 被释放掉了,
// 但是r在外部作用域是有效的, 如果这段代码继续工作, r将会引用在x离开作用域时被释放的内存
// 作用域越大我们就说它存在的越旧
/*
#[test]
fn test_lifetime1() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);
}
*/

// 借用检查器
// rust 编译器又一个借用检查器 borrow checker , 它比较作用域来确保所有的借用都是有效的

// 函数中的泛型生命周期

// 生命周期注解语法
// eg:
// &i32 //引用
// &'a i32 //带有显示生命周期的引用
// &'a mut i32 //带有显示生命周期的可变引用
// 单个的生命周期注解本身没有多少意义，因为生命周期注解告诉rust多个引用的泛型生命周期
// 参数如何相互联系的。例如如果函数又一个生命周期'a的

// 函数签名中的生命周期注解
/*
 * 不能编译，因为不确定那个代码块会被执行，不确定返回的是x还是y
fn longest(s: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
*/
#[allow(unused)]
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 深入理解生命周期
// 指定生命周期参数的正确方式依赖函数实现的具体功能。
// 例如，如果将 longest 函数的实现修改为总是返回第一个参数而不是最长的字符串 slice，
// 就不需要为参数 y 指定一个生命周期。如下代码将能够编译：
#[allow(unused)]
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// 当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配
// 。如果返回的引用 没有 指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值，
// 它将会是一个悬垂引用，因为它将会在函数结束时离开作用域

// note!!! 无法指定生命周期参数来改变悬垂引用，而且 Rust 也不允许我们创建一个悬垂引用。
// 在这种情况，最好的解决方案是返回一个有所有权的数据类型而不是一个引用，
// 这样函数调用者就需要负责清理这个值了。
/*
fn longest3<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
*/

#[test]
fn lifetime_test() {
    // 错误示例
    /*
     * 虽然这里返回的是string1的引用，但是，生命周期确定与最小的保持一致
     * 所以，result的生命周期与string2相同，在代码块外已经被释放
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }

    println!("The longest string is {}", result)
    */
}

// 结构体定义中的生命周期注解

/*
这个结构体有一个字段，part，它存放了一个字符串 slice，这是一个引用。
类似于泛型参数类型，必须在结构体名称后面的尖括号中声明泛型生命周期参数，
以便在结构体定义中使用生命周期参数。
这个注解意味着 ImportantExcerpt 的实例不能比其 part 字段中的引用存在的更久。
*/
#[allow(unused)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[test]
fn lifetime_in_struct_test() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentenct = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentenct,
    };
}

// 生命周期省略
/*
 * Q: 该函数的入参和返回值都是引用，为什么不用指定生命周期
 * A: Rust团队发现在特定情况下Rust程序员们总是重复的写着一摸一样的生命周期，可预测，且遵循
 * 几个明确的模式，就把它们写进了Rust编译器
 *
 * 被编码进Rust引用分析的模式被称为生命周期省略规则。
 * 省略规则并不提供完整的推断：
 * 如果 Rust 在明确遵守这些规则的前提下变量的生命周期仍然是模棱两可的话，
 * 它不会猜测剩余引用的生命周期应该是什么。在这种情况，编译器会给出一个错误，
 * 这可以通过增加对应引用之间相联系的生命周期注解来解决。
 *
 * 函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），
 * 而返回值的生命周期被称为 输出生命周期（output lifetimes）。
 *
 * 用于输出生命周期。如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，
 * 编译器将会停止并生成错误。这些规则适用于 fn 定义，以及 impl 块。
 *
 * 一：
 *  第一条规则是每一个是引用的参数都有它自己的生命周期参数。
 *  换句话说就是，有一个引用参数的函数有一个生命周期参数：fn foo<'a>(x: &'a i32)，
 *  有两个引用参数的函数有两个不同的生命周期参数，fn foo<'a, 'b>(x: &'a i32, y: &'b i32)，依此类推。
 *
 * 二:
 *  第二条规则是如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：
 *  fn foo<'a>(x: &'a i32) -> &'a i32。
 *
 * 三:
 *  第三条规则是如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，
 *  说明是个对象的方法(method)(译者注： 这里涉及rust的面向对象参见17章),
 *  那么所有输出生命周期参数被赋予 self 的生命周期。第三条规则使得方法更容易读写，因为只需更少的符号。
 */
#[allow(unused)]
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// 方法定义中的生命周期注解
// impl 之后和类型名称之后的生命周期参数是必要的，
// 不过因为第一条生命周期规则我们并不必须标注 self 引用的生命周期。
#[allow(unused)]
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // 这里有两个输入生命周期，所以 Rust 应用第一条生命周期省略规则并给予 &self 和
    // announcement 他们各自的生命周期。接着，因为其中一个参数是 &self，
    // 返回值类型被赋予了 &self 的生命周期，这样所有的生命周期都被计算出来了。
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// 静态生命周期

/*
 * 这里有一种特殊的生命周期值得讨论：'static，其生命周期能够存活于整个程序期间。
 * 所有的字符串字面值都拥有 'static 生命周期，我们也可以选择像下面这样标注出来
 *
 * 这个字符串的文本被直接储存在程序的二进制文件中而这个文件总是可用的。
 * 因此所有的字符串字面值都是 'static 的。
let s: &'static str = "I have a static lifetime.";
*/

// 结合范型类型参数、trait bounds 和生命周期

/*
 * 这个是示例 10-22 中那个返回两个字符串 slice 中较长者的 longest 函数，
 * 不过带有一个额外的参数 ann。ann 的类型是泛型 T，
 * 它可以被放入任何实现了 where 从句中指定的 Display trait 的类型。
 * 这个额外的参数会在函数比较字符串 slice 的长度之前被打印出来，
 * 这也就是为什么 Display trait bound 是必须的。因为生命周期也是泛型，
 * 所以生命周期参数 'a 和泛型类型参数 T 都位于函数名后的同一尖括号列表中。
 */
#[allow(unused)]
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
