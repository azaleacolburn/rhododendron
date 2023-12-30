use std::collections::HashMap;

use crate::parser::{Error, NodeType, RhErr, TokenNode};

// This is technically somehow working right now and I have no idea why, I think it does things
// backwards but it's ok
pub struct ScopeHandler {
    scopes: Vec<String>,
    curr_scope: usize,
}

impl ScopeHandler {
    fn new_scope(&mut self) {
        self.curr_scope = self.scopes.len();
        self.scopes.push(format!("\n.L{}:", self.curr_scope));
    }

    fn push_to_scope(&mut self, string: &str) {
        self.scopes[self.curr_scope].push_str(string)
    }

    pub fn format_scopes(&self) -> String {
        let mut ret = String::from(".global _start\n");

        for scope in self.scopes.iter() {
            let mut chars = scope.chars();
            chars.next();
            let mod_scope = chars.as_str().to_string();
            let mut lines = mod_scope.split("\n").collect::<Vec<&str>>().into_iter();
            ret.push_str(format!("{}\n", lines.next().expect("No lines in iterator")).as_str());
            for line in lines {
                ret.push_str(format!("    {}\n", line).as_str());
            }
        }

        ret
    }

    pub fn print_scopes(&self) {
        for scope in self.scopes.iter() {
            let mut chars = scope.chars();
            chars.next();
            let mod_scope = chars.as_str().to_string();
            let mut lines = mod_scope.split("\n").collect::<Vec<&str>>().into_iter();
            println!("{}", lines.next().expect("No lines in iterator"));
            for line in lines {
                println!("    {}", line);
            }
        }
    }
}

/// This struct manages the stack
/// It stores the furthest offset from the base of the stack
/// This model works on the assumption that the base of the stack shifts around as well
/// So a store looks like this
/// "store x1, [sp, #{}]", (furthest_offset + sp_offset)
pub struct StackHandler {
    stack_handler: HashMap<String, i32>, // contain the relative stack positions
    furthest_offset: i32,
}

impl StackHandler {
    fn new() -> Self {
        StackHandler {
            stack_handler: HashMap::new(),
            furthest_offset: 0,
        }
    }

    fn get_id(&self, id: &str) -> Option<&i32> {
        self.stack_handler.get(id)
    }

    fn insert(&mut self, id: impl ToString, position: i32) {
        if position > self.furthest_offset {
            self.furthest_offset = position
        }
        self.stack_handler.insert(id.to_string(), position);
    }

    fn insert_new_16(&mut self, id: impl ToString) {
        self.furthest_offset -= 16;
        self.insert(id, self.furthest_offset);
    }
}

macro_rules! switch {
    ($x:expr) => {
        $x = 1 - $x
    };
}

// Ask Andrew how to enter into main after trying to do it with code_gen, not parsing
pub fn main(node: &TokenNode) -> String {
    let mut scopes = ScopeHandler {
        scopes: vec![String::from("_start:")],
        curr_scope: 0,
    };
    println!("In code gen");
    // use this later
    // var_name : pos_on_stack
    // Stores the variable name and their absolute position on the stack

    // Stores the variable name and their relative position on the stack(from the top of the stack
    // at the begining of runtime). This should be on a per_scope level
    let mut stack_handler = StackHandler::new();
    println!("{:?}", node.token);
    println!("{:?}", node.children.as_ref().unwrap()[0].token);
    if node.token == NodeType::Program {
        scope_code_gen(
            &node.children.as_ref().unwrap()[0],
            &mut stack_handler,
            &mut 0,
            &mut scopes,
        );
    }

    scopes.format_scopes()
    // code.trim().to_string()
}

