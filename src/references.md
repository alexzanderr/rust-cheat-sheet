
# References - The Rust programming language
##  Books
- [`rust by example`](https://doc.rust-lang.org/stable/rust-by-example/index.htmls)
- [`list with docs`](https://doc.rust-lang.org/stable/)
- [`performance book`](https://nnethercote.github.io/perf-book/title-page.html)
- [`list with books`](https://lborb.github.io/book/title-page.html)
- [`cook book`](https://rust-lang-nursery.github.io/rust-cookbook/)


## rust-analyzer LSP
- https://stackoverflow.com/a/65223815/12172291

get rust analyzer nightly from here:
- https://github.com/rust-analyzer/rust-analyzer/releases/download/nightly/rust-analyzer-x86_64-unknown-linux-gnu.gz


# more docs
list:
- https://williamhuey.github.io/posts/rust-serde-iterating-over-json-keys/
- https://blog.logrocket.com/interacting-with-assembly-in-rust/
- https://stackoverflow.com/questions/30414424/how-can-i-update-a-value-in-a-mutable-hashmap
- https://stackoverflow.com/questions/59150327/generate-markdown-docs-with-rustdoc
- https://stackoverflow.com/questions/28126735/is-there-a-way-to-perform-an-index-access-to-an-instance-of-a-struct#28126836
- https://github.com/hcpl/crates.io-http-api-reference
- https://docs.rs/crates_io_api/latest/crates_io_api/
- https://cheats.rs/
- https://rust-lang-nursery.github.io/rust-cookbook/web/clients/requests.html

- https://rust-unofficial.github.io/too-many-lists/fourth-building.html

- https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html

- https://stackoverflow.com/questions/4279611/how-to-embed-a-video-into-github-readme-md/4279746#4279746
- https://github.com/rust-lang/mdBook/wiki/Automated-Deployment%3A-GitHub-Pages
- https://rust-lang.github.io/mdBook
- http://ilkinulas.github.io/programming/2016/02/05/sublime-text-syntax-highlighting.html

# put all the packages into the current project

```shell
export CARGO_HOME=$PWD/.cargo
```
note that this env variable will be only for current shell

very good for docker container

and when you do
```shell
cargo fetch
```

all the extern crates will be downloaded into the current project


# rust with docker
https://windsoilder.github.io/writing_dockerfile_in_rust_project.html

# sccache - Shared Compilation Cache
https://github.com/mozilla/sccache/


# your own OS with rust
https://os.phil-opp.com/

### for sublime text
replace the rust analyzer from `~/.cache/sublime-text/Package Storage/LSP-rust-analyzer` with the nightly version

and you are good to go, no more macro proc errors


## rustup toolchain

install
```shell
sudo pacman -S rustup
```

install nightly rust compiler
```shell
rustup install nightly
```

and you are ready to go


## extend cargo with more commands

install the `cargo-edit` crate
```shell
cargo install cargo-edit
```

and you are ready to use
```shell
cargo add crate
```


# different names between crate name and library name
https://stackoverflow.com/questions/44769922/how-to-import-a-crate-dependency-when-the-library-name-is-different-from-the-pac#44771221


# read documentation for crate from markdown file
https://stackoverflow.com/questions/48980028/how-is-it-possible-to-keep-rust-module-documentation-in-separate-markdown-files


# async rust
- https://users.rust-lang.org/t/how-to-implement-async-await-in-main/38007/6
- https://blog.logrocket.com/a-practical-guide-to-async-in-rust/

# 4 web assembly
```shell
cargo install cargo-web
```

# errors
https://learning-rust.github.io/docs/e7.custom_error_types.html

# rust formatter

if you are using rust analyzer with rustfmt inside sublime

you can customize the global settings for `rustfmt` in:
- `~/.config/rustfmt/rustfmt.toml`


# install excvr and excvr_jupyter to run rust inside jupyter lab or jupyer notebook

installation
```shell
cargo install evcxr_jupyter
evcxr_jupyter --install
```

note: you need jupyter lab first

```shell
pip install jupyterlab
```

run with custom folder
```shell
jupyter-lab --notebook-dir=path to dir
```

install one dark theme
```shell
jupyter labextension install jupyterlab_onedarkpro
```

done. happy coding.


## cargo-edit package error with the segfault
https://github.com/killercup/cargo-edit/issues/641

running this command will fix the issue
```shell
https://github.com/killercup/cargo-edit/issues/641
```


# cargo watch
- https://crates.io/crates/cargo-watch
- https://stackoverflow.com/questions/29461693/how-can-i-get-cargo-to-recompile-changed-files-automatically