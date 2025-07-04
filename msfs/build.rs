fn main() {
    let wasm = std::env::var("TARGET").unwrap().starts_with("wasm32-");
    let msfs_sdk = msfs_sdk::calculate_msfs_sdk_path().unwrap();
    println!("Found MSFS SDK: {msfs_sdk:?}");

    // build nanovg wrapper
    if wasm {
        std::env::set_var("AR", "llvm-ar");
        cc::Build::new()
            .compiler("clang")
            .flag(format!("--sysroot={msfs_sdk}/WASM/wasi-sysroot"))
            .flag("-fms-extensions") // intended to be used with msvc
            .flag("-D__INTELLISENSE__") // get rid of incorrect __attribute__'s from asobo
            .flag("-Wno-unused-parameter") // warning in nanovg
            .flag("-Wno-sign-compare") // warning in nanovg
            .flag("-mthread-model") // no thread support
            .flag("single") // no thread support
            .include(format!("{msfs_sdk}/WASM/include"))
            .file(format!("{msfs_sdk}/WASM/src/MSFS/Render/nanovg.cpp"))
            .compile("nanovg");
    }

    // build bindings
    {
        println!("cargo:rerun-if-changed=src/bindgen_support/wrapper.h");
        let mut bindings = bindgen::Builder::default()
            .clang_arg(format!("-I{msfs_sdk}/WASM/include"))
            .clang_arg(format!("-I{msfs_sdk}/SimConnect SDK/include"))
            .clang_arg(format!("-I{}", "src/bindgen_support"))
            .clang_arg("-fms-extensions")
            .clang_arg("-fvisibility=default")
            .clang_arg("-xc++")
            .clang_arg("-std=c++17")
            .clang_arg("-v")
            .header("src/bindgen_support/wrapper.h")
            .blocklist_function("nvgFillColor")
            .blocklist_function("nvgFillPaint")
            .blocklist_function("nvgStrokeColor")
            .blocklist_function("nvgStrokePaint")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
            .rustified_enum("SIMCONNECT_EXCEPTION")
            .impl_debug(false);

        if wasm {
            bindings = bindings.clang_arg("-D_MSFS_WASM 1");
        }

        bindings
            .generate()
            .unwrap()
            .write_to_file(
                std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("msfs-sys.rs"),
            )
            .unwrap();
    }

    // SimConnect native linkage
    if !wasm {
        println!("cargo:rustc-link-search={msfs_sdk}/SimConnect SDK/lib/static");
        println!("cargo:rustc-link-lib=SimConnect");
        println!("cargo:rustc-link-lib=shlwapi");
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=ws2_32");
        println!("cargo:rustc-link-lib=shell32");
    }
}
