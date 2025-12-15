# Sauce 🌶️

**Sauce** is a statically typed, expression-oriented programming language that *feels like a script*.

It is designed around **pipelines as the primary data flow** and **algebraic effects (`toss` / `handle`) instead of async/await**.
The goal is to explore a modern, minimal language design while building a full compiler from scratch in Rust.

This repository contains the **compiler frontend and typechecker**, with LLVM-based code generation planned next.

---

## Why Sauce?

Most languages bolt pipelines, async, and effects on top of existing semantics.

Sauce does the opposite:

* Pipelines (`|>`) are the *default* way data flows
* Effects are explicit (`toss`) instead of implicit control flow
* The language stays small, readable, and predictable

Example:

```sauce
grab x = (1 |> 2) |> toss boom "oh no";
yell x;
toss crash "fatal";
```

---

## Language Overview

### Keywords

| Keyword | Meaning           |
| ------- | ----------------- |
| `grab`  | Bind a value      |
| `yell`  | Print a value     |
| `toss`  | Perform an effect |

---

## Expressions

Sauce is expression-first.

Supported expressions:

* Integer literals
* String literals
* Identifiers
* Parenthesized expressions
* Pipelines (`a |> b |> c`)
* Effect expressions (`toss effect arg`)

### Pipelines

Pipelines are **left-associative**:

```sauce
a |> b |> c
```

parses as:

```
(a |> b) |> c
```

This is enforced in the parser using a fold-left strategy.

---

### Effect Expressions (`toss`)

`toss` is a first-class expression, not a statement-only construct.

```sauce
toss network_error "timeout"
```

It can appear:

* inside pipelines
* inside bindings
* as a standalone expression statement

This design allows effects to compose naturally with data flow.

---

## Statements

Supported statements:

```sauce
grab x = expr;
yell expr;
expr;
```

`toss` does **not** need a special statement form — it is parsed as an expression statement when used alone.

---

## Compiler Architecture

```
source code
   ↓
Lexer (Logos)
   ↓
SpannedToken stream
   ↓
Parser (Chumsky)
   ↓
AST
   ↓
Typechecker
   ↓
(codegen next)
```

---

## Lexer

* Built using **Logos**
* Produces `SpannedToken { token, span }`
* Every token carries byte-range information for error reporting
* Invalid tokens produce structured `LexError`

---

## Parser

* Built using **Chumsky**
* Operates on `SpannedToken` instead of raw text
* Fully recursive expression grammar
* Clean separation between expressions and statements
* Dedicated parser driver (`SauceParser`) used by the CLI

### Key parser features

* Recursive expressions via `recursive(|expr| { ... })`
* Correct pipeline associativity using `foldl`
* `toss` integrated directly into expression grammar
* Expression statements supported

---

## AST

### Expressions

```rust
enum Expr {
    Int(i64),
    String(String),
    Ident(String),
    Pipeline(Box<Expr>, Box<Expr>),
    Toss {
        effect: String,
        arg: Option<Box<Expr>>,
    },
}
```

### Statements

```rust
enum Statement {
    Let { name: String, expr: Expr },
    Yell { expr: Expr },
    ExprStmt(Expr),
}
```

### Program

```rust
struct Ast {
    items: Vec<Statement>,
}
```

---

## Type System (Implemented)

Sauce currently has a **minimal but strict type system**.

### Types

```rust
enum Type {
    Int,
    String,
    Unit,
}
```

### Type Environment

The typechecker tracks bindings using a scoped environment:

```rust
struct TypeEnv {
    vars: HashMap<String, Type>,
}
```

---

## Type Checking Rules

### Literals

* `Int` → `Type::Int`
* `String` → `Type::String`

### Identifiers

* Must exist in the environment
* Otherwise → type error

### `grab`

```sauce
grab x = expr;
```

* `expr` is typechecked
* `x` is bound to that type

### `yell`

```sauce
yell expr;
```

* Expression must be type-valid
* Result is discarded

### Pipelines

```sauce
a |> b
```

Rules:

* `a` must be a valid expression
* `b` must **not** be a literal (`Int` or `String`)
* Resulting type is the type of the right-hand side

This enforces that pipelines represent *computation*, not value chaining.

### `toss`

```sauce
toss effect arg
```

* Always typechecks to `Unit`
* Effects currently do not propagate types (handlers coming next)

---

## Current Status

### Implemented

* Lexer with spans
* Recursive parser
* Expression grammar
* Pipelines
* `toss` syntax
* AST
* Typechecker
* CLI runner that prints AST

### Not Implemented Yet

* Effect handlers (`handle`)
* Effect typing
* Code generation
* Runtime system
* Standard library

---

## Next Steps

### Short-term

* Design `handle { ... }` blocks
* Extend AST for handlers
* Typecheck effect handling
* Decide effect semantics (resume vs abort)

### Medium-term

* LLVM IR generation using Inkwell
* Runtime support for `yell` and effects
* `sauce build` / `sauce run` CLI

### Long-term

* Effect polymorphism
* Better diagnostics (Ariadne)
* Documentation & examples
* Sauce v1.0

---

## Building & Running

```bash
cargo run
```

By default, the compiler reads `example.sauce` and prints the parsed AST.

---

## Philosophy

Sauce is intentionally:

* small
* explicit
* pipeline-first
* effect-aware

This project is as much about **learning language design and compiler architecture** as it is about building a usable language.
