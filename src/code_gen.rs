use std::collections::HashMap;

use crate::parser::{TokenNode, NodeType};

macro_rules! switch {
    ($x:expr) => {
        if $x == 1 { $x = 2 } else { $x = 1 }
    };
}

// Idea: Hashmap to connect stack positions with ids
pub fn code_gen(node: &TokenNode) -> String {
    // var_name : pos_on_stack
    let mut code: Option<String> = None;
    let mut map: HashMap<String, i32> = HashMap::new();
    let mut sp = 5060;
    if node.token == NodeType::Program {
        if node.children.as_ref().expect("Program node to have children")[0].token == NodeType::Declaration {
            code = Some(declare_code_gen(&node.children.as_ref().expect("Declaration node to have children")[0], &mut sp, &mut map));
        }
    }
    if code.is_none() { panic!("Expected valid program") }
    code.expect("A valid program").trim().to_string()
}

/// Returns the generated code
/// Modifies the sp
pub fn declare_code_gen(node: &TokenNode, sp: &mut i32, map: &mut HashMap<String, i32>) -> String {
    println!("{:?}", node);
    println!("{:?}", node.token);
    // let var_name = match node.token {
    //     NodeType::Id(id) => id,
    //     _ => { panic!("must have valid variable name") }
    // };
    let mut expr_code = expr_code_gen(&node.children.as_ref().expect("Node to have children")[0], sp, map, &mut 1);
    // map.insert(var_name, *sp-8);
    expr_code.push_str(format!("\nstr x1, [sp, #-8]").as_str());
    // store_var(node, num, bar_name, map, 1);
    println!("expr_code_gen code: {}", expr_code);
    expr_code
}

/// x is either 1 or 2, depending on what the current register to be loaded is
/// TODO: Fix this function
pub fn expr_code_gen(node: &TokenNode, sp: &mut i32, map: &mut HashMap<String, i32>, x: &mut i32) -> String {
    let mut code = String::from("");
    println!("expr_code_gen");
    println!("token: {:?}", node.token);
    match &node.token {
        NodeType::NumLiteral(num) => {
            code.push_str(format!("\nmov {}, x{}", num, x).as_str());
            switch!(*x);
            return code;
        },
        NodeType::Id(id) => {
            code.push_str(load_var(sp, id.to_string(), map, x).as_str());
            return code;
        },
        _ => {
            code.push_str(expr_code_gen(&node.children.as_ref().expect("Op node to have children")[0], sp, map, x).as_str());
            code.push_str(expr_code_gen(&node.children.as_ref().expect("Op node to have echildren")[1], sp, map, x).as_str());
            match node.token {
                NodeType::Add => { code.push_str("\nadd x1, x2, x1"); },
                NodeType::Sub => { code.push_str("\nsub x1, x2, x1"); },
                NodeType::Div => { code.push_str("\ndiv x1, x2, x1"); },
                NodeType::Mul => { code.push_str("\nmul x1, x2, x1"); },
                NodeType::BAnd => { code.push_str("\nand x1, x2, x1"); },
                NodeType::BOr => { code.push_str("\nor x1, x2, x1"); },
                NodeType::BXor => { code.push_str("\nxor x1, x2, x1"); },
                _ => println!("Expected Expression")
            };
            switch!(*x);

        }
    };

    code
}

pub fn store_var(sp: &mut i32, node: &TokenNode, num: i32, var: String, map: &mut HashMap<String, i32>, mut x: i32) -> String {
    let val = match node.token {
        NodeType::NumLiteral(i) => i,
        _ => { panic!("can only store literals") }
    };
    map.insert(var, val);
    // *(sp + 8) <- x1
    // sp <- sp - 8
    let c = format!("\nmov {}, x{}
            \nstr x{}, [sp, #-8]!", num, x, x);
    switch!(x);
    *sp -= 8;
    c
}

/// Loads a variable from the stack into the given reigster
pub fn load_var(sp: &mut i32, var: String, map: &HashMap<String, i32>, x: &mut i32) -> String {
    let stack_position = map.get(&var).expect("Variable to have been initialized");
    let relative_stack_pos = *stack_position - *sp;
    // x1 <- *sp
    // sp <- sp + 8
    let c = format!("\nldr x{}, [sp], #{}", x, relative_stack_pos);
    switch!(*x);
    *sp += relative_stack_pos;
    c
}