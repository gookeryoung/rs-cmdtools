fn main() {
    let mut s = String::from("Test");
    s.push_str(", new idea.");
    println!("Hello, file renamer, {s}!");

    let s1 = String::from("hello");
    let s2 = &s1;
    let s3 = s1.clone();
    println!("s1 = {}, s2 = {}, s3 = {}", s1, s2, s3);

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    change_value();
}

fn change_value() {
    let mut s = String::from("hello");
    push_str(&mut s);
}

fn push_str(str: &mut String) {
    str.push_str(", world!");
    println!("{}", str);
}