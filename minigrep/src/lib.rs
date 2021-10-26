use std::{env, error::Error, fs};

/*
fn parse_config(args: &[String]) -> (&str, &str) {
    let quer = &args[1];
    let filename = &args[2];

    (quer, filename)
}
*/

// 组合配置
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

/*
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}
*/

impl Config {
    /*
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
    */

    /*
     * 现在 new 函数返回一个 Result，在成功时带有一个 Config 实例而在出现错误时带有一个 &'static str
     * 回忆一下第十章 “静态生命周期” 中讲到 &'static str 是字符串字面值的类型，也是目前的错误信息。
     */
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        // 处理环境变量
        // 运行：export CASE_INSENSITIVE=1 & cargo run THE poem.txt
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

// 从main提取逻辑
/*
fn run(config: Config) {
    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
*/

// 从run 中返回错误
/*
 * 这里我们进行了三处明显的修改：
 * 第一处：将 run 函数的返回类型变为 Result<(), Box<dyn Error>>。之前这个函数返回 unit 类型 ()，
 * 现在它仍然保持作为 Ok 时的返回值。
 * 对于错误类型，使用了 trait 对象 Box<dyn Error>
 * （在开头使用了 use 语句将 std::error::Error 引入作用域）。第十七章 会涉及 trait 对象。
 * 目前只需知道 Box<dyn Error> 意味着函数会返回实现了 Error trait 的类型，
 * 不过无需指定具体将会返回的值的类型。这提供了在不同的错误场景可能有不同类型的错误返回值的灵活性。
 * 这也就是 dyn，它是 “动态的”（“dynamic”）的缩写。
 *
 * 第二处：是去掉了 expect 调用并替换为 第九章 讲到的 ?。
 * 不同于遇到错误就 panic!，? 会从函数中返回错误值并让调用者来处理它。
 *
 * 第三处：现在成功时这个函数会返回一个 Ok 值。因为 run 函数签名中声明成功类型返回值是 ()，
 * 这意味着需要将 unit 类型值包装进 Ok 值中。Ok(()) 一开始看起来有点奇怪，
 * 不过这样使用 () 是惯用的做法，表明调用 run 函数只是为了它的副作用；
 * 函数并没有返回什么有意义的值。
 */
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    /*
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    */

    Ok(())
}

// 采用测试驱动开发完善search功能 (TDD)

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // 遍历内容的每一行文本。
    // 查看这一行是否包含要搜索的字符串。
    // 如果有，将这一行加入列表返回值中。
    // 如果没有，什么也不做。
    // 返回匹配到的结果列表
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

// 处理环境变量

pub fn search_case_insensitive<'a>(query: &'a str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
