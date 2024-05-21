<div align="center">

  <h1><code>Rust tracing jump start</code></h1>
  <strong>
      Simple jump start project to get hands on the tracing crate
  </strong>

</div>

I want to prevent people to waste time with searching for good examples in the documentation.
I had to scan multiple sources and issues to find some of the features used here so hope it helps someone.

How to find out the minimal version for rust to run this project?
==
There is pretty simple tool for that:
* `cargo install cargo-msrv`
after success run 
* `cargo msrv list`
or to make things more complicated
* `cargo msrv`

Why it that important?
===
Well, since rust cargo works kind of different like other package managers you
have to use the "=" prefix to pin a version for the cargo.toml.
For example:
```toml
[dependencies]
tracing = "=0.1.40"
```
Most developers do not use this feature, so it could be that some of the dependencies change over time
while you use this package. 

That is the reason I can not recommend you a specific minimal version for now, because it can change tomorrow.

In my case however the minimal version was: `1.67.0`

But I tested it on: `rustc 1.79.0-nightly (dbce3b43b 2024-04-20)`