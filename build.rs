extern crate cmake;
extern crate cc;


fn main()
{
    let dst = cmake::Config::new("libtest")
        .very_verbose(true)
        .target("all")
        .build();

    println!("cargo:rustc-link-search=native={}", dst.display().to_string());
    println!("cargo:rustc-link-lib=static=test");
}
