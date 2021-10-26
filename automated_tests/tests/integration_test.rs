use automated_tests;
mod common;

/*
 * 与单元测试不同，我们需要在文件顶部添加 use adder。
 * 这是因为每一个 tests 目录中的测试文件都是完全独立的 crate，所以需要在每一个文件中导入库。
 *
 * 并不需要将 tests/integration_test.rs 中的任何代码标注为 #[cfg(test)]。
 * tests 文件夹在 Cargo 中是一个特殊的文件夹， Cargo 只会在运行 cargo test 时编译这个目录中的文件
 */

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, automated_tests::add_two(2));
}
