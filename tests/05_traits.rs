#[test]
fn test_print_point() {
    let pt = Point {x: 5, y: 15};

    pt.print();
}

trait Print {
    fn print(&self);
}

impl<T: std::fmt::Display> Print for T {
    fn print(&self) {
        println!("{}", self);
    }
}

struct Point {
    x: i32,
    y: i32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
