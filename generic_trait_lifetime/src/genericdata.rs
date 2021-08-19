#[test]
fn generic_data() {
    let number_list = vec![34, 50, 11, 100, 65];

    let largest_number = largest_i32(&number_list);
    println!("The largest number is {}", largest_number);

    let char_list = vec!['y', 'a', 'm', 'z'];

    let largest_char = largest_cahr(&char_list);
    println!("The largest char is {}", largest_char);

    // generic function
    let largest_t = largest(&number_list);
    println!("The largest number is {}", largest_t);

    let largest_t = largest(&char_list);
    println!("The largest char is {}", largest_t);

    // generic struct
    let integer_point_one = PointOne { x: 5, y: 10 };
    println!("integer point: {:?}", integer_point_one);
    let float_point_one = PointOne { x: 1.0, y: 10.0 };
    println!("float point: {:?}", float_point_one);

    let point_two1 = PointeTwo { x: 5, y: 1.0 };
    println!("point two: {:?}", point_two1);
    let point_two2 = PointeTwo { x: "aaaa", y: 1 };
    println!("opint two: {:?}", point_two2);

    // generic method
    let integer_point_one_x = integer_point_one.x();
    println!("integer_point_one x: {}", integer_point_one_x);
    // 指定具体类型的方法
    let distance_float_point_one = float_point_one.distance_from_origin();
    println!(
        "{:?} distance (0.0) : {}",
        float_point_one, distance_float_point_one
    );
    // 结构体中定义的泛型参数并不总是与结构体方法签名中使用的泛型是同一类型
    let p1 = PointeTwo { x: 5, y: 10.1 };
    let p2 = PointeTwo { x: "Hello", y: 'r' };
    let mixup_point = p2.mixup(p1);
    println!("mixup point from p1, p2 -> {:?}", mixup_point)
}

#[allow(unused)]
fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }

    return largest;
}

#[allow(unused)]
fn largest_cahr(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

#[allow(unused)]
// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
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

#[allow(unused)]
#[derive(Debug)]
struct PointOne<T> {
    x: T,
    y: T,
}

#[allow(unused)]
impl PointOne<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2)) + self.y.powi(2).sqrt()
    }
}

#[allow(unused)]
#[derive(Debug)]
struct PointeTwo<T, U> {
    x: T,
    y: U,
}

#[allow(unused)]
impl<T> PointOne<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

#[allow(unused)]
impl<T, U> PointeTwo<T, U> {
    fn mixup<V, W>(self, other: PointeTwo<V, W>) -> PointeTwo<T, W> {
        PointeTwo {
            x: self.x,
            y: other.y,
        }
    }
}
