# Toy Transpiler

I made this transpiler while watching the
Destroy All Software screencast [A Compiler From Scratch](https://www.destroyallsoftware.com/screencasts/catalog/a-compiler-from-scratch).

Instead of following along in Ruby, I decided
to follow along in Rust. Due to the nature of 
rust, This has lead to a few significantly 
different design decisions, but overall it 
works the same way.

## Todo

- Make `token_kinds` STATIC?
- Better way to do sub-types of Token?

## Development

### Run

```bash
cargo run
```

### Test 

```bash
cargo test
```

A helpful command in vim:

```
:nmap <leader>t :w \| !clear && RUST_BACKTRACE=1 ncargo test<CR>
```

Then type `,t` to run tests from inside vim.

### Format Code

```bash
cargo fmt
```

### Linter
```bash
cargo clippy
```


