#[test]
fn test() {
    assert_eq!(2 + 3, 5);
}

#[test]
fn test1() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}