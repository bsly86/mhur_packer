#[cfg(windows)]
fn main() {
    println!("cargo:rustc-link-arg-bin=MyHeroPak=app_icon.res");
}

#[cfg(not(windows))]
fn main() {}
