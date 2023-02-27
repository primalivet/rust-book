# Minigrep

Code from tutorial at https://doc.rust-lang.org/book/ch12-00-an-io-project.html

## Commands

```
cargo run -- <pattern> <filepath> 
```

## Settings

Use `MINIGREP_IGNORE_CASE` to toggle case insensitive behaviour. Note that at the time this variable can be set to `false` or `0` but will still be truthy as we just check if it's set at all or not

```
MINIGREP_IGNORE_CASE=1 cargo run -- <pattern> <filepath>
```

## Redirection

Valid unix redirection by printing to `stdout`

```
cargo run -- <pattern> <filepath> > <outputfile>
```
Valid unix redirection by printing to `stderr`

```
cargo run -- <pattern> <filepath> 2> <outputfile>
```
