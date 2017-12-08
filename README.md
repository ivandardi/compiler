# Rust inspired C Minus (Crust) Compiler

## How to run

1. Install [pipenv](https://github.com/pypa/pipenv)

2. Install Antlr4

3. Clone this repository

4. Open the repository

5. Execute `pipenv --python python3.6 install`

6. Execute `antlr4 -Dlanguage=Python3 -visitor -o compiler/gen crust.g4`

7. Execute `pipenv run python -m compiler --lexer --ast --symbol --file tests/0.crust`

## Command Line Options

The compiler runs as a Python module, so you have to run `python -m compiler` to actually execute the compiler.

To make the compiler output the tokens, add the option `--lexer`.

To make the compiler output the AST, add the option `--ast`.

To make the compiler output the symbol table, add the option `--symbol`.

To specify which file it should compile, add the option `--file <your file here>`.
