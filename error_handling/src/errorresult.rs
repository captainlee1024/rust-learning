use std::{
    fs::File,
    io,
    io::{Error, ErrorKind, Read},
};

#[test]
fn error1() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };
}

#[test]
fn match_dif_err() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

#[test]
fn unwrap_and_expect() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
    let f = File::open("hello.txt").unwrap();
}

#[test]
fn call_ruff() {
    println!("=== 使用Result<> 和模式匹配传播错误 ===");
    match read_username_from_file() {
        Ok(_) => println!("SUCCESS!"),
        Err(e) => println!("read failed: {:?}", e),
    }

    println!("=== 使用Result<> 和?运算符传播错误 ===");
    match read_username_from_file2() {
        Ok(_) => println!("SUCCESS!"),
        Err(e) => println!("read failed: {:?}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
