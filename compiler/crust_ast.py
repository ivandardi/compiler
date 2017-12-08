from .gen.crustParser import crustParser
from .gen.crustVisitor import crustVisitor


class AstNode:
    def __init__(self, line=-1):
        self.line = line


class Program(AstNode):
    def __init__(self, var_decls, function_decls, line=-1):
        super().__init__(line)
        self.var_decls = var_decls
        self.function_decls = function_decls


class VariableDecl(AstNode):
    def __init__(self, ident, type_, line=-1):
        super().__init__(line)
        self.ident = ident
        self.type = type_


class FunctionDecl(AstNode):
    def __init__(self, ident, params, type_, var_decls, stmts, line=-1):
        super().__init__(line)
        self.ident = ident
        self.params = params
        self.type = type_
        self.var_decls = var_decls
        self.stmts = stmts


class StmtAssign(AstNode):
    def __init__(self, variable, expr, line=-1):
        super().__init__(line)
        self.variable = variable
        self.expr = expr


class StmtIf(AstNode):
    def __init__(self, condititon, if_body, else_body, line=-1):
        super().__init__(line)
        self.condititon = condititon
        self.if_body = if_body
        self.else_body = else_body


class StmtWhile(AstNode):
    def __init__(self, condititon, stmts, line=-1):
        super().__init__(line)
        self.condititon = condititon
        self.stmts = stmts


class StmtReturn(AstNode):
    def __init__(self, expr=None, line=-1):
        super().__init__(line)
        self.expr = expr


class Comparison(AstNode):
    def __init__(self, expr_left, relational_op, expr_right, line=-1):
        super().__init__(line)
        self.expr_left = expr_left
        self.relational_op = relational_op
        self.expr_right = expr_right


class BinOp(AstNode):
    def __init__(self, left, operator, right, line=-1):
        super().__init__(line)
        self.left = left
        self.operator = operator
        self.right = right


class FunctionCall(AstNode):
    def __init__(self, ident, args, line=-1):
        super().__init__(line)
        self.ident = ident
        self.args = args


class Variable(AstNode):
    def __init__(self, ident, expr=None, line=-1):
        super().__init__(line)
        self.ident = ident
        self.expr = expr  # Optional expression


class TypeDecl(AstNode):
    def __init__(self, type_, optional_type=None, line=-1):
        super().__init__(line)
        self.type = type_
        self.optional_type = optional_type

    def __str__(self):
        if self.optional_type:
            return '[]' + str(self.optional_type)
        return self.type


class Number(AstNode):
    def __init__(self, number, line=-1):
        super().__init__(line)
        self.number = int(number)


class AstBuilderVisitor(crustVisitor):

    def visitProgram(self, ctx: crustParser.ProgramContext):
        return Program(
            var_decls=[self.visit(decl) for decl in ctx.variableDecl()],
            function_decls=[self.visit(decl) for decl in ctx.functionDecl()],
            line=ctx.start.line,
        )

    def visitVariableDecl(self, ctx: crustParser.VariableDeclContext):
        return VariableDecl(
            ident=ctx.ID().getText(),
            type_=(self.visit(ctx.typeDecl())),
            line=ctx.start.line,
        )

    def visitFunctionDecl(self, ctx: crustParser.FunctionDeclContext):
        return FunctionDecl(
            ident=ctx.ID().getText(),
            params=[self.visit(param) for param in ctx.params],
            type_=self.visit(ctx.typeDecl()),
            var_decls=[self.visit(decl) for decl in ctx.variableDecl()],
            stmts=[self.visit(stmt) for stmt in ctx.stmt()],
            line=ctx.start.line,
        )

    def visitFunctionParamsDecl(self, ctx: crustParser.FunctionParamsDeclContext):
        return VariableDecl(
            ident=ctx.ID().getText(),
            type_=self.visit(ctx.typeDecl()),
            line=ctx.start.line,
        )

    def visitStmt_expr(self, ctx: crustParser.Stmt_exprContext):
        return self.visit(ctx.expr())

    def visitStmt_assign(self, ctx: crustParser.Stmt_assignContext):
        return StmtAssign(
            variable=self.visit(ctx.variable()),
            expr=self.visit(ctx.expr()),
            line=ctx.start.line,
        )

    def visitStmt_if(self, ctx: crustParser.Stmt_ifContext):

        def if_body():
            if isinstance(ctx.if_body, list):
                return [self.visit(expr) for expr in ctx.if_body]
            return [self.visit(ctx.if_body)]

        def else_body():
            if ctx.else_body:
                if isinstance(ctx.else_body, list):
                    return [self.visit(expr) for expr in ctx.else_body]
                return [self.visit(ctx.else_body)]
            return []

        return StmtIf(
            condititon=self.visit(ctx.condition()),
            if_body=if_body(),
            else_body=else_body(),
            line=ctx.start.line,
        )

    def visitStmt_while(self, ctx: crustParser.Stmt_whileContext):
        return StmtWhile(
            condititon=self.visit(ctx.condition()),
            stmts=[self.visit(stmt) for stmt in ctx.stmt()],
            line=ctx.start.line,
        )

    def visitStmt_return(self, ctx: crustParser.Stmt_returnContext):
        return StmtReturn(
            expr=self.visit(ctx.expr()) if ctx.expr() else None,
            line=ctx.start.line,
        )

    def visitCondition(self, ctx: crustParser.ConditionContext):
        assert len(ctx.expr()) == 2
        return Comparison(
            expr_left=self.visit(ctx.left),
            relational_op=ctx.relational.text,
            expr_right=self.visit(ctx.right),
            line=ctx.start.line,
        )

    def visitExpr(self, ctx: crustParser.ExprContext):
        if ctx.op:
            return BinOp(
                left=self.visit(ctx.expr()),
                operator=ctx.op.text,
                right=self.visit(ctx.factor()),
                line=ctx.start.line,
            )
        return self.visit(ctx.factor())

    def visitFactor(self, ctx: crustParser.FactorContext):
        if ctx.op:
            return BinOp(
                left=self.visit(ctx.factor()),
                operator=ctx.op.text,
                right=self.visit(ctx.term()),
                line=ctx.start.line,
            )
        return self.visit(ctx.term())

    def visitTerm(self, ctx: crustParser.TermContext):
        if ctx.NUMBER():
            return Number(
                number=ctx.NUMBER().getText(),
                line=ctx.start.line,
            )
        return self.visitChildren(ctx)

    def visitFunctionCall(self, ctx: crustParser.FunctionCallContext):
        return FunctionCall(
            ident=ctx.ID().getText(),
            args=[self.visit(arg) for arg in ctx.args],
            line=ctx.start.line,
        )

    def visitVariable(self, ctx: crustParser.VariableContext):
        return Variable(
            ident=ctx.ID().getText(),
            expr=self.visit(ctx.expr()) if ctx.expr() else None,
            line=ctx.start.line,
        )

    def visitTypeDecl(self, ctx: crustParser.TypeDeclContext):
        return TypeDecl(
            type_=ctx.getText(),
            optional_type=self.visit(ctx.typeDecl()) if ctx.typeDecl() else None,
            line=ctx.start.line,
        )


class AstVisitor:
    def visit(self, node):
        method_name = 'visit_' + type(node).__name__
        visitor = getattr(self, method_name, self.generic_visit)
        return visitor(node)

    def generic_visit(self, node):
        raise Exception('No visit_{} method'.format(type(node).__name__))
