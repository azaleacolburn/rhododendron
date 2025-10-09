*at the end of statement means 0 or more of the previous thing
|at the end of statements means 0 or one of the previous thing

scope := { (statement ;)* }

statement := declaration
                := assignment
                := loop
                := if
                := function_call

if := condition_expr

condition_expr := condition_term (== != condition_term)*
  
condition_term := factor (|| && condition_term)*

declaration := expression

expression := term (+ - term)*

term := factor (* / term)*

factor := numerical_literal
		  := identifier
		  := (expression)

function_declaration := identifier( declaration* ) (-> type)| scope

function_call := identifier()