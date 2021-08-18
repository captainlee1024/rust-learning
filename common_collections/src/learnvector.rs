#[test]
fn learn_vector() {
    // 新建 vector
    let mut v1: Vec<i32> = Vec::new();
    let mut v2 = vec![1, 2, 3];

    // 更新 vector
    v1.push(1);
    v1.push(2);
    v1.push(3);
    v1.push(4);
    v1.push(5);

    v2.push(1);

    // 丢弃 vector 时也会丢弃其所有元素
    {
        let _v = vec![1, 2, 3];
    }
    // _v.push(1);

    // 读取 vector 元素
    match v1.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    match v1.get(5) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 一旦程序获取了一个有效引用，借用检查器将会执行所有权和借用规则来确保 vector
    // 内容的这个引用和任何其他引用保持有效.
    // vector
    // 结尾新增元素时，空间不够，将所有的元素依次相邻存放的情况下可能会要求分配新内存将老的元素拷贝
    // 到新的内存空间中，第一个元素的应用指向的内存就被释放了。
    /*
    let first = &v1[0];
    v1.push(6);

    println!("The first element is: {}", first);
    */

    // 遍历 vector 中的元素
    for i in &v1 {
        println!("{}", i);
    }

    for i in &mut v1 {
        *i += 10;
    }

    for i in &v1 {
        println!("{}", i);
    }

    // 使用枚举来存储多种类型

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Text(String::from("Cyan")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("{:?}", row);
}
