fn main(){
    cc::Build::new()
        .file("src/mkeypress.c")
        .compile("mkeypress.a");        
    println!("cargo:rustc-link-lib=dylib=mkeypress.a"); // works like "rustc -l doubler.o"}
    println!("cargo:rustc-link-lib=framework=Carbon");
}