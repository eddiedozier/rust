fn main() {
    println!("{}", say_hello("Eddie"));
}

fn say_hello(name:&'static str) -> String {
    let speak = "Hello ".to_owned() + name; 
    return speak;
}