struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn add(&self, x: &str) -> i32 {
        let x: i32 = match x.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("{:?}", "Boops"),
        };
        return (self.x) + x;
    }
}




fn main() {
    let y = &5; // this is the same as `let _y = 5; let y = &_y;`
    let f = Foo { x: y };

    println!("x is: {}", f.add("1"));
}
