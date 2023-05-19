# corrosion-hello

An example of how to use Rust in a CMake project using [corrosion-rs/corrosion](https://github.com/corrosion-rs/corrosion) and [mozilla/cbindgen](https://github.com/mozilla/cbindgen)

## Usage

```bash
$ git clone https://github.com/ethanavatar/corrosion-hello.git
$ cd corrosion-hello
$ mkdir build
$ cmake -S . -B build
$ cmake --build build
```

```bash
$ ./build/Debug/hello_c.exe
Hello from Rust!
```