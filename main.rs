
fn main() {
    println!("{}", std::env::current_dir().unwrap().to_str().unwrap().to_string());
}
