fn foo() -> i32 { 5 }
fn bar() -> () { () }
fn baz() -> () { 5; }

fn main() {
    let _x: i16 = 55;
    let mut _y: i16 = 64;
    let _x = "shadowed!";
    let (_a, _b) = (5, 6);

    _y += 1;

    println!("Hello, world!");
    println!("foo {} ", foo());

    // slices
    let _arr = [0, 1, 2, 3, 4, 5];
    let _full_slice = &_arr;
    let _full_slice = &_arr[..];
    let _part_slice = &_arr[2..5];

    // strings and slices
    let s1: &str = "galaxy";
    let s2 = "galaxy";
    let s3 = s1.to_string();
    let s4: &str = &s3;
    println!("{} {} {} {}", s1, s2, s3, s4);


    // casting
    let x: i32 = 100;
    let _y: u32 = x as u32;


    // tuple
    let _foo: (i32, char, f64) = (72, 'H', 5.1);
    let (_x, _y, _z) = _foo;
    let _foo = (72, 'H', 5.1);

    // vector
    let v0: Vec<i32> = Vec::new();

    // need mutable to push
    let mut v1: Vec<i32>  = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(2);

    // main way to create a vector
    let v1 = vec![1, 2, 3];
    let mut v1 = vec![1, 2, 3];
    v1.push(5);
    v1[2] = 55;
    println!("{}", v1[2]);


    let x = -5;
    let y = if x > 0 { "greater" } else { "less" };
    println!("x = {} is {} than zero", x, y);
}
