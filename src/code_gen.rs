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
        self.scopes
            .push(format!("\n.balign 4\n.L{}:", self.curr_scope));
    }

    fn push_to_scope(&mut self, string: impl ToString) {
        self.scopes[self.curr_scope].push_str(string.to_string().as_str())
    }

    pub fn format_scopes(&self) -> String {
        let mut ret = String::new();
        for scope in self.scopes.iter() {
            let mut lines = scope.lines();
            ret.push_str(format!("{}\n", lines.next().unwrap()).as_str());
            ret.push_str(format!("{}\n", lines.next().unwrap()).as_str());
            ret.push_str(format!("{}\n", lines.next().unwrap()).as_str());

            for line in lines {
                ret.push_str(format!("    {}\n", line).as_str());
            }
        }

        ret.trim().to_string()
    }
    #[allow(dead_code)]
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

    fn insert_expr_literal(&mut self) {
        self.furthest_offset -= 16;
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
        scopes: vec![String::from("\n.global _main\nmain:")],
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
                declare_code_gen(
                    &child_node,
                    stack_handler,
                    w,
                    scopes,
                    name.as_ref()
                        .expect("valid name to have been given")
                        .clone(),
                );
            }
            NodeType::Assignment(_) => {
                assignment_code_gen(&child_node, stack_handler, w, scopes);
            }
            NodeType::If => {
                if_code_gen(&child_node, stack_handler, w, scopes);
            }
            NodeType::While => {
                while_code_gen(&child_node, stack_handler, w, scopes);
            }
            NodeType::Loop => {}
            NodeType::FunctionCall(_id) => {}
            _ => {}
        };
    }
    scopes.push_to_scope("\nret");
}

/// Returns the generated code
/// Modifies the sp
pub fn declare_code_gen(
    node: &TokenNode,
    stack_handler: &mut StackHandler,
    w: &mut i32,
    scopes: &mut ScopeHandler,
    name: String,
) {
    println!("Declare Node: {:?}", node.token);
    expr_code_gen(
        &node.children.as_ref().expect("Node to have children")[0],
        stack_handler,
        w,
        scopes,
    );
    stack_handler.insert_new_16(&name);
    scopes.push_to_scope(
        format!(
            "\nstr w0, [sp, #{}]",
            stack_handler
                .get_id(&name)
                .expect("variable wasn't pushed to stack")
        )
        .as_str(),
    );
}

pub fn assignment_code_gen(
    node: &TokenNode,
    stack_handler: &mut StackHandler,
    w: &mut i32,
    scopes: &mut ScopeHandler,
) {
    println!("assignment node: {:?}", node.token);

    let name = match &node.token {
        NodeType::Assignment(name) => name.as_ref().unwrap(),
        _ => panic!("Given given invalid assignment node"),
    };

    expr_code_gen(
        &node.children.as_ref().unwrap()[1],
        stack_handler,
        w,
        scopes,
    );
    let relative_stack_position = stack_handler.get_id(name).expect("Undefined Identifier");

    match node.children.as_ref().unwrap()[0].token {
        NodeType::Eq => {
            scopes.push_to_scope(format!("\nstr w0, [sp, #{}]", relative_stack_position));
        }
        NodeType::AddEq => {
            scopes.push_to_scope(format!(
                "\nldr w1, [sp, #{}]\nadd w0, w0, w1\nstr w0, [sp, #{}]",
                relative_stack_position, relative_stack_position
            ));
        }
        NodeType::SubEq => {
            scopes.push_to_scope(format!(
                "\nldr w1, [sp, #{}]\nsub w0, w0, w1\nstr w0, [sp, #{}]",
                relative_stack_position, relative_stack_position
            ));
        }
        NodeType::MulEq => {
            scopes.push_to_scope(format!(
                "\nldr w1, [sp, #{}]\nmul w0, w0, w1\nstr w0, [sp, #{}]",
                relative_stack_position, relative_stack_position
            ));
        }
        NodeType::DivEq => {
            scopes.push_to_scope(format!(
                "\nldr w1, [sp, #{}]\ndiv w0, w0, w1\nstr w0, [sp, #{}]",
                relative_stack_position, relative_stack_position
            ));
        }
        NodeType::BOrEq => {
            scopes.push_to_scope(format!(
                "\nldr w1, [sp, #{}]\nbor w0, w0, w1\nstr w0, [sp, #{}]",
                relative_stack_position, relative_stack_position
            ));
        }
        NodeType::BAndEq => {
            scopes.push_to_scope(format!(
                "\nldr w1, [sp, #{}]\nand w0, w0, w1\nstr w0, [sp, #{}]",
                relative_stack_position, relative_stack_position
            ));
        }
        NodeType::BXorEq => {
            scopes.push_to_scope(format!(
                "\nldr w1, [sp, #{}]\nxor w0, w0, w1\nstr w0, [sp, #{}]",
                relative_stack_position, relative_stack_position
            ));
        }
        _ => {
            panic!("Expected Assignment")
        }
    };
}

