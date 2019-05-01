# Issue: unpack Ruby via `tar` crate

When unpacking `ruby-2.6.0.tar.bz2` with the `bzip2` and `tar` crates, the
following error is produced:

```
Custom {
    kind: Other,
    error: TarError {
        desc: "failed to unpack `/path/to/issue-unpack-ruby-tar/ruby-2.6.0/libexec/bundle`",
        io: Custom {
            kind: Other,
            error: TarError {
                desc: "failed to unpack `ruby-2.6.0/libexec/bundle` into `/path/to/issue-unpack-ruby-tar/ruby-2.6.0/libexec/bundle`",
                io: Os {
                    code: 20,
                    kind: Other,
                    message: "Not a directory",
                },
            },
        },
    },
}
```

However, running the following works without error:

```
tar xjf ruby-2.6.0.tar.bz2
```

## Testing

Simply execute `cargo run` or to test against a specific Ruby version:

```sh
RUBY_VERSION=2.6.2 cargo run
```

Note that the `tar` crate is able to unpack `2.6.2`, but not `2.6.0`.
