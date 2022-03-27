
# cargo-edit

## install cargo-edit
```shell
cargo install cargo-edit
```
this command will install the crate into the `$HOME/.cargo`

on linux is: `/home/username/.cargo`

the binaries from the crate, which are `add`, `rm`, etc.., are installed into the folder: `$HOME/.cargo/bin`


## cargo add

### add feature from command line

```shell
if you want for example to add the feature `derive` from the crate `clap`, run this:
cargo add clap --features derive
```

### what if cargo add gives `segmentation fault`

what to do?

```shell
git clone https://github.com/killercup/cargo-edit
cd cargo-edit
cargo install --path .
```
this will install globally cargo-edit from this current folder from where you cloned the repo

cause: the package has a bug on `crates.io`, its not updated yet, but on github master branch its merged and fixed