fn expr_code_gen(
    node: &TokenNode,
    stack_handler: &mut StackHandler,
    w: &mut i32,
    scopes: &mut ScopeHandler,
) {
    match &node.token {
        NodeType::NumLiteral(val) => {
            stack_handler.insert_expr_literal();
            scopes.push_to_scope(format!("\nstr #{val}, [sp, #16]"));
        }
        NodeType::Id(name) => {
            stack_handler.insert_expr_literal();
            let address = stack_handler.get_id(name).expect("Undefined identifier");
            scopes.push_to_scope(format!(
                "\nldr w{w}, [sp, #{address}]\nstr, w{w}, [sp, #{}]",
                stack_handler.furthest_offset
            ));
            switch!(*w);
        }
        _ => {
            expr_code_gen(
                &node.children.as_ref().unwrap()[0],
                stack_handler,
                w,
                scopes,
            );
            expr_code_gen(
                &node.children.as_ref().unwrap()[1],
                stack_handler,
                w,
                scopes,
            );
            stack_handler.furthest_offset += 32;
            scopes.push_to_scope(format!("\nldr w0, [sp, #16]\nldr w1, [sp, #32]"));
            match &node.token {
                NodeType::Add => {
                    scopes.push_to_scope(format!("\nadd w{w}, w0, w1").as_str());
                }
                NodeType::Sub => {
                    scopes.push_to_scope(format!("\nsub w{w}, w0, w1"));
                }
                NodeType::Div => {
                    scopes.push_to_scope(format!("\ndiv w{w}, w0, w1"));
                }
                NodeType::Mul => {
                    scopes.push_to_scope(format!("\nmul w{w}, w0, w1"));
                }
                NodeType::BAnd => {
                    scopes.push_to_scope(format!("\nand w{w}, w0, w1"));
                }
                NodeType::BOr => {
                    scopes.push_to_scope(format!("\nor w{w}, w0, w1"));
                }
                NodeType::BXor => {
                    scopes.push_to_scope(format!("\nxor w{w}, w0, w1"));
                }
                _ => panic!("Expected Expression"),
            };
            stack_handler.insert_expr_literal();
            scopes.push_to_scope(format!(
                "\nstr w{w}, [sp, #{}]",
                stack_handler.furthest_offset
            ));
            switch!(*w);
        }
    }
    scopes.push_to_scope(format!(
        "\nldr w{w}, [sp, #{}]",
        stack_handler.furthest_offset
    ));
    stack_handler.furthest_offset += 16;
}

pub fn if_code_gen(
    node: &TokenNode,
    stack_handler: &mut StackHandler,
    w: &mut i32,
    scopes: &mut ScopeHandler,
) {
    match &node.children {
        Some(children) => {
            // cmp x1, #n
            // beq label
            // node.children.unwrap()[0] is condition node, other child is scope
            let orig_scope = scopes.curr_scope;

            condition_expr_code_gen(&children[0], w, stack_handler, scopes);
            scope_code_gen(&children[1], stack_handler, w, scopes);

            scopes.curr_scope = orig_scope;
        }
        None => {
            panic!("Expected Condition");
        }
    };
}

pub fn condition_expr_code_gen(
    node: &TokenNode,
    w: &mut i32,
    stack_handler: &mut StackHandler,
    scopes: &mut ScopeHandler,
) {
    println!("condition expr node: {:?}", node.token);
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
            );
            condition_expr_code_gen(
                &node
                    .children
                    .as_ref()
                    .expect("more children in condition expr")[1],
                w,
                stack_handler,
                scopes,
            );
            scopes.push_to_scope(format!("\ncmp w0, w1\nbeq .L{}", scopes.curr_scope + 1).as_str());
            scopes.new_scope();
        }
        NodeType::Id(id) => {
            let relative_path = stack_handler.get_id(id).unwrap(); // relative path moves stack down without -

            scopes.push_to_scope(format!("\nldr w{}, [sp, #{}]", w, relative_path).as_str());
            switch!(*w);
            // scopes.push_to_scope(format!("\nadd sp, {}, sp", relative_path).as_str());
        }
        NodeType::NumLiteral(num) => {
            scopes.push_to_scope(format!("\nmov w{}, {}", w, num).as_str());
            switch!(*w);
        }
        _ => {
            panic!("Expected Condition");
        }
    };
    // code.push_str("\nbeq");
}

fn while_code_gen(
    node: &TokenNode,
    stack_handler: &mut StackHandler,
    w: &mut i32,
    scopes: &mut ScopeHandler,
) {
    match &node.children {
        Some(children) => {
            scopes.push_to_scope(format!("\nb .L{}", scopes.curr_scope + 1));
            scopes.new_scope();
            let anchor_scope = scopes.curr_scope;
            condition_expr_code_gen(&children[0], w, stack_handler, scopes);
            scope_code_gen(&children[1], stack_handler, w, scopes);
            remove_scope_ret(scopes);
            scopes.push_to_scope(format!("\nb .L{}", anchor_scope));
            scopes.curr_scope -= 1;
        }
        None => {
            panic!("Expected Condition")
        }
    }
}

// TODO: Fix the program so this isn't needed(maybe pass a ret flag into scope_code_gen)
fn remove_scope_ret(scopes: &mut ScopeHandler) {
    // Removes the last line which is ret
    let mut check = false;
    for i in (0..scopes.scopes[scopes.curr_scope].len() - 1).rev() {
        if scopes.scopes[scopes.curr_scope].chars().nth(i).unwrap() == '\n' {
            let len = scopes.scopes.clone()[scopes.curr_scope].len() - 2;
            scopes.scopes[scopes.curr_scope].truncate(len);
            if check {
                break;
            }
            check = true
        }
    }
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
