

# test only the private modules and functions

```shell
cargo test -j 8 --lib -- --show-output
```
`--lib` means test only the library tests, meaning that all library tests are private, because they are not integration tests.
