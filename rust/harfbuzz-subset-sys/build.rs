//! Compiles the HarfBuzz subsetter amalgamation from the repository's `src/`
//! (build scripts run in the package directory, `rust/harfbuzz-subset-sys`).

fn main() {
    let mut build = cc::Build::new();
    // HarfBuzz builds as C++11 without exceptions, RTTI or thread-safe
    // statics (its meson.build); with these it needs no libstdc++.
    build
        .cpp(true)
        .std("c++11")
        .warnings(false)
        .flag_if_supported("-fno-exceptions")
        .flag_if_supported("-fno-rtti")
        .flag_if_supported("-fno-threadsafe-statics")
        .flag_if_supported("-fvisibility-inlines-hidden")
        .file("../../src/harfbuzz-subset.cc");
    if std::env::var("CARGO_CFG_TARGET_FAMILY").is_ok_and(|f| f.split(',').any(|f| f == "unix")) {
        // pthread mutexes rather than std::mutex, as meson configures it.
        build.define("HAVE_PTHREAD", "1");
    }
    if std::env::var("CARGO_CFG_TARGET_ENV").is_ok_and(|env| env == "msvc") {
        // The amalgamation has more sections than an MSVC object holds by default (C1128); meson.build adds the
        // same flag.
        build.flag("/bigobj");
    }
    build.compile("harfbuzz-subset");
    println!("cargo:rerun-if-changed=../../src");
    println!("cargo:rerun-if-changed=build.rs");
}
