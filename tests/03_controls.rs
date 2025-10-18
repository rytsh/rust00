#[test]
fn test_loop() {
    let mut count = 0;
    loop {
        count += 1;
        if count == 5 {
            break;
        }
    }
    assert_eq!(count, 5);
}

#[test]
fn test_while() {
    let mut x = 0;
    while x != 42 {
        x += 1;
    }
    assert_eq!(x, 42);
}

#[test]
fn test_for() {
    for x in 0..5 {
        print!("{}", x);
    }

    println!();

    for x in 0..=5 { // inclusive range
        print!("{}", x);
    }

    println!();
}

#[test]
fn test_break_value() {
    let mut x = 0;
    let v = loop {
        x += 1;
        if x == 10 {
            break x * 2;
        }
    };
    assert_eq!(v, 20);
}

#[test]
fn test_match() {
    let x = 42;

    match x {
        0 => {
            println!("found zero");
        }
        // we can match against multiple values
        1 | 2 => {
            println!("found 1 or 2!");
        }
        // we can match against ranges
        3..=9 => {
            println!("found a number 3 to 9 inclusively");
        }
        // we can bind the matched number to a variable
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100!", matched_num);
        }
        // this is the default match that must exist if not all cases are handled
        _ => {
            println!("found something else!");
        }
    }
}
