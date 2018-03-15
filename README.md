# Rust inspired C Minus (Crust) Compiler

## Useful links to learn Python

http://python.swaroopch.com/ (great book)

https://learnxinyminutes.com/docs/python3/ (for people who know programming already)

https://automatetheboringstuff.com/ (for complete beginners to programming)

https://docs.python.org/3.6/tutorial/ (official tutorial)

## How to run

1. Install [pipenv](https://github.com/pypa/pipenv)

2. Install Antlr 4.7

3. Clone this repository

4. `cd` into the repository

5. Execute `pipenv --python python3.6 install` to install the dependencies

6. Execute `antlr4 -Dlanguage=Python3 -visitor -o compiler/gen crust.g4` to generate the parser

7. Execute `pipenv run python -m compiler --lexer --ast --symbol --file tests/0.crust` to run the compiler

## Command Line Options

The compiler runs as a Python module, so you have to run `python -m compiler` to actually execute the compiler.

To make the compiler output the tokens, add the option `--lexer`.

To make the compiler output the AST, add the option `--ast`.

To make the compiler output the symbol table, add the option `--symbol`.

To specify which file it should compile, add the option `--file <your file here>`.
