fn bindgen_demo(input: &str, output: &str) {
    let out_dir_str = std::env::var_os("OUT_DIR").unwrap();
    let out_dir = std::path::Path::new(&out_dir_str);

    let mut config = bindgen::CodegenConfig::empty();
    config.insert(bindgen::CodegenConfig::FUNCTIONS);

    bindgen::Builder::default()
        .with_codegen_config(config)
        .header(input)
        // https://github.com/rust-lang-nursery/rust-bindgen/issues/550
        .blocklist_type("max_align_t")
        .ctypes_prefix("libc")
        .size_t_is_usize(true)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_dir.join(output))
        .expect("Couldn't write bindings");
}

fn build_c_demo() {
    println!("cargo:rerun-if-changed=libc-demo/");

    cc::Build::new()
        .file("libc-demo/lib.c")
        .compile("libc-demo.a");
}

fn build_cxx_demo() {
    println!("cargo:rerun-if-changed=libcxx-demo/");

    let dst = cmake::build("libcxx-demo");

    println!("cargo:rustc-link-lib=static=cxx-demo");
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/lib32", dst.display());
    println!("cargo:rustc-link-search=native={}/lib64", dst.display());
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    bindgen_demo("libc-demo/lib.h", "c_demo.rs");
    build_c_demo();
    bindgen_demo("libcxx-demo/lib.h", "cxx_demo.rs");
    build_cxx_demo();
}
