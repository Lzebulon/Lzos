fn main() {
    println!("cargo::rustc-link-arg-bin=lzos=-Tlink_script.ld");
}
