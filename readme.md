# `-sys` crate metadata example

This workspace explores Cargo's `DEP_NAME_KEY=value` feature. The goal is to understand how multiple `-sys` crates can be used in a project where the C source of one `-sys` crate (`bar`) depends on another `-sys` crate's C source (`foo`).

- `foo` builds and links `libfoo`
- `bar` builds and links `libbar` which depends on `libfoo`
- `baz` depends on `bar`
