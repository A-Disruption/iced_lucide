pub fn main() {
    println!("cargo::rerun-if-changed=fonts/my-icons.toml");
    iced_lucide::build("fonts/my-icons.toml").expect("Build icon module");
}
