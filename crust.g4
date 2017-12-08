grammar crust;

// Starting tokens

OP_eq: '==';
OP_ne: '!=';
OP_le: '<=';
OP_ge: '>=';
OP_lt: '<' ;
OP_gt: '>' ;

ID    : [a-zA-Z][a-zA-Z0-9]*;
NUMBER: [0-9]+;

WS          : [ \t\r\n]+ -> skip;
COMMENT     : '/*' .*? '*/' -> skip;
LINE_COMMENT: '//' ~[\r\n]* -> skip;

// Context free grammar

program
: variableDecl* functionDecl*
;

variableDecl
: 'let' ID ':' typeDecl ';'
;

functionDecl
: 'fn' ID '(' (params+=functionParamsDecl ',')* params+=functionParamsDecl? ')' ':' typeDecl '{' variableDecl* stmt* '}'
;

functionParamsDecl
: ID ':' typeDecl
;

// Statements

stmt
: stmt_expr
| stmt_assign
| stmt_if
| stmt_while
| stmt_return
;

stmt_expr
: expr ';'
;

stmt_assign
: variable '=' expr ';'
;

stmt_if
: 'if' condition '{' if_body=stmt* '}' ('else' '{' else_body=stmt* '}')?
;

stmt_while
: 'while' condition '{' stmt* '}'
;

stmt_return
: 'return' expr? ';'
;

condition
: left=expr relational=(OP_eq | OP_ne | OP_le | OP_ge | OP_lt | OP_gt) right=expr
;

// Expressions

expr
: expr op=('+' | '-') factor
| factor
;

factor
: factor op=('*' | '/') term
| term
;

term
: NUMBER
| variable
| functionCall
| '(' expr ')'
;

functionCall
: ID '(' (args+=expr ',')* args+=expr? ')'
;

variable
: ID ('[' expr ']')?
;

typeDecl
: 'int'
| 'void'
| '[' ']' typeDecl
;