pub fn scope_code_gen(
    node: &TokenNode,
    stack_handler: &mut StackHandler,
    w: &mut i32,
    scopes: &mut ScopeHandler,
) {
    println!("Scope Node: {:?}", node);
    for child_node in node.children.as_ref().expect("Scope to have children") {
        match &child_node.token {
            NodeType::Declaration(name) => {
                scopes.push_to_scope(&declare_code_gen(
                    &child_node,
                    stack_handler,
                    w,
                    name.as_ref()
                        .expect("valid name to have been given")
                        .clone(),
                ));
            }
            NodeType::Assignment(_) => {
                scopes.push_to_scope(&assignment_code_gen(&child_node, stack_handler, w).unwrap());
            }
            NodeType::If => {
                match if_code_gen(&child_node, stack_handler, w, scopes) {
                    Ok(()) => {}
                    Err(err) => panic!("Error in if code gen: {:?}", err),
                };
            }
            NodeType::While => {}
            NodeType::Loop => {}
            NodeType::FunctionCall(_id) => {}
            _ => {}
        }
    }
    scopes.push_to_scope("\nret");
}

/// Returns the generated code
/// Modifies the sp
pub fn declare_code_gen(
    node: &TokenNode,
    stack_handler: &mut StackHandler,
    w: &mut i32,
    name: String,
) -> String {
    // println!("{:?}", node);
    println!("declare node: {:?}", node.token);
    let mut code = String::from("");
    // let mut code = String::from("\nmov x1, #0\nmov x2, #0");
    // let var_name = match node.token {
    //     NodeType::Id(id) => id,
    //     _ => { panic!("must have valid variable name") }
    // };
    code.push_str(
        &expr_code_gen(
            &node.children.as_ref().expect("Node to have children")[0],
            stack_handler,
            w,
        )
        .unwrap(),
    );
    stack_handler.insert_new_16(&name);
    code.push_str(
        format!(
            "\nstr w0, [sp, #{}]",
            stack_handler
                .get_id(&name)
                .expect("variable wasn't pushed to stack")
        )
        .as_str(),
    );
    // store_var(node, num, bar_name, map, 1);
    code
}

pub fn assignment_code_gen(
    node: &TokenNode,
    stack_handler: &mut StackHandler,
    w: &mut i32,
) -> Result<String, RhErr> {
    let code;
    println!("assignment node: {:?}", node.token);

    let name = match &node.token {
        NodeType::Assignment(name) => name.as_ref().unwrap(),
        _ => panic!("Given given invalid assignment node"),
    };

    let expr_code: String =
        expr_code_gen(&node.children.as_ref().unwrap()[1], stack_handler, w).unwrap();
    let relative_stack_position = match stack_handler.get_id(name) {
        Some(pos) => pos,
        None => return Err(RhErr::new(Error::UndeclaredId, None)),
    };

    match node.children.as_ref().unwrap()[0].token {
        NodeType::Eq => {
            code = format!("{}\nstr w0, [sp, #{}]", expr_code, relative_stack_position);
        }
        NodeType::AddEq => {
            code = format!(
                "{}\nldr w1, [sp, #{}]\nadd w0, w0, w1\nstr w0, [sp, #{}]",
                expr_code, relative_stack_position, relative_stack_position
            );
        }
        NodeType::SubEq => {
            code = format!(
                "{}\nldr w1, [sp, #{}]\nsub w0, w0, w1\nstr w0, [sp, #{}]",
                expr_code, relative_stack_position, relative_stack_position
            );
        }
        NodeType::MulEq => {
            code = format!(
                "{}\nldr w1, [sp, #{}]\nmul w0, w0, w1\nstr w0, [sp, #{}]",
                expr_code, relative_stack_position, relative_stack_position
            );
        }
        NodeType::DivEq => {
            code = format!(
                "{}\nldr w1, [sp, #{}]\ndiv w0, w0, w1\nstr w0, [sp, #{}]",
                expr_code, relative_stack_position, relative_stack_position
            );
        }
        NodeType::BOrEq => {
            code = format!(
                "{}\nldr w1, [sp, #{}]\nor w0, w0, w1\nstr w0, [sp, #{}]",
                expr_code, relative_stack_position, relative_stack_position
            );
        }
        NodeType::BAndEq => {
            code = format!(
                "{}\nldr w1, [sp, #{}]\nand w0, w0, w1\nstr w0, [sp, #{}]",
                expr_code, relative_stack_position, relative_stack_position
            );
        }
        NodeType::BXorEq => {
            code = format!(
                "{}\nldr w1, [sp, #{}]\nxor w0, w0, w1\nstr w0, [sp, #{}]",
                expr_code, relative_stack_position, relative_stack_position
            );
        }
        _ => {
            return Err(RhErr::new(Error::ExpectedAssignment, None));
        }
    };

    // let expr_code: String = expr_code_gen(node, sp, stack_handler, reg_tracker);
    // code = format!("{}\nstr x1, [sp, #-16]", expr_code, ); // actually write saving and loading

    Ok(code)
}

