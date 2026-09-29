load("@rules_rust_pyo3//:defs.bzl", "rust_pyo3_toolchain", "pyo3_toolchain")

package(default_visibility = ["//visibility:public"])

rust_pyo3_toolchain(
    name = "rust_pyo3_toolchain",
    pyo3 = "@crates//:pyo3",
    pyo3_introspection = "@crates//:pyo3-introspection",
)

toolchain(
    name = "rust_toolchain",
    toolchain = ":rust_pyo3_toolchain",
    toolchain_type = "@rules_rust_pyo3//:rust_toolchain_type",
)

pyo3_toolchain(
    name = "pyo3_toolchain",
)

toolchain(
    name = "toolchain",
    toolchain = ":pyo3_toolchain",
    toolchain_type = "@rules_rust_pyo3//:toolchain_type",
)
