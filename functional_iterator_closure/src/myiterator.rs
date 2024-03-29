#[test]
fn lazy_iterator() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Go: {}", val);
    }
}

#[test]
fn iter_trait_and_next_fn() {
    /*
     * 迭代器都实现了一个叫做 Iterator 的标准库 trait
     * pub trait Iterator {
     *  type Item;
     *  fn next(&mut self) -> Option<Self::Item>;
     *  // ---snip default impl
     * }
     */

    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn consum_adaptors() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    // iterator sum
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}

#[test]
fn iterator_adaptors() {
    let v1: Vec<i32> = vec![1, 2, 3];

    // v1.iter().map(|x| x + 1);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4])
}

#[derive(PartialEq, Debug)]
pub struct Shoe {
    pub size: u32,
    pub style: String,
}

pub fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[test]
fn filters_by_size() {
    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ]
    );
}

// 实现Iterator trait 创建自定义迭代器
pub struct Counter {
    count: u32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    // 1, 2, 3, 4, 5
    // 2, 3, 4, 5, None
    // -> 2*2 + 2*3 + 3*4 + 4*5    %3==0
    // -> 2*3 + 3*4
    // -> 18
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
