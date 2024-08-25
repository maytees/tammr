# Tammr Scripting Language Specification

> [!NOTE]  
> This project is a "fork" of my implementation of the Monki programming
> langauge, from the Writing an Interpreter in Go Book, by Thorston Ball.
> Meaning the code will be very similar.

> [!WARNING]  
> Nothing in this README is representative of the current source code.
> Everything below is "theoretical". It will be moved to a wiki/docs website
> soon.

## Table of Contents

- [Tammr Scripting Language Specification](#tammr-scripting-language-specification)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Basic Syntax](#basic-syntax)
    - [Comments](#comments)
    - [Variables](#variables)
    - [Data Types](#data-types)
  - [Control Structures](#control-structures)
    - [Conditionals](#conditionals)
    - [Loops](#loops)
  - [Functions](#functions)
  - [Modules and Imports](#modules-and-imports)
  - [String Operations](#string-operations)
  - [Error Handling](#error-handling)
  - [File Operations](#file-operations)
  - [Command Execution](#command-execution)
  - [Environment Variables](#environment-variables)
  - [Standard Library](#standard-library)
  - [Shebang Support](#shebang-support)

## Introduction

This document specifies the grammar and features of Tammr, a custom scripting language designed for creating automation scripts. Tammr aims to be simple, concise, and powerful enough for various automation tasks.

## Basic Syntax

### Comments

```tammr
// This is a single-line comment in Tammr

/*
   This is a
   multi-line comment in Tammr
*/
```

### Variables

```tammr
let name = value
let type name = value  // type is optional and serves as a hint in Tammr
```

### Data Types

Tammr supports the following data types:

- `str`: String
- `int`: Integer
- `float`: Float
- `boolean`: Boolean (true/false)
- `arr`: Array
- `module`: Imported module
- `hash`: Key-value pairs (similar to dictionaries)

## Control Structures

### Conditionals

```tammr
if condition do
    // Tammr code here
else if another_condition do
    // More Tammr code
else do
    // Even more Tammr code
end
```

### Loops

```tammr
// While-style loop in Tammr
loop do
    // Tammr code
    exit loopName if condition
end

// For-style loop in Tammr
loop i from 0 to 10 do
    // Tammr code
end

// Foreach-style loop in Tammr
foreach item in collection do
    // Tammr code
end
```

## Functions

```tammr
function funcName(arg1, arg2 = defaultValue) do
    // Tammr function body
    return value
end

// Anonymous function in Tammr
let double = (x) -> x * 2
```

## Modules and Imports

```tammr
import "./path/to/file.tmr" as moduleName

moduleName.function()
// or
function()  // if not ambiguous
```

## String Operations

```tammr
// String concatenation in Tammr
let fullName = firstName + " " + lastName

// String interpolation in Tammr
let greeting = "Hello, ${name}!"

// Multiline strings (Here document) in Tammr
let multiline = <<EOF
This is a
multiline string in Tammr
EOF

// String methods in Tammr
let length = myString.length()
let uppercase = myString.toUpper()
let lowercase = myString.toLower()
let trimmed = myString.trim()
let split = myString.split(",")
let contains = myString.contains("substring")
let replaced = myString.replace("old", "new")
```

## Error Handling

```tammr
try do
    // risky Tammr code
catch error do
    // handle error in Tammr
end
```

## File Operations

```tammr
// Reading a file in Tammr
let content = readFile("path/to/file.txt")

// Writing to a file in Tammr
writeFile("path/to/file.txt", content)

// Appending to a file in Tammr
appendFile("path/to/file.txt", newContent)

// Checking if a file exists in Tammr
let exists = fileExists("path/to/file.txt")
```

## Command Execution

```tammr
let result = exec("ls -l")
println(result)
```

## Environment Variables

```tammr
// Get environment variable in Tammr
let path = getEnv("PATH")

// Set environment variable in Tammr
setEnv("MY_VAR", "value")
```

## Standard Library

Tammr should include a standard library with common utility functions for:

- Math operations
- Date and time manipulation
- Array and hash operations
- Networking (HTTP requests, etc.)
- JSON parsing and serialization

## Shebang Support

Tammr scripts can start with a shebang for direct execution on Unix-like systems:

```tammr
#!/usr/bin/env tammr

// Your Tammr script code here
```
