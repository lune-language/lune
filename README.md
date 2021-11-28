# Lune
A small C like programming language with the goal of compiling to C.

## Progress:
- [x] Lexing
- [ ] Parsing
- [ ] AST generation
- [ ] Type checking
- [ ] Codegen

### Syntax
```nim
# A comment
proc say_hello() {
    print("hello world!")
}

# conditionals example
var age : int = 18
if age >= 18 {
    print("you can drive")
} else {
    print("you can't drive")
}
```

### Running
```
% cargo run
```