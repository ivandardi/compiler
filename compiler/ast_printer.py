from . import crust_ast


class AstPrinter(crust_ast.AstVisitor):

    def visit_Program(self, node: crust_ast.Program):
        return {
            'Program': {
                'Global Variables': [self.visit(decl) for decl in node.var_decls],
                'Functions': [self.visit(decl) for decl in node.function_decls],
            }
        }

    def visit_VariableDecl(self, node: crust_ast.VariableDecl):
        return {
            'Variable Declaration': {
                'Ident': node.ident,
                'Type': self.visit(node.type)
            }
        }

    def visit_FunctionDecl(self, node: crust_ast.FunctionDecl):
        return {
            'Function Declaration': {
                'Ident': node.ident,
                'Parameters': [self.visit(decl) for decl in node.params],
                'Return Type': self.visit(node.type),
                'Body': {
                    'Variable Declarations': [self.visit(decl) for decl in node.var_decls],
                    'Statements': [self.visit(stmt) for stmt in node.stmts],
                }
            }
        }

    def visit_StmtAssign(self, node: crust_ast.StmtAssign):
        return {
            'Assignment': {
                'Variable': self.visit(node.variable),
                'Expression': self.visit(node.expr),
            }
        }

    def visit_StmtIf(self, node: crust_ast.StmtIf):
        return {
            'If': {
                'Condition': self.visit(node.condititon),
                'If Body': [self.visit(stmt) for stmt in node.if_body],
                'Else Body': [self.visit(stmt) for stmt in node.else_body],
            }
        }

    def visit_StmtWhile(self, node: crust_ast.StmtWhile):
        return {
            'While': {
                'Condition': self.visit(node.condititon),
                'Body': [self.visit(stmt) for stmt in node.stmts],
            }
        }

    def visit_StmtReturn(self, node: crust_ast.StmtReturn):
        if node.expr:
            return {
                'Return': {
                    'Expression': self.visit(node.expr),
                }
            }
        return 'Empty Return'

    def visit_Comparison(self, node: crust_ast.Comparison):
        return {
            'Comparison': {
                'Left Expression': self.visit(node.expr_left),
                'Comparison Operator': node.relational_op,
                'Right Expression': self.visit(node.expr_right),
            }
        }

    def visit_BinOp(self, node: crust_ast.BinOp):
        return {
            'Binary Operation': {
                'Left Expression': self.visit(node.left),
                'Relational Operator': node.operator,
                'Right Expression': self.visit(node.right),
            }
        }

    def visit_FunctionCall(self, node: crust_ast.FunctionCall):
        return {
            'Function Call': {
                'Ident': node.ident,
                'Arguments': [self.visit(arg) for arg in node.args],
            }
        }

    def visit_Variable(self, node: crust_ast.Variable):
        if node.expr:
            return {
                'Array Variable': {
                    'Ident': node.ident,
                    'Array Expression': self.visit(node.expr),
                }
            }
        return {
            'Variable': {
                'Ident': node.ident,
            }
        }

    def visit_TypeDecl(self, node: crust_ast.TypeDecl):
        if node.optional_type:
            return {
                'Array Type': self.visit(node.optional_type),
            }
        return node.type

    def visit_Number(self, node: crust_ast.Number):
        return {
            'Number': node.number,
        }
