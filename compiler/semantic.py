from tabulate import tabulate

from . import crust_ast


class Symbol:
    def __init__(self, name, scope, line, id_type, canonical_type):
        self.name = name
        self.scope = scope
        self.lines = {line}
        self.id_type = id_type
        self.canonical_type = canonical_type

    def as_tuple(self):
        return self.name, self.scope, ', '.join(map(str, sorted(self.lines))), self.id_type, self.canonical_type


class SymbolTableGenerator(crust_ast.AstVisitor):
    def __init__(self, ast_):
        self.table = {}
        self.errors = []
        self._scope = ''

        self.visit(ast_)

        if 'main' not in self.table:
            self.errors.append(f'No main function declared')

    def __str__(self):
        return tabulate(
            tabular_data=[(key,) + symbol.as_tuple() for key, symbol in self.table.items()],
            headers=['Key', 'Name', 'Scope', 'Lines', 'Id Type', 'Data Type'],
            tablefmt='grid',
        )

    def scoped_name(self, name):
        if not self._scope:
            return name
        return f'{self._scope}.{name}'

    def visit_Program(self, node: crust_ast.Program):
        for decl in node.var_decls:
            self.visit(decl)
        for decl in node.function_decls:
            self.visit(decl)

    def visit_FunctionDecl(self, node: crust_ast.FunctionDecl):
        if node.ident in self.table:
            self.errors.append(f'{node.line}: Function "{node.ident}" declared again')
            return
        self.table[node.ident] = Symbol(node.ident, self._scope, node.line, 'fn', node.type)
        self._scope = node.ident
        for decl in node.params:
            self.visit(decl)
        for decl in node.var_decls:
            self.visit(decl)
        for stmt in node.stmts:
            self.visit(stmt)
        self._scope = ''

    def visit_VariableDecl(self, node: crust_ast.VariableDecl):
        name = self.scoped_name(node.ident)
        if name in self.table:
            self.errors.append(f'{node.line}: Variable "{node.ident}" declared again')
            return False
        if node.ident in self.table:
            self.errors.append(f'{node.line}: Variable "{node.ident}" shares name with a function')
            return False
        if node.type.type == 'void':
            self.errors.append(f'{node.line}: Cannot declare void variable')
        self.table[name] = Symbol(node.ident, self._scope, node.line, 'var', node.type)
        return True

    def visit_StmtAssign(self, node: crust_ast.StmtAssign):
        if not self.visit(node.variable) or not self.visit(node.expr):
            return False
        class_name = type(node.expr).__name__
        if class_name == 'FunctionCall':
            function_call = self.table[node.expr.ident]
            actual_type = function_call.canonical_type
        elif class_name == 'Variable':
            var = self.table[self.scoped_name(node.expr.ident)]
            actual_type = var.canonical_type
        elif class_name == 'Number':
            actual_type = 'Number'
        elif class_name == 'BinOp':
            return True
        else:
            self.errors.append(f'{node.line}: Not an instance, its actually a {type(node.expr).__name__}')
            return False
        if actual_type.type == 'void':
            self.errors.append(f'{node.line}: Invalid assignment of type {actual_type}')
            return False
        return True

    def visit_StmtIf(self, node: crust_ast.StmtIf):
        return (self.visit(node.condititon)
                and all(self.visit(stmt) for stmt in node.if_body)
                and all(self.visit(stmt) for stmt in node.else_body))

    def visit_StmtWhile(self, node: crust_ast.StmtWhile):
        return self.visit(node.condititon) and all(self.visit(stmt) for stmt in node.stmts)

    def visit_StmtReturn(self, node: crust_ast.StmtReturn):
        if node.expr:
            return self.visit(node.expr)
        return True

    def visit_Comparison(self, node: crust_ast.Comparison):
        return all([self.visit(node.expr_left), self.visit(node.expr_right)])

    def visit_BinOp(self, node: crust_ast.BinOp):
        return all([self.visit(node.left), self.visit(node.right)])

    def visit_FunctionCall(self, node: crust_ast.FunctionCall):
        name = node.ident
        is_in_global_scope = name in self.table
        if not is_in_global_scope:
            self.errors.append(f'{node.line}: Function "{node.ident}" used but not declared')
            return False
        self.table[name].lines.add(node.line)
        return all(self.visit(arg) for arg in node.args)

    def visit_Variable(self, node: crust_ast.Variable):
        name = self.scoped_name(node.ident)
        is_in_local_scope = name in self.table
        is_in_global_scope = node.ident in self.table
        if is_in_local_scope:
            self.table[name].lines.add(node.line)
        elif is_in_global_scope:
            self.table[node.ident].lines.add(node.line)
        else:
            self.errors.append(f'{node.line}: Variable "{node.ident}" used but not declared')
            return False
        return True

    def visit_Number(self, node: crust_ast.Number):
        pass
