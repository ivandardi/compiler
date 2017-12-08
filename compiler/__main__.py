import json
import sys

import argparse
import antlr4

from .semantic import SymbolTableGenerator
from .crust_ast import AstBuilderVisitor
from .ast_printer import AstPrinter
from .gen.crustLexer import crustLexer
from .gen.crustParser import crustParser


def main(argv):
    parser = argparse.ArgumentParser(description='Crust compiler')
    parser.add_argument('--file')
    parser.add_argument('--lexer', action='store_true')
    parser.add_argument('--ast', action='store_true')
    parser.add_argument('--symbol', action='store_true')

    args = parser.parse_args()

    input_str = antlr4.FileStream(args.file)
    lexer = crustLexer(input_str)
    stream = antlr4.CommonTokenStream(lexer)
    parser = crustParser(stream)
    tree = parser.program()

    if args.lexer:
        for token in stream.tokens:
            print(token.line, ":", token.text)

    ast = AstBuilderVisitor().visit(tree)

    if args.ast:
        json_tree = AstPrinter().visit(ast)
        json_tree = json.dumps(json_tree, indent=2)
        print(json_tree)

    semantic = SymbolTableGenerator(ast)

    if args.symbol:
        print(semantic)

    if semantic.errors:
        print('Errors')
        for error in semantic.errors:
            print(error)
        return


if __name__ == '__main__':
    main(sys.argv)
