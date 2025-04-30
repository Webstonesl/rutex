fn main() {
    if std::env::var("GOOGLE_API_KEY").is_ok() {
        println!("cargo::rustc-cfg=GOOGLE_API_KEY")
    }
}
