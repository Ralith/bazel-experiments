load("@crates//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rs//rs:rust_library.bzl", "rust_library")
load("@rules_rust//extensions/pyo3:defs.bzl", "pyo3_extension")

pyo3_extension(
    name = "demo",
    srcs = ["src/lib.rs"],
    module_name = "_native",
    deps = all_crate_deps(normal = True),
)
