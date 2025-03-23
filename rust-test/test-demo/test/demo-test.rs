use rust_demo;
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn test_add() {
    // assert_eq!(rust_demo::add(3), 2 + 1)
    assert_eq!(rust_demo.add(3), 2 + 1)

}
