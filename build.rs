extern crate cmake;
extern crate cc;
use cmake::Config;

fn main()
{
    let dst = Config::new("libtest").build();
    //let dst_sdl = Config::new("libtest/SDL").build();

    println!("cargo:rustc-link-search=native={}", dst.display().to_string());
    println!("cargo:rustc-link-lib=static=test");
    
    //println!("cargo:rustc-link-search=native={}", dst_sdl.display().to_string());
    //println!("cargo:rustc-link-lib=static=SDL2");
}
