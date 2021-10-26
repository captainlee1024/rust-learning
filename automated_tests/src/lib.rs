#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    // 忽略某些测试
    #[test]
    #[ignore]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    // 使用assert_eq!和assert_ne!宏来测试相等
    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
        assert_ne!(5, add_two(2));
    }

    // 自定义失败信息
    /*
     * 这个程序的需求还没有被确定，因此问候文本开头的 Hello 文本很可能会改变。
     * 然而我们并不想在需求改变时不得不更新测试，所以相比检查 greeting 函数返回的确切值，
     * 我们将仅仅断言输出的文本中包含输入参数。
     */
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // assert!(result.contains("xxx"));
        // 此时出错只会告诉我行号，我们想要一些失败的具体信息可以这样
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    // 使用should_panic 检查 panic
    /*
     * 然而 should_panic 测试结果可能会非常含糊不清，因为它只是告诉我们代码并没有产生 panic。
     * should_panic 甚至在一些不是我们期望的原因而导致 panic 时也会通过。
     * 为了使 should_panic 测试结果更精确，我们可以给 should_panic
     * 属性增加一个可选的 expected 参数。测试工具会确保错误信息中包含其提供的文本。
     */
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    // 将Result<T, E> 用于测试
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
}

// 使用assert!宏来检查结果
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(unused)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// 使用assert_eq!和assert_ne!宏来测试相等
/*
 * assert_eq! 和 assert_ne! 宏在底层分别使用了 == 和 !=。
 * 当断言失败时，这些宏会使用调试格式打印出其参数，
 * 这意味着被比较的值必需实现了 PartialEq 和 Debug trait。
 * 所有的基本类型和大部分标准库类型都实现了这些 trait。
 * 对于自定义的结构体和枚举，需要实现 PartialEq 才能断言他们的值是否相等。
 * 需要实现 Debug 才能在断言失败时打印他们的值。
 * 因为这两个 trait 都是派生 trait，如第五章示例 5-12 所提到的，
 * 通常可以直接在结构体或枚举上添加 #[derive(PartialEq, Debug)] 注解。
 * 附录 C “可派生 trait” 中有更多关于这些和其他派生 trait 的详细信息。
 */
#[allow(unused)]
pub fn add_two(a: i32) -> i32 {
    a + 2
}

// 自定义失败信息
#[allow(unused)]
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

// 使用should_panic 检查 panic
#[allow(unused)]
pub struct Guess {
    value: i32,
}

#[allow(unused)]
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            )
        }

        Guess { value }
    }
}

// cargo test -- --test-threads=1 执行线程数
// cargo test -- --nocapture 显示函数输出
#[allow(unused)]
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);
    10
}

// cargo test one_hundred 执行任意名称运行单个测试函数
// cargo test add 任何包含add的测试都会执行
// 忽略某些测试
