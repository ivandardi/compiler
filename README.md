compiler

# References

1. [Understanding lvalues and rvalues in C and C++](http://eli.thegreenplace.net/2011/12/15/understanding-lvalues-and-rvalues-in-c-and-c)

# Scope system

Composite pattern, works like folders in Unix.

Can be encoded with strings, so scope of `input()` is `""`
, scope of `baz` is `"foo"` and scope of `foobar` is `"foo.innerfoo"`.

```
/ (global scope)
├── foo()
│   ├── var baz
│   ├── var bar
│   └── inner_foo()
│       └── var foobar
├── var global
├── input()
├── print()
└── main()
```

# TODO

1. Add `read()` and `print()` functions
2. Implement semantic checks
3. Create symbol table
