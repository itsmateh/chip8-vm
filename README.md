# CHIP-8 Virtual Machine (Rust)

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Status](https://img.shields.io/badge/status-active-success.svg?style=for-the-badge)

A lightweight implementation of a CHIP-8 CPU core written in Rust. This project simulates the **Fetch-Decode-Execute** cycle, memory management, and stack operations of the classic CHIP-8 interpreter.

## 📖 About the Project

This project started as an exploration of systems programming based on an example from the book *Rust in Action*. While the initial concept covered basic function calls (`CALL`, `RET`) and simple addition, I significantly expanded the architecture to build a more complete and functional CPU emulation.

**Key extensions I implemented:**
* **Full ALU Operations:** Implemented `SUB`, `AND`, `OR`, `XOR`, and `MOV` logic.
* **Flag Management:** Added logic for the `VF` (0xF) register to handle **carry** (overflow) and **borrow** flags during arithmetic operations.
* **Opcode Helpers:** Created helper functions to construct and decode opcodes dynamically, making the code more modular and testable.
* **Safety Checks:** Implemented stack overflow/underflow protection using Rust's safety features.

## ⚙️ Architecture

The CPU struct simulates the standard CHIP-8 components:

```rust
struct CPU {
    registers: [u8; 16],      // General purpose registers (V0-VF)
    position_in_memory: usize,// Program Counter (PC)
    memory: [u8; 0x1000],     // 4KB RAM
    stack: [u16; 16],         // Subroutine stack
    stack_pointer: usize,     // Stack Pointer (SP)
}
```
## 🚀 Implemented Opcodes

The emulator currently supports the following instruction families:

| Opcode Family | Description | Status |
| :--- | :--- | :--- |
| **0x00E0** | Clear Screen | 🚧 Todo |
| **0x00EE** | Return from subroutine (`RET`) | ✅ Implemented |
| **0x2NNN** | Call subroutine at NNN (`CALL`) | ✅ Implemented |
| **0x8XY0** | Move VY into VX | ✅ Implemented |
| **0x8XY1** | Bitwise OR | ✅ Implemented |
| **0x8XY2** | Bitwise AND | ✅ Implemented |
| **0x8XY3** | Bitwise XOR | ✅ Implemented |
| **0x8XY4** | Add VY to VX (w/ Carry flag) | ✅ Implemented |
| **0x8XY5** | Subtract VY from VX (w/ Borrow flag)| ✅ Implemented |
