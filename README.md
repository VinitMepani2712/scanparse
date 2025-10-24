# 🧩 ScanParse — Rust Compiler Front-End

### 📚 Course
Rutgers University — CS 515: Programming Languages & Compilers  
Author: **Vinit Mepani**  
Project: **ScanParse (Rust Edition)**  
Semester: **Fall 2025**

---

## 🚀 Overview
**ScanParse** is a Rust-based compiler front-end that implements the major phases of compilation:
1. **Lexical Analysis (Scanner)** — tokenizes the input expression.  
2. **Parsing** — constructs an **Abstract Syntax Tree (AST)** using recursive-descent parsing.  
3. **DAG Generation** — builds a **Directed Acyclic Graph** to eliminate redundant subexpressions.  
4. **LLVM IR Generation** — produces LLVM-compliant intermediate representation (`.ll`) files.  

This project forms the first phase of a full compiler pipeline for arithmetic expressions, producing IR code compatible with `llc` and `gcc`.

---

## 🧠 Features

| Stage | Description |
|--------|-------------|
| **Scanner** | Converts input into tokens such as identifiers, numbers, operators, and parentheses. |
| **Parser** | Validates expressions according to grammar rules and generates an AST. |
| **DAG Builder** | Optimizes common subexpressions using DAG representation. |
| **LLVM IR Emitter** | Outputs LLVM IR files (e.g., `test1.ll`) for further compilation. |
| **Error Handling** | Reports lexical or syntax errors with descriptive messages and positions. |

---

## 🧩 Grammar Rules

Example grammar used in parsing:

```
E → E + T | T
T → T * F | F
F → (E) | id | num
```

---

## ⚙️ Build & Run Instructions

### 1️⃣ Unpack & Setup
```bash
unzip scanparse.zip
cd scanparse
```

### 2️⃣ Build the Project
```bash
cargo build --release
```

### 3️⃣ Run with Inline Expression
```bash
cargo run --release -- "a + b * (c + d)"
```

### 4️⃣ Run with Input File
```bash
cargo run --release -- input/test1.exp
```

### 5️⃣ Output Files
After running, outputs are saved under the following directories:

| Directory | Description |
|------------|-------------|
| `outputASTDAG/` | AST and DAG text representations |
| `outputLLVMIR/` | LLVM IR files (`.ll`) and assembly outputs (`.s`) |
| `test.c` | C test harness for verifying generated LLVM code |

---

## 🧪 Example

**Input**
```
a + b * (c + d)
```

**Tokens**
```
IDENTIFIER(a)
PLUS
IDENTIFIER(b)
STAR
LPAREN
IDENTIFIER(c)
PLUS
IDENTIFIER(d)
RPAREN
EOF
```

**AST Representation**
```
        (+)
       /   \
     a      (*)
           /   \
          b     (+)
               /   \
              c     d
```

---

## 🧰 Environment & Tools
- **Language:** Rust  
- **Build Tool:** Cargo  
- **Target:** x86-64  
- **IR Backend:** LLVM  
- **Testing Harness:** C (test.c)  
- **Platform:** Linux (Rutgers iLab)  

---

## 🧾 Notes
- Ensure `cargo`, `llc`, and `gcc` are installed and accessible in PATH.  
- To clean build artifacts: `cargo clean`  
- To rebuild IR for all tests: `cargo run --release` for each `.exp` input file.  
- Project verified on Rutgers iLab environment.

---

## 📜 License
This project is part of **Rutgers University CS515 — Programming Languages & Compilers** coursework.  
All rights reserved © 2025 **Vinit Mepani**.
