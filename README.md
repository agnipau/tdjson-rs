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
tdjson =  { git = "https://github.com/agnipau/tdjson_rs", features = ["types"] }
```

Note that it will significantly increase compile time, because there a lot of
proc macros that are required to run to generate the code. This feature is
disabled by default.
