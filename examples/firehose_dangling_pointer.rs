fn main() {
    // let my_int_ptr: &i32;
    // {
    //     let my_int: i32 = 5;
    //     my_int_ptr = &my_int;
    // }
    // dbg!(*my_int_ptr);
    let mut my_int = 5;
    let reference1 = &mut my_int;
    *reference1 += 1;
    let reference2 = &mut my_int;
    *reference2 += 1;
    assert_eq!(my_int, 7);
}
