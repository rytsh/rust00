/// Swap two integers and return them as a tuple
/// 
/// # Examples
/// 
/// ```
/// let result = swap(1, 2);
/// assert_eq!(result, (2, 1));
/// ```
fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

// fn make_nothing() -> () {
//     return ();
// }

#[test]
fn test_swap() {
    // return a tuple of return values
    let result = swap(123, 321);
    assert_eq!(result, (321, 123));

    // destructure the tuple into two variables names
    let (a, b) = swap(result.0, result.1);
    assert_eq!((a, b), (123, 321));
}
