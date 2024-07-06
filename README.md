# Move-ish to MASM Compiler

Subset of Move language to Polygon Miden Assembly Compiler

## Resources

### Miden

- [Miden Docs](https://0xpolygonmiden.github.io/miden-vm/)
- [Miden IR Builder API Reference](https://github.com/0xPolygonMiden/compiler/blob/dd07284ed96208e9cc6d30eaf18cb143afa1b9af/frontend-wasm/src/code_translator/mod.rs)

### Move

- [Move Book](https://move-book.com/index.html)

## Move Language

## Miden Assembly

### Code Organization

#### Procedures

A procedure can be used to encapsulate a frequently-used sequence of instructions which can later be invoked via a label.

These can be declared and envoked

#### Modules

A module contains one or more procedures. There are library modules and executable modules

##### Declaration

###### Library Modules

- Contain zero or more internal procedures (defined with `proc`)
- One or more internal procedures

###### Executable Modules (Programs)

Executable modules are used to define programs

- Contain zero or more internal procedures (defined with `proc`)
- Exactly one `main` procedure
- Declared with `begin` and `end`

##### Invocation

- Invoked with cimilsr syntax to Rust

#### Constants

Constants must be declared right after module imports and before procedures or program bodies

#### Comments

```txt
#! This is a comment
```

#### Execution Contexts

They are used to control the execution context of a program

### Control Flow

#### If-Else

Similar to execution of high level for if else statements

#### Repeat (For Loop)

Similar to execution of high level for loops

#### While Loop

Similar to execution of high level while loops

### Field Operations

Understand this ...

### u32 operations

Understand this ...

### Stack manipulation

Understand this ...

## Ideas

1. My initial instinct was to do a toy language to MASM
2. My second instinct was to do Move Bytecode -> Miden via their SSA API
3. The above task was somewhat beyond the scope of my current class
4. What I'd like to end up doing now is to make a basic Move Bytecode to MASM transpiler
5. I should really be using their internal tooling if I want to make this production ready,
   but I don't have the time or the bandwidth to do that
6. I think something that would be a lot of fun is for me to write an actual Move to Miden compiler.
   in this case, maybe I can get hired by Mysten labs or Polygon lol

## Todo

- [ ] Import each Move crate into my project
- [ ] Modify the necessary files to complete the translation

## Notes

- For Move -> Miden compilation, we use the stack machine representation of the bytecode
  because Miden VM is a stack machine
- We are NOT doing the

## Important

- move-binary-format/src/file_format.rs

### Move Addresses

Every Sui Move address has a separate address on the blockchain. It's a global shared state that you have access to
