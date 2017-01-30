An example crate for using [elm](http://elm-lang.org/) together with a backend written in [rust](https://www.rust-lang.org/) using the [iron web framework](http://ironframework.io/).

This includes a `build.rs` script which builds the elm source during `cargo build` and a `main.rs` that handles statically serving the generated HTML and routing backend requests.

Simply `cargo run` it and point your browser to http://localhost:8080

**Disclaimer:** I'm still learning both rust and elm, so I'll gladly take any advice on how to do stuff better.
