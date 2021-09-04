# Fungi

A rewrite of the [mold linker](https://github.com/rui314/mold) in rust.

## Platform support

None!
No really, this is 0% complete.

## Setup

### 1
Install lld e.g. `pacman -Syu lld` or `apt-get install lld`

### 2
Create `~/.cargo/config` containing:
```toml
[build]
[target.x86_64-unknown-linux-gnu]
rustflags = [
  "-C", "link-arg=-fuse-ld=lld",
]
```
Rust will now use lld as its linker!

### 3
Run `./install.sh`
This will replace lld with fungi.

We need to do this because linkers are kinda cursed and the linker is always taken from an unconfigurable absolute path.

### 4
When you want your linker to work again, run `./uninstall.sh` to restore lld to its original state.
lld is pretty nice so you can leave the `.cargo/config` as is.

## Goals

*   Learn about and document how linkers work - I have no idea what I'm getting myself into here 🙃
    +   turns out mold is already pretty well documented! Not sure I can add much here.
*   Why is mold so fast? Can rust work as well as c++ here?
    +   From a quick look mold uses a lot of mmap which is supposedly not fun to use in rust
    +   But it also seems to get a lot of speed from parallelization which rust is good at. rayon 😍
*   Only need to support linking rust programs. Maybe there are some optimizations we can take that benefit rust? ¯\\\_(ツ)\_/¯

## License

I don't really understanding how licensing works in these cases but I assume I need to maintain the same license as mold because I'm referencing it so closely.
Mold itself is licensed under AGPL but will be relicensed to MIT if someone sponsors Rui the main dev.

So I guess that is fungi's fate until then.