/// result of expression is always in x1(ideally), if not this than x{x}
/// think of a clever way to parse all literal statements
/// there's defintely a cleverer recursive way of handling this
/// refactor later
/// Needs better handling when invalid identifier is found
pub fn expr_code_gen(
    node: &TokenNode,
    stack_handler: &mut StackHandler,
    w: &mut i32,
) -> Result<String, RhErr> {
    let mut code = String::from("");
    // println!("expr_code_gen");
    println!("expr node: {:?}", node.token);
    // optimizes num literals
    match &node.token {
        NodeType::NumLiteral(val) => {
            code.push_str(format!("\nmov w0, #{}", val).as_str());
            // switch!(*x);
        }
        NodeType::Id(name) => {
            code.push_str(
                format!(
                    "\nldr w0, [sp, #{}]",
                    stack_handler.get_id(name).expect("variable to exist")
                )
                .as_str(),
            );
            // switch!(*x);
        }
        _ => {
            let child0 = &node.children.as_ref().expect("Op node to have children")[0];
            let child1 = &node.children.as_ref().expect("Op node to have children")[1]; // here
            println!("child0: {:?}", child0.token);
            match &child0.token {
                NodeType::NumLiteral(num0) => {
                    match &child1.token {
                        NodeType::NumLiteral(num1) => {
                            let n: i32;
                            match node.token {
                                NodeType::Add => n = num0 + num1,
                                NodeType::Sub => n = num0 - num1,
                                NodeType::Div => n = num0 / num1,
                                NodeType::Mul => n = num0 * num1,
                                NodeType::BAnd => n = num0 & num1,
                                NodeType::BOr => n = num0 | num1,
                                NodeType::BXor => n = num0 ^ num1,
                                _ => panic!("Expected Expression"),
                            };
                            code.push_str(format!("\nmov x{}, #{}", w, n).as_str());
                            switch!(*w);
                            return Ok(code);
                        }
                        NodeType::Id(id) => {
                            code.push_str(
                                format!(
                                    "\nldr w{}, [sp, #{}]",
                                    w,
                                    stack_handler.get_id(id).expect("id should be valid")
                                )
                                .as_str(),
                            );
                            switch!(*w);
                            return Ok(code);
                        }
                        _ => {
                            code.push_str(expr_code_gen(node, stack_handler, w)?.as_str());
                        }
                    };
                }
                NodeType::Id(id) => {
                    let relative_stack_pos = match stack_handler.get_id(id) {
                        Some(pos) => pos,
                        None => {
                            return Err(RhErr::new(Error::UndeclaredId, None));
                        }
                    };
                    code.push_str(format!("\nldr w{}, [sp, {}]", w, relative_stack_pos).as_str());
                    switch!(*w);
                }
                _ => {
                    code.push_str(expr_code_gen(child0, stack_handler, w)?.as_str());
                    match &child1.token {
                        NodeType::NumLiteral(num) => {
                            code.push_str(format!("\nmov x{}, {}", w, num).as_str());
                            switch!(*w);
                        }
                        NodeType::Id(id) => {
                            let relative_stack_pos = match stack_handler.get_id(id) {
                                Some(pos) => pos,
                                None => {
                                    return Err(RhErr::new(Error::UndeclaredId, None));
                                }
                            };
                            code.push_str(
                                format!("\nldr w{}, [sp, {}]", w, relative_stack_pos).as_str(),
                            );
                            switch!(*w);
                        }
                        _ => code.push_str(expr_code_gen(child1, stack_handler, w)?.as_str()),
                    };

                    match node.token {
                        NodeType::Add => {
                            code.push_str(format!("\nadd w0, w0, w1").as_str());
                        }
                        NodeType::Sub => {
                            code.push_str("\nsub w0, w0, w1");
                        }
                        NodeType::Div => {
                            code.push_str("\ndiv w0, w0, w1");
                        }
                        NodeType::Mul => {
                            code.push_str("\nmul w0, w0, w1");
                        }
                        NodeType::BAnd => {
                            code.push_str("\nand w0, w0, w1");
                        }
                        NodeType::BOr => {
                            code.push_str("\nor w0, w0, w1");
                        }
                        NodeType::BXor => {
                            code.push_str("\nxor w0, w0, w1");
                        }
                        _ => println!("Expected Expression"),
                    };
                    return Ok(code);
                }
            };
        }
    };

    Ok(code)
}

