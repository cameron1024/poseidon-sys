# poseidon-sys

An FFI wrapper around <https://github.com/scroll-tech/poseidon-circuit> that compiles on stable Rust without depending on any halo2 crates directly.

This library requires the `LIBPOSEIDON_PATH` env var to be set to the path of a directory containing `libposeidon.a`, which can be downloaded from the release page.
