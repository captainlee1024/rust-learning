// 如果想让别人也能够使用这个creat 中的trait 为他自己的类型实现 Summary中的方法
// 我们必须声明为共有trait，在trait前加pub关键字
// 这样别人可以用过 use learntrait::Summary实;然后就可以为其类型实现Summary trait
// note! 实现trait时需要注意的一个限制，只有当trait或者要实现trait的类型位于crate的本地作用域时
// 才能为该类型实现trait
#![allow(unused)]

use std::fmt::{Debug, Display};
pub trait Summary {
    fn summary(&self) -> String;
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

#[derive(Debug)]
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

#[test]
fn summary_test1() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summary());
}

// 为trait提供默认实现

pub trait Summary2 {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

// 我们不直接为 NewArticle 定义 summarize 方法，而是使用该trait提供的默认实现
// 可以通过impl Summary2 for NewArticle {} 指定一个空的 impl 块
impl Summary2 for NewArticle {}

#[test]
fn summary2_test() {
    let article = NewArticle {
        headline: String::from("Penguins win the Stanley Cup Champinoship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best
                              hockey team in the NHL.",
        ),
    };

    println!("New article avalible! {}", article.summarize());
}

// 默认实现允许调用相同trait 中的其他方法，哪怕这些方法没有提供默认实现
// note! 无法从相同方法的重载实现中调用默认方法。
pub trait Summary3 {
    fn summarsize_author(&self) -> String;
    fn summarsize(&self) -> String {
        format!("(Read more fomr {}...)", self.summarsize_author())
    }
}

impl Summary3 for Tweet {
    fn summarsize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

#[test]
fn summary3_test() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarsize());

    print!("tweet: {:#?}, notify: ", tweet);
    notify(tweet);
}

// trait 做为默认参数

pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summary());
}

// Trait Bound
pub fn notify_with_out_trait_bound(item1: impl Summary, item2: impl Summary) {}

pub fn notify_with_trait_bound<T: Summary>(imte1: T, item2: T) {}

pub fn notify_with_out_trait_bound2(item: impl Summary + Display) {}

pub fn notify_with_trait_bound2<T: Summary + Display>(item1: T, item2: T) {}

pub fn notify_with_out_where<T: Display + Clone, U: Clone + Debug>(t: T, u: U) {}

pub fn notify_with_where<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    return 1;
}

// 返回实现了trait的类型
// 不过只能返回单一类型，如果内部需要根据情况返回两种实现就行不通了
// 这是后就需要使用到 为使用不同类型的值而设计的trait对象
fn retures_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

// 使用trait bounds的largest函数
fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

#[test]
fn largest_test() {
    let number_list = vec![11, 22, 55, 44, 66, 0];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['a', 'c', 'y', 'A', 'Z', 'z'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// 使用 trait bound 有条件地实现方法
pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest memberis y = {}", self.y);
        }
    }
}

// 也可以对任何实现了特定trait的类型有条件低地实现trait
// 对任何满足特定trait bound 的类型实现trait被称为blanket implementations
// 它们被广泛运用于rust标准库，例如，标准库为任何实习了 Display trait 的类型实现了 ToString trait,
// 这个impl块看起来像这样
/*
 * impl<T: Display> ToString for T {
 *      // --snip--
 * }
 */
//因为标准库有了这些 blanket implementation 我们可以对任何实现了Display trait
//的类型调用由ToString定义的 to_string
//方法，例如我们可以将整型转换为对应的String值，因为整型实现了Display
#[test]
fn blanket_implementation_test() {
    let s = 3.to_string();
}
