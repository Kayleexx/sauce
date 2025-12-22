# Sauce 🌶️

Sauce is a **statically typed programming language that feels like a script**.

It is built around two core ideas:

- **Pipelines (`|>`) are the default way data flows**
- **Effects are explicit (`toss`) instead of hidden control flow**

Sauce is intentionally small.  
It focuses on correctness and clarity over features.

This repository contains a full compiler pipeline written in Rust:
- lexer
- parser
- typechecker
- interpreter
- LLVM backend

---

## What can Sauce do right now?

As of **v0.1.0**, Sauce can:

- parse and typecheck real programs
- run programs using an interpreter
- compile programs to native binaries using LLVM
- handle integers and strings
- express data flow using pipelines
- represent effects explicitly (interpreter only)

Sauce is early, but it is **real and working**.

---

## A simple example

```sauce
yell "answer is:";
yell 67;
````

Interpreter output:

```
String("answer is:")
Int(67)
```

LLVM-compiled output:

```
answer is:
67
```

The same program can be interpreted, typechecked, or compiled.

---

## Language basics

### Keywords

| Keyword | Meaning           |
| ------- | ----------------- |
| `grab`  | Bind a value      |
| `yell`  | Print a value     |
| `toss`  | Perform an effect |

---

## Expressions

Sauce is expression-oriented.

Supported expressions:

* integer literals (`42`)
* string literals (`"hello"`)
* identifiers
* parenthesized expressions
* pipelines (`a |> b`)
* effect expressions (`toss effect arg`)

---

## Pipelines

Pipelines are the main way data flows.

```sauce
a |> b
```

This means:

1. evaluate `a`
2. bind the result to `_`
3. evaluate `b`

Example:

```sauce
grab x = 10 |> _;
yell x;
```

Pipelines are **left-associative**:

```sauce
a |> b |> c
```

is evaluated as:

```
(a |> b) |> c
```

### Important rule

The right side of a pipeline **cannot be a literal**.
Pipelines represent computation, not value chaining.

---

## The `_` placeholder

Inside a pipeline, `_` refers to the value from the left side.

```sauce
grab x = 5 |> _;
```

Outside a pipeline, `_` has no meaning.

---

## Statements

Supported statements:

```sauce
grab x = expr;
yell expr;
expr;
```

There is no special `toss` statement.
`toss` is just an expression.

---

## Effects (`toss`)

`toss` represents an explicit effect.

```sauce
toss network_error "timeout";
```

Notes:

* Effects are supported in the interpreter
* Effects are **not supported in the LLVM backend yet**
* Attempting to compile effects will produce a clear error

This is an intentional design boundary for v0.1.

---

## Type system

Sauce has a small but strict type system.

### Types

```text
Int
String
Unit
```

### Rules

* Identifiers must be defined before use
* Types are inferred
* Pipelines propagate types from right to left
* Effects evaluate to `Unit`

---

## CLI usage

```bash
sauce run example.sauce
sauce check example.sauce
sauce build example.sauce
```

Legacy flags are also supported.

---

## Current limitations (important)

Sauce v0.1.0 does **not** include:

* arithmetic operators
* conditionals
* loops
* functions

Because of this, algorithms like prime checking or fibonacci
are **not expressible yet**.

This is intentional. The focus is on core semantics first.

---

## Compiler architecture

```
source code
  ↓
Lexer (Logos)
  ↓
Parser (Chumsky)
  ↓
AST
  ↓
Typechecker
  ↓
Interpreter or LLVM backend
```

Each phase is explicit and separate.

---

## Stability notes

Sauce v0.1.0 is an early release.

The architecture is stable, but:

* the AST may evolve
* effect handling is interpreter-only
* the LLVM backend supports a pure subset

Small, focused contributions are preferred.

---

## Philosophy

Sauce is designed to be:

* small
* explicit
* pipeline-first
* effect-aware

It is built to understand how languages work, not to hide complexity.

