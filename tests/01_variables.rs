const PI_GLOBAL: f32 = 3.14159;

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

#[test]
fn test_variable() {
    let f = 3.14;
    assert_eq!(
        format!("The value of f is: {}", f),
        "The value of f is: 3.14"
    );

    // rust infers the type of x
    let x = 13;
    println!("{}", x);

    // rust can also be explicit about the type
    let x: f64 = 3.14159;
    println!("{}", x);

    // rust can also declare and initialize later, but this is rarely done
    let x;
    x = 0;
    println!("{}", x);

    // change variable
    let mut x = 42;
    println!("{}", x);
    x = 13;
    println!("{}", x);

    // array
    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", nums[1]);

    println!("PI_GLOBAL is {}", PI_GLOBAL);
    let sum = add(1, 2);
    assert_eq!(sum, 3);
}

#[test]
fn test_string() {
    // Using a static method to create an instance of String
    let s = String::from("Hello world!");
    // Using a method on the instance
    println!("{} is {} characters long.", s, s.len());
}
