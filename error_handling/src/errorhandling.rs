#[test]
fn panic_test1() {
    panic!("crash and burn");
}

#[test]
fn panic_out_of_bound() {
    let v = vec![1, 2, 3];

    v[99];
}
