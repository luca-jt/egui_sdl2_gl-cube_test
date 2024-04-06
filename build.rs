extern crate cmake;
use cmake::Config;

fn main()
{
    let dst = Config::new("libtest").build();

    println!("cargo:rustc-link-search=native={}", dst.display().to_string());
    println!("cargo:rustc-link-lib=static=test");
}
