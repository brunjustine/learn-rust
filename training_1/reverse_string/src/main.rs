pub fn reverse(input: &str) -> String {
    let mut res = String::from("");
    for element in input.chars() {
        println!("e : {}",element);
        res.insert(0,element);
    }
    res
}

fn main() {
    let res = reverse("hello");
    println!("Hello, world! {}",res);
}
