use std::{collections::HashMap};

use crate::parser::{TokenNode, NodeType, RhErr, Error};

macro_rules! switch {
    ($x:expr) => {
        if $x == 1 { $x = 2 } else { $x = 1 }
    };
}

// Idea: Hashmap to connect stack positions with ids
pub fn code_gen(node: &TokenNode) -> String {
    println!("In code gen");
    // use this later
    //                                x0     x1     x2
    // let mut reg_tracker: &mut [bool; 12] = &mut [false, false, false, false, false, false, false, false, false, false, false, false]; // false mean avaliable
    // var_name : pos_on_stack
    let mut code: String = String::from("");
    // Stores the variable name and their position on the stack
    let mut vars: HashMap<String, i32> = HashMap::new();
    let mut sp = 5060;
    println!("{:?}", node.token);
    println!("{:?}", node.children.as_ref().unwrap()[0].token);
    if node.token == NodeType::Program {
        for child_node in node.children.as_ref().expect("program node to have children") {
            match &child_node.token {
                NodeType::Declaration(name) => {
                    code.push_str(&declare_code_gen(&child_node, &mut sp, &mut vars, &mut 1, name.as_ref().expect("valid name to have been given").clone()));
                },
                NodeType::Assignment(name) => {
                    code.push_str(&assignment_code_gen(&child_node, &mut sp, &mut vars, &mut 1, name.as_ref().expect("valid name to have been given").clone()).unwrap());
                },
                NodeType::If => {
                    
                },
                NodeType::While => {
    
                },
                NodeType::Loop => {
    
                },
                _ => {
                    
                }
            }
        }
    }
    if code == "" { panic!("Expected valid program"); }
    println!();
    println!("{}", code);
    println!();
    code.trim().to_string()
}

/// Returns the generated code
/// Modifies the sp
pub fn declare_code_gen(node: &TokenNode, sp: &mut i32, vars: &mut HashMap<String, i32>, x: &mut i32, name: String) -> String {
    println!("{:?}", node);
    println!("{:?}", node.token); 
    let mut code = String::from("\nmov x1, #0\nmov x2, #0");
    // let var_name = match node.token {
    //     NodeType::Id(id) => id,
    //     _ => { panic!("must have valid variable name") }
    // };
    code.push_str(&expr_code_gen(&node.children.as_ref().expect("Node to have children")[0], sp, vars, x));
    vars.insert(name, *sp-16);
    code.push_str(format!("\nstr x1, [sp, #-16]").as_str());
    // store_var(node, num, bar_name, map, 1);
    println!("declare code: {}", code);
    code
}

pub fn assignment_code_gen(node: &TokenNode, sp: &mut i32, vars: &mut HashMap<String, i32>, x: &mut i32, name: String) -> Result<String, RhErr> {
    let mut code = String::from("");
    

    let expr_code: String = expr_code_gen(node, sp, vars, x);
    let stack_position = match vars.get_mut(&name) {
        Some(pos) => pos,
        None => return Err(RhErr::new(Error::UndeclaredId, None))
    };

    match node.token {
        NodeType::Eq => {
            code = format!("{}\nstr x1,={:#x}\n", expr_code, stack_position);
        },
        NodeType::AddEq => {
            code = format!("{}\nldr x2,={:#x}\nadd x1, x1, x2\nstr x1,={:#x}", expr_code, stack_position, stack_position);
        },
        NodeType::SubEq => {
            code = format!("{}\nldr x2,={:#x}\nsub x1, x1, x2\nstr x1,={:#x}", expr_code, stack_position, stack_position);
        },
        NodeType::MulEq => {
            code = format!("{}\nldr x2,={:#x}\nmul x1, x1, x2\nstr x1,={:#x}", expr_code, stack_position, stack_position);
        },
        NodeType::DivEq => {
            code = format!("{}\nldr x2,={:#x}\ndiv x1, x1, x2\nstr x1,={:#x}", expr_code, stack_position, stack_position);
        },
        NodeType::BOrEq => {
            code = format!("{}\nldr x2,={:#x}\nor x1, x1, x2\nstr x1,={:#x}", expr_code, stack_position, stack_position);
        },
        NodeType::BAndEq => {
            code = format!("{}\nldr x2,={:#x}\nand x1, x1, x2\nstr x1,={:#x}", expr_code, stack_position, stack_position);
        },
        NodeType::BXorEq => {
            code = format!("{}\nldr x2,={:#x}\nxor x1, x1, x2\nstr x1,={:#x}", expr_code, stack_position, stack_position);
        },
        _ => {
            return Err(RhErr::new(Error::ExpectedAssignment, None));
        }
    };

    // let expr_code: String = expr_code_gen(node, sp, vars, reg_tracker);
    // code = format!("{}\nstr x1, [sp, #-16]", expr_code, ); // actually write saving and loading

    Ok(code)
}

