struct Truth;
impl Truth {
    fn reveal() -> String {
        "Knowledge is the bridge to the future.".to_string()
    }
}
fn main() {
    println!("{}", Truth::reveal());
}