# tdjson-rs

To use this crate you must have TDLib installed.
Look [here](https://github.com/agnipau/tdjson-sys/blob/master/README.md) for
more informations.

## Why not [this version](https://github.com/mersinvald/tdjson-rs)?

This library doesn't use expect, it let's you decide how to handle errors.

Additionally it includes a feature to send typed requests and receive typed
responses.

To use this feature add this to your Cargo.toml:

```toml
tdjson =  { git = "https://github.com/agnipau/tdjson-rs", features = ["types"] }
```

Note that it will significantly increase compile time, because there a lot of
proc macros that are required to run to generate the code. This feature is
disabled by default.

#### License

<sup>
Everything outside of <a href="src/lib.rs">src/lib.rs</a>, <a href="src/client/unsafe_.rs">src/client/unsafe_.rs</a> and <a href="src/client/untyped.rs">src/client/untyped.rs</a> is licensed under either of <a
href="LICENSE-APACHE">Apache License, Version 2.0</a> or <a
href="LICENSE-MIT">MIT license</a> at your option. <a href="src/lib.rs">src/lib.rs</a>, <a href="src/client/unsafe_.rs">src/client/unsafe_.rs</a> and <a href="src/client/untyped.rs">src/client/untyped.rs</a> are
licensed under the <a href="src/client/LICENSE-MIT">MIT license</a>.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

