# Generated from /home/ivan/Documents/Github/UNIFESP/compiler_antlr/crust.g4 by ANTLR 4.7
from antlr4 import *
if __name__ is not None and "." in __name__:
    from .crustParser import crustParser
else:
    from crustParser import crustParser

# This class defines a complete generic visitor for a parse tree produced by crustParser.

class crustVisitor(ParseTreeVisitor):

    # Visit a parse tree produced by crustParser#program.
    def visitProgram(self, ctx:crustParser.ProgramContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by crustParser#variableDecl.
    def visitVariableDecl(self, ctx:crustParser.VariableDeclContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by crustParser#functionDecl.
    def visitFunctionDecl(self, ctx:crustParser.FunctionDeclContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by crustParser#functionParamsDecl.
    def visitFunctionParamsDecl(self, ctx:crustParser.FunctionParamsDeclContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by crustParser#stmt.
    def visitStmt(self, ctx:crustParser.StmtContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by crustParser#stmt_expr.
    def visitStmt_expr(self, ctx:crustParser.Stmt_exprContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by crustParser#stmt_assign.
    def visitStmt_assign(self, ctx:crustParser.Stmt_assignContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by crustParser#stmt_if.
    def visitStmt_if(self, ctx:crustParser.Stmt_ifContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by crustParser#stmt_while.
    def visitStmt_while(self, ctx:crustParser.Stmt_whileContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by crustParser#stmt_return.
    def visitStmt_return(self, ctx:crustParser.Stmt_returnContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by crustParser#condition.
    def visitCondition(self, ctx:crustParser.ConditionContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by crustParser#expr.
    def visitExpr(self, ctx:crustParser.ExprContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by crustParser#factor.
    def visitFactor(self, ctx:crustParser.FactorContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by crustParser#term.
    def visitTerm(self, ctx:crustParser.TermContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by crustParser#functionCall.
    def visitFunctionCall(self, ctx:crustParser.FunctionCallContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by crustParser#variable.
    def visitVariable(self, ctx:crustParser.VariableContext):
        return self.visitChildren(ctx)


    # Visit a parse tree produced by crustParser#typeDecl.
    def visitTypeDecl(self, ctx:crustParser.TypeDeclContext):
        return self.visitChildren(ctx)



del crustParser