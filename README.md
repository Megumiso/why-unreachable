# why-unreachable

This code shows unreachable at Line 9 `.expect` on compiler. But it runs in "unreachable" line and exits with status code 1.

Possibly, it doesn't require Reqwest at all, but I found this while using reqwest.

# Versions

rustc: `rustc --version` rustc 1.65.0 (897e37553 2022-11-02)
cargo: `cargo --versiob` cargo 1.65.0 (4bc8f24d3 2022-10-20)
