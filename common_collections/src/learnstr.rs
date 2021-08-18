#[test]
fn learn_str() {
    // 新建空字符串
    let mut s1 = String::new();

    // 从字符串字面值创建字符串
    let data = "initial contents";
    let s2 = data.to_string();

    // 该方法用于任何实现了Display trait 的类
    // 也可以作用与字符串字面量
    let _s3 = "initial contents".to_string();

    // 使用 String::from() 从字符串字面值创建 String
    let _s4 = String::from("initial contents");

    // 使用 push_str 追加字符串 slice
    let mut s5 = String::from("foo");
    let s_bar = "bar";
    s5.push_str(s_bar);
    println!("s5 is {}", s2);

    // push 只能将一个字符追加到字符串中
    let mut s6 = String::from("lo");
    s6.push('l');
    println!("s6 is {}", s6);

    // 使用 + 运算符或format!宏拼接字符串
    // 注意 s7 所有权被移动了，不能继续使用了
    let s7 = String::from("Hello, ");
    let s8 = String::from("world!");
    let s9 = s7 + &s8;
    println!("{}", s9);

    // 使用 format! 宏
    let s10 = String::from("tic");
    let s11 = String::from("tac");
    let s12 = String::from("toc");

    let s13 = format!("{}-{}-{}", s10, s11, s12);
    println!("{}", s13);

    // 索引字符串
    // 如果尝试索引字符串的一部分，会出现错误
    // let s14 = String::from("hello");
    // let h = s14[0];

    // 内部表现: String 是一个 Vec<u8>的封装
    let len = String::from("Hola").len();
    println!("Hola len: {}", len);
    // 这里len为4，意味者存储字符串Hola的长度为四个字节;这里的每个字母的UTF-8编码都占用一个字节

    // 这里len为24，每个Unicode标量值需要两个字节存储
    // 因此一个字符串字节值的索引并不总是对应一个有效的Unicode标量值
    let len = String::from("Здравствуйте").len();
    println!("Здравствуйте len: {}", len);

    // 字节、标量值和字形簇
    // 从Rust角度看，有三种相关方式可以理解字符串：字节、标量值、字形簇（字母）
    // 字节： [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    // Unicode标量角度看： ['न', 'म', 'स', '्', 'त', 'े']
    // 字形簇（字母）角度看: ["न", "म", "स्", "ते"]

    // 字符串slice
    // 不能索引字符串，因为返回的类型是不明确的，因此，让我们真正需要索引时，我们应该明确这些东西
    // 这时候就需要一个字符串slice
    let s15 = "Здравствуйте";
    let s16 = &s15[0..4];
    println!("s15: {}, s16: {}", s15, s16);
    // 如果此时我们索引 1，3，5会发生panic，无效索引。

    // 遍历字符串的方法
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for c in "नमस्ते".bytes() {
        println!("{}", c);
    }
}
