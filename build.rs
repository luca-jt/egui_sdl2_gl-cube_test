extern crate cmake;
extern crate cc;

use cmake::Config;


fn main()
{
    let dst = Config::new("libtest")
        .very_verbose(true)
        .target("all")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display().to_string());
    println!("cargo:rustc-link-lib=static=test");
}
