pub fn main() {
    println!("cargo::rerun-if-changed=fonts/my-icons.toml");
    iced_lucide::build_all("fonts/my-icons.toml").expect("Build all icons");
}
