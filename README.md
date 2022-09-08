# All Credit goes to the original creator, baskerville
### Sketch program from plato, ported to inkbox os
# Compilation
get this docker image:
```
docker pull ghcr.io/cross-rs/arm-unknown-linux-gnueabihf:edge
```
run distrobox on it:
```
distrobox create --image ghcr.io/cross-rs/arm-unknown-linux-gnueabihf:edge
distrobox enter arm-unknown-linux-gnueabihf-edge
```
copy libraries from libs the toolchain dirs:
```
/x-tools/arm-unknown-linux-gnueabihf/lib/
/x-tools/arm-unknown-linux-gnueabihf/arm-unknown-linux-gnueabihf/lib/
/x-tools/arm-unknown-linux-gnueabihf/arm-unknown-linux-gnueabihf/sysroot/lib/
```
enter root, and copy the PATH variable from the first user, then install rustup:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
add the rust target:
```
rustup target add arm-unknown-linux-gnueabihf
```
and build it:
```
cargo build --target arm-unknown-linux-gnueabihf --release
```
Maybe also needed, add this too `.cargo/config`
```
[target.arm-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
```
Propably a better way to compile all of this:
```
cargo rustc --release --target=arm-unknown-linux-gnueabihf -- -C target-feature=+v7,+vfp3,+a9,+neon
```
Copy everything from needed-files to the same dir as the binary, libs too and thats should be it