pub fn if_code_gen(
    node: &TokenNode,
    stack_handler: &mut StackHandler,
    w: &mut i32,
    scopes: &mut ScopeHandler,
) -> Result<(), RhErr> {
    match &node.children {
        Some(children) => {
            // cmp x1, #n
            // beq label
            // node.children.unwrap()[0] is condition node, other child is scope
            let orig_scope = scopes.curr_scope;

            condition_expr_code_gen(&children[0], w, stack_handler, scopes)?;
            scope_code_gen(&children[1], stack_handler, w, scopes);

            scopes.curr_scope = orig_scope;
            Ok(())
        }
        None => Err(RhErr::new(Error::ExpectedCondition, None)),
    }
}

pub fn condition_expr_code_gen(
    node: &TokenNode,
    w: &mut i32,
    stack_handler: &mut StackHandler,
    scopes: &mut ScopeHandler,
) -> Result<(), RhErr> {
    println!("condition expr code: {:?}", node.token);
    match &node.token {
        NodeType::AndCmp => {}
        NodeType::OrCmp => {}
        NodeType::NeqCmp => {}
        NodeType::EqCmp => {
            condition_expr_code_gen(
                &node
                    .children
                    .as_ref()
                    .expect("more children in condition expr")[0],
                w,
                stack_handler,
                scopes,
            )
            .unwrap();
            condition_expr_code_gen(
                &node
                    .children
                    .as_ref()
                    .expect("more children in condition expr")[1],
                w,
                stack_handler,
                scopes,
            )
            .unwrap();
            scopes.push_to_scope(format!("\ncmp w0, w1\nbeq .L{}", scopes.curr_scope + 1).as_str());
            scopes.new_scope();
        }
        NodeType::Id(id) => {
            let relative_path = stack_handler.get_id(id).unwrap(); // relative path moves stack down without -

            scopes.push_to_scope(format!("\nldr w{}, [sp, #{}]", w, relative_path).as_str());
            switch!(*w)
            // scopes.push_to_scope(format!("\nadd sp, {}, sp", relative_path).as_str());
        }
        NodeType::NumLiteral(num) => {
            scopes.push_to_scope(format!("\nmov w{}, {}", w, num).as_str());
            switch!(*w)
        }
        _ => {
            return Err(RhErr::new(Error::ExpectedCondition, None));
        }
    }
    // code.push_str("\nbeq");
    Ok(())
}

// pub fn store_var(sp: &mut i32, node: &TokenNode, num: i32, var: String, vars: &mut HashMap<String, i32>, reg_tracker: &mut [bool; 12]) -> String {
//     let val = match node.token {
//         NodeType::NumLiteral(i) => i,
//         _ => { panic!("can only store literals") }
//     };
//     vars.insert(var, val);
//     // *(sp + 8) <- x1
//     // sp <- sp - 8
//     let c = format!("\nmov {}, x{}
//             \nstr x{}, [sp, #-16]!", num, x, x);
//     switch!(x);
//     *sp -= 16;
//     c
// }

// /// Loads a variable from the stack into the given reigster
// pub fn load_var(sp: &mut i32, var: String, vars: &HashMap<String, i32>, reg_tracker: &mut [bool; 12]) -> String {
//     let stack_position = vars.get(&var).expect("Variable to have been initialized");
//     let relative_stack_pos = *stack_position - *sp;
//     // x1 <- *sp
//     // sp <- sp + 8
//     let c = format!("\nldr x{}, [sp], #{}", x, relative_stack_pos);
//     switch!(*x);
//     *sp += relative_stack_pos;
//     c
// }
