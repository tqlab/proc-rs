#[cfg(feature = "bindgen")]
extern crate bindgen;

extern crate cc;
extern crate pkg_config;
use std::env;
use std::path::PathBuf;

fn main() {
    #[cfg(feature = "bindgen")]
    generate();

    let mut proc = PathBuf::from(&env::var("CARGO_MANIFEST_DIR").unwrap());
    proc.push("proc-c");

    if pkg_config::find_library("libproc-monitor").is_err() {
        let mut builder = cc::Build::new();

        builder
            .file(proc.join("proc_info.c"))
            .flag_if_supported("-Wno-unused-parameter")
            .flag_if_supported("-Wbad-function-cast")
            .flag_if_supported("-Wuninitialized");
        if env::var("CARGO_FEATURE_WITH_ASAN").is_ok() {
            builder.flag("-fsanitize=address");
        }

        if env::var("CARGO_FEATURE_WITH_FUZZER").is_ok() {
            builder.flag("-fsanitize=fuzzer");
        } else if env::var("CARGO_FEATURE_WITH_FUZZER_NO_LINK").is_ok() {
            builder.flag("-fsanitize=fuzzer-no-link");
        }

        builder.compile("libproc-monitor.a");
    }
}

#[cfg(feature = "bindgen")]
fn generate() {
    println!("cargo::rustc-link-lib=dylib=proc-monitor");

    let bindings = bindgen::Builder::default()
        .header("proc-c/proc_info.h")
        .allowlist_var("^(proc)_.*")
        .allowlist_type("^(proc)_.*")
        .allowlist_function("^(get_proc_info).*")
        .allowlist_function("^(free_memory).*")
        .size_t_is_usize(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings.");

    let out_path = PathBuf::from("src/");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings.");
}
