# javascriptcore [![Build Status](https://travis-ci.org/gtk-rs/javascriptcore-rs.png?branch=master)](https://travis-ci.org/gtk-rs/javascriptcore-rs) [![Gitter](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/gtk-rs/gtk)

[Project site](http://gtk-rs.org/) | [Online documentation](http://gtk-rs.org/docs/)

__Rust__ bindings and wrappers for __javascriptcore__.

## Using

We recommend using [crates from crates.io](https://crates.io/keywords/gtk-rs),
as [demonstrated here](http://gtk-rs.org/#using).

If you want to track the bleeding edge, use the git dependency instead:

```toml
[dependencies]
javascriptcore-rs = { git = "https://github.com/gtk-rs/javascriptcore-rs.git" }
```

Avoid mixing versioned and git crates like this:

```toml
# This will not compile
[dependencies]
gtk = "0.2"
javascriptcore-rs = { git = "https://github.com/gtk-rs/javascriptcore-rs.git" }
```

## Contribute

Contributor you're welcome!

## License

__javascriptcore-rs__ is available under the MIT License, please refer to it.
