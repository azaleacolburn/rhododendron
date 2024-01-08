*at the end of statement means 0 or more of the previous thing

statement := declaration
                := assignment
                := loop
                := if

if := condition_expr

condition_expr := condition_term (== != condition_term)*
  
condition_term := factor (|| && condition_term)*

declaration := expressionw

expression := term (+ - term)*

term := factor (* / term)*

factor := numerical_literal
		  := identifier
		  := (expression)