/// result of expression is always in x1(ideally), if not this than x{x}
/// think of a clever way to parse all literal statements
/// there's defintely a cleverer recursive way of handling this
/// refactor later
/// Needs better handling when invalid identifier is found
pub fn expr_code_gen(node: &TokenNode, sp: &mut i32, vars: &mut HashMap<String, i32>, x: &mut i32) -> String {
    let mut code = String::from("");
    // println!("expr_code_gen");
    println!("expr node: {:?}", node.token);
    // optimizes num literals
    match &node.token {
        NodeType::NumLiteral(val) => {
            code.push_str(format!("\nmov x{}, {}", x, val).as_str());
            switch!(*x);
        },
        NodeType::Id(name) => {
            code.push_str(format!("\nldr x{},={:#x}", x, vars.get(name).expect("variable to exist")).as_str());
            switch!(*x);
        },
        _ => {
            let child0 = &node.children.as_ref().expect("Op node to have children")[0];
            let child1 = &node.children.as_ref().expect("Op node to have children")[1];
            println!("child0: {:?}", child0.token);
            match &child0.token {
                NodeType::NumLiteral(num0) => {
                    match &child1.token {
                        NodeType::NumLiteral(num1) => {
                            let n: i32;
                            match node.token {
                                NodeType::Add => { n = num0 + num1 },
                                NodeType::Sub => { n =  num0 - num1 },
                                NodeType::Div => { n = num0 / num1 },
                                NodeType::Mul => { n = num0 * num1 },
                                NodeType::BAnd => { n = num0 & num1 },
                                NodeType::BOr => { n = num0 | num1 },
                                NodeType::BXor => { n = num0 ^ num1 },
                                _ => panic!("Expected Expression")
                            };
                            code.push_str(format!("\nmov x{}, {}", x, n).as_str());
                            switch!(*x);
                            return code;
                        },
                        NodeType::Id(id) => {
                            code.push_str(format!("\nldr x{},={:#x}", x, vars.get(id).expect("id should be valid")).as_str());
                            switch!(*x);
                            return code;
                        }
                        _ => {
                            code.push_str(expr_code_gen(node, sp, vars, x).as_str());
                        }
                    };
                },
                NodeType::Id(id) => {
                    code.push_str(format!("\nldr x{},={:#x}", x, vars.get(id).expect("id should be valid")).as_str());
                    switch!(*x);
                }
                _ => {
                    code.push_str(expr_code_gen(child0, sp, vars, x).as_str());
                    match &child1.token {
                        NodeType::NumLiteral(num) => {
                            code.push_str(format!("\nmov x{}, {}", x, num).as_str());
                            switch!(*x);
                        },
                        NodeType::Id(id) => {
                            let stack_position = vars.get(id).expect("id should be valid");
                            code.push_str(format!("\nldr x{},={:#x}", x, stack_position).as_str());
                            switch!(*x);
                        },
                        _ => code.push_str(expr_code_gen(child1, sp, vars, x).as_str())
                    };
                    

                    match node.token {
                        NodeType::Add => { code.push_str(format!("\nadd x1, x1, x2").as_str()); },
                        NodeType::Sub => { code.push_str("\nsub x1, x1, x2"); },
                        NodeType::Div => { code.push_str("\ndiv x1, x1, x2"); },
                        NodeType::Mul => { code.push_str("\nmul x1, x1, x2"); },
                        NodeType::BAnd => { code.push_str("\nand x1, x1, x2"); },
                        NodeType::BOr => { code.push_str("\nor x1, x1, x2"); },
                        NodeType::BXor => { code.push_str("\nxor x1, x1, x2"); },
                        _ => println!("Expected Expression")
                    };
                    return code;
                }
            };
        }
    };
    
    code
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