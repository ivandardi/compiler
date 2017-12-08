# Generated from /home/ivan/Documents/Github/UNIFESP/compiler_antlr/crust.g4 by ANTLR 4.7
from antlr4 import *
if __name__ is not None and "." in __name__:
    from .crustParser import crustParser
else:
    from crustParser import crustParser

# This class defines a complete listener for a parse tree produced by crustParser.
class crustListener(ParseTreeListener):

    # Enter a parse tree produced by crustParser#program.
    def enterProgram(self, ctx:crustParser.ProgramContext):
        pass

    # Exit a parse tree produced by crustParser#program.
    def exitProgram(self, ctx:crustParser.ProgramContext):
        pass


    # Enter a parse tree produced by crustParser#variableDecl.
    def enterVariableDecl(self, ctx:crustParser.VariableDeclContext):
        pass

    # Exit a parse tree produced by crustParser#variableDecl.
    def exitVariableDecl(self, ctx:crustParser.VariableDeclContext):
        pass


    # Enter a parse tree produced by crustParser#functionDecl.
    def enterFunctionDecl(self, ctx:crustParser.FunctionDeclContext):
        pass

    # Exit a parse tree produced by crustParser#functionDecl.
    def exitFunctionDecl(self, ctx:crustParser.FunctionDeclContext):
        pass


    # Enter a parse tree produced by crustParser#functionParamsDecl.
    def enterFunctionParamsDecl(self, ctx:crustParser.FunctionParamsDeclContext):
        pass

    # Exit a parse tree produced by crustParser#functionParamsDecl.
    def exitFunctionParamsDecl(self, ctx:crustParser.FunctionParamsDeclContext):
        pass


    # Enter a parse tree produced by crustParser#stmt.
    def enterStmt(self, ctx:crustParser.StmtContext):
        pass

    # Exit a parse tree produced by crustParser#stmt.
    def exitStmt(self, ctx:crustParser.StmtContext):
        pass


    # Enter a parse tree produced by crustParser#stmt_expr.
    def enterStmt_expr(self, ctx:crustParser.Stmt_exprContext):
        pass

    # Exit a parse tree produced by crustParser#stmt_expr.
    def exitStmt_expr(self, ctx:crustParser.Stmt_exprContext):
        pass


    # Enter a parse tree produced by crustParser#stmt_assign.
    def enterStmt_assign(self, ctx:crustParser.Stmt_assignContext):
        pass

    # Exit a parse tree produced by crustParser#stmt_assign.
    def exitStmt_assign(self, ctx:crustParser.Stmt_assignContext):
        pass


    # Enter a parse tree produced by crustParser#stmt_if.
    def enterStmt_if(self, ctx:crustParser.Stmt_ifContext):
        pass

    # Exit a parse tree produced by crustParser#stmt_if.
    def exitStmt_if(self, ctx:crustParser.Stmt_ifContext):
        pass


    # Enter a parse tree produced by crustParser#stmt_while.
    def enterStmt_while(self, ctx:crustParser.Stmt_whileContext):
        pass

    # Exit a parse tree produced by crustParser#stmt_while.
    def exitStmt_while(self, ctx:crustParser.Stmt_whileContext):
        pass


    # Enter a parse tree produced by crustParser#stmt_return.
    def enterStmt_return(self, ctx:crustParser.Stmt_returnContext):
        pass

    # Exit a parse tree produced by crustParser#stmt_return.
    def exitStmt_return(self, ctx:crustParser.Stmt_returnContext):
        pass


    # Enter a parse tree produced by crustParser#condition.
    def enterCondition(self, ctx:crustParser.ConditionContext):
        pass

    # Exit a parse tree produced by crustParser#condition.
    def exitCondition(self, ctx:crustParser.ConditionContext):
        pass


    # Enter a parse tree produced by crustParser#expr.
    def enterExpr(self, ctx:crustParser.ExprContext):
        pass

    # Exit a parse tree produced by crustParser#expr.
    def exitExpr(self, ctx:crustParser.ExprContext):
        pass


    # Enter a parse tree produced by crustParser#factor.
    def enterFactor(self, ctx:crustParser.FactorContext):
        pass

    # Exit a parse tree produced by crustParser#factor.
    def exitFactor(self, ctx:crustParser.FactorContext):
        pass


    # Enter a parse tree produced by crustParser#term.
    def enterTerm(self, ctx:crustParser.TermContext):
        pass

    # Exit a parse tree produced by crustParser#term.
    def exitTerm(self, ctx:crustParser.TermContext):
        pass


    # Enter a parse tree produced by crustParser#functionCall.
    def enterFunctionCall(self, ctx:crustParser.FunctionCallContext):
        pass

    # Exit a parse tree produced by crustParser#functionCall.
    def exitFunctionCall(self, ctx:crustParser.FunctionCallContext):
        pass


    # Enter a parse tree produced by crustParser#variable.
    def enterVariable(self, ctx:crustParser.VariableContext):
        pass

    # Exit a parse tree produced by crustParser#variable.
    def exitVariable(self, ctx:crustParser.VariableContext):
        pass


    # Enter a parse tree produced by crustParser#typeDecl.
    def enterTypeDecl(self, ctx:crustParser.TypeDeclContext):
        pass

    # Exit a parse tree produced by crustParser#typeDecl.
    def exitTypeDecl(self, ctx:crustParser.TypeDeclContext):
        pass


