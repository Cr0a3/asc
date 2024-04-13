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

## Support
### Registers
|64bit |32bit |16bit |8bit  |
|------|------|------|------|
|rax   |eax   |ax    |al    |
|rbx   |ebx   |bx    |bl    |
|rcx   |ecx   |cx    |cl    |
|rdx   |edx   |dx    |dl    |
|rsi   |esi   |si    |sil   |
|rdi   |edi   |di    |dil   |
|rsp   |esp   |sp    |spl   |
|rbp   |ebp   |bp    |bpl   |
|r8    |r8d   |r8w   |  /   |
|r9    |r8d   |r8w   |  /   |
|r10   |r10d  |r10w  |  /   |
|r11   |r11d  |r11w  |  /   |
|r12   |r12d  |r12w  |  /   |
|r13   |r13d  |r13w  |  /   |
|r14   |r14d  |r14w  |  /   |
|r15   |r15d  |r15w  |  /   |

### Instructions
Following Instructions are supported:

| Intel syntax | ASM++ synatx |
|--------------|--------------|
|mov %reg, val | reg = val    |
|add %reg, val | reg + val    |
|sub %reg, val | reg - val    |
|mul %reg, val | reg * val    |
|div %reg, val | reg / val    |