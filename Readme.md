# asc

The compiler for the ASM++ language

## Example

Simple example how to create a simple programm in ASM++:

```ASM++
main {
    eax = 5
    ret
}
```

You can then compile and link it:

```bash
asc test.a++ -o test.o
g++ test.o -o test.out
```

Then you can run it as followed:

```bash
./a.out
```

## Building

First you need to clone the reposentrory:

```bash
git clone https://github.com/Toni-Graphics/asc.git
```

Then you can compile it (you need to have rust installed):

```bash
cargo build --release
```

You can then copy from `target/release/asc.exe` to your installation directory

## Documentation

ToDo
