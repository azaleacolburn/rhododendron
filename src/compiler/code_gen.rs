use std::collections::HashMap;

use crate::compiler::parser::{NodeType, ScopeType, TokenNode};

// This is technically somehow working right now and I have no idea why, I think it does things
// backwards but it's ok
/// This struct manages the stack and all scopes
/// It stores the furthest offset from the base of the stack
/// This model works on the assumption that the base of the stack shifts around as well
/// So a store looks like this
/// "store x1, [sp, #{}]", (furthest_offset + sp_offset)

pub struct SymbolTable {
    table: HashMap<String, i32>,
    parent: Option<&'static SymbolTable>,
    children: Vec<SymbolTable>,
}

impl SymbolTable {
    fn new(parent: Option<&SymbolTable>) -> SymbolTable {
        SymbolTable {
            table: HashMap::new(),
            parent,
            children: vec![],
        }
    }

    fn add_child(&mut self) {
        let child = SymbolTable::new(Some(&self));
        self.children.push(child);
    }

    fn get_id(&self, id: String) -> Option<&i32> {
        self.table.get(&id).to_owned()
    }
}

pub struct Handler {
    scopes: Vec<String>,
    curr_scope: usize,
    sym_tree: SymbolTable,
    break_anchors: Vec<usize>, // each number represents the insertuction pointer that can be broken to from the current scope
}

impl Handler {
    fn new() -> Self {
        Handler {
            scopes: vec![String::from("\n.global _main\nmain:")],
            curr_scope: 0,
            break_anchors: vec![],
            sym_tree: SymbolTable::new(None),
        }
    }

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

    fn insert_break(&mut self) {
        self.push_to_scope(format!("\nb .L{}", self.curr_scope + 1));
        self.new_scope();
    }

    fn new_break_anchor(&mut self) {
        self.break_anchors.push(self.curr_scope);
    }

    fn get_id(&self, id: impl ToString) -> Result<&i32> {
        let sym_res = self.sym_tree.get_id(id.to_string());

        if sym_res.is_none() {
            return match self.sym_tree.parent {
                Some(parent) => parent.get_id(&id),
                None => return Err() // TODO: Return an error because the sym doesn't exist
            }
            self.sym_tree.parent.get_id(&id);
        }

        Ok(sym_res)
    }

    fn insert(&mut self, id: impl ToString, size: i32) {
        self.furthest_offset -= size;
        self.stack_handler
            .insert(id.to_string(), self.furthest_offset);
    }

    fn insert_new_16(&mut self, id: impl ToString) {
        self.insert(id, 16);
    }

    fn insert_expr_literal(&mut self) {
        self.furthest_offset -= 16;
    }

    fn insert_function(&mut self, name: String) {
        self.function_handler.insert(name, self.curr_scope as i32);
    }
}

macro_rules! switch {
    ($x:expr) => {
        $x = 1 - $x
    };
}

// Ask Andrew how to enter into main after trying to do it with code_gen, not parsing
pub fn main(node: &TokenNode) -> String {
    let mut handler = Handler::new();
    println!("In code gen");
    // use this later
    // var_name : pos_on_stack
    // Stores the variable name and their absolute position on the stack

    // Stores the variable name and their relative position on the stack(from the top of the stack
    // at the begining of runtime). This should be on a per_scope level
    println!("{:?}", node.token);
    println!("{:?}", node.children.as_ref().unwrap()[0].token);
    if node.token == NodeType::Program {
        scope_code_gen(
            &node.children.as_ref().unwrap()[0],
            &mut handler,
            &mut 0,
            ScopeType::Program,
        );
    }
    handler.curr_scope = 0;
    handler.push_to_scope("\nmov x7, #1\nmov x0, #0\nsvc 0");

    handler.format_scopes()
    // code.trim().to_string()
}

pub fn scope_code_gen(node: &TokenNode, handler: &mut Handler, w: &mut i32, scope_type: ScopeType) {
    println!("Scope Node: {:?}", node);
    for child_node in node.children.as_ref().expect("Scope to have children") {
        match &child_node.token {
            NodeType::Declaration(name) => {
                declare_code_gen(
                    &child_node,
                    handler,
                    w,
                    name.as_ref()
                        .expect("valid name to have been given")
                        .clone(),
                );
            }
            NodeType::Assignment(_) => {
                assignment_code_gen(&child_node, handler, w);
            }
            NodeType::If => {
                if_code_gen(&child_node, handler, w);
            }
            NodeType::While => {
                while_code_gen(&child_node, handler, w);
            }
            NodeType::Loop => {}
            NodeType::FunctionCall(_id) => {}
            NodeType::Break => handler.insert_break(),
            _ => {}
        };
    }
    if scope_type == ScopeType::While {
        handler.push_to_scope(format!("\nb .L{}", handler.curr_scope + 1));
    }
    handler.push_to_scope("\nret");
}

/// Returns the generated code
/// Modifies the sp
pub fn declare_code_gen(node: &TokenNode, handler: &mut Handler, w: &mut i32, name: String) {
    println!("Declare Node: {:?}", node.token);
    expr_code_gen(
        &node.children.as_ref().expect("Node to have children")[0],
        handler,
        w,
    );
    handler.insert_new_16(&name);
    handler.push_to_scope(
        format!(
            "\nstr w0, [sp, #{}]",
            handler
                .get_id(&name)
                .expect("variable wasn't pushed to stack")
        )
        .as_str(),
    );
}

pub fn assignment_code_gen(node: &TokenNode, handler: &mut Handler, w: &mut i32) {
    println!("assignment node: {:?}", node.token);

    let name = match &node.token {
        NodeType::Assignment(name) => name.as_ref().unwrap(),
        _ => panic!("Given given invalid assignment node"),
    };

    expr_code_gen(&node.children.as_ref().unwrap()[1], handler, w);
    let relative_stack_position = handler.get_id(name).expect("Undefined Identifier");

    match node.children.as_ref().unwrap()[0].token {
        NodeType::Eq => {
            handler.push_to_scope(format!("\nstr w0, [sp, #{}]", relative_stack_position));
        }
        NodeType::AddEq => {
            handler.push_to_scope(format!(
                "\nldr w1, [sp, #{}]\nadd w0, w0, w1\nstr w0, [sp, #{}]",
                relative_stack_position, relative_stack_position
            ));
        }
        NodeType::SubEq => {
            handler.push_to_scope(format!(
                "\nldr w1, [sp, #{}]\nsub w0, w0, w1\nstr w0, [sp, #{}]",
                relative_stack_position, relative_stack_position
            ));
        }
        NodeType::MulEq => {
            handler.push_to_scope(format!(
                "\nldr w1, [sp, #{}]\nmul w0, w0, w1\nstr w0, [sp, #{}]",
                relative_stack_position, relative_stack_position
            ));
        }
        NodeType::DivEq => {
            handler.push_to_scope(format!(
                "\nldr w1, [sp, #{}]\ndiv w0, w0, w1\nstr w0, [sp, #{}]",
                relative_stack_position, relative_stack_position
            ));
        }
        NodeType::BOrEq => {
            handler.push_to_scope(format!(
                "\nldr w1, [sp, #{}]\nbor w0, w0, w1\nstr w0, [sp, #{}]",
                relative_stack_position, relative_stack_position
            ));
        }
        NodeType::BAndEq => {
            handler.push_to_scope(format!(
                "\nldr w1, [sp, #{}]\nand w0, w0, w1\nstr w0, [sp, #{}]",
                relative_stack_position, relative_stack_position
            ));
        }
        NodeType::BXorEq => {
            handler.push_to_scope(format!(
                "\nldr w1, [sp, #{}]\nxor w0, w0, w1\nstr w0, [sp, #{}]",
                relative_stack_position, relative_stack_position
            ));
        }
        _ => {
            panic!("Expected Assignment")
        }
    };
}

fn expr_code_gen(node: &TokenNode, handler: &mut Handler, w: &mut i32) {
    match &node.token {
        NodeType::NumLiteral(val) => {
            handler.insert_expr_literal();
            handler.push_to_scope(format!("\nstr #{val}, [sp, #-16]"));
        }
        NodeType::Id(name) => {
            handler.insert_expr_literal();
            let address = handler.get_id(name).expect("Undefined identifier");
            handler.push_to_scope(format!(
                "\nldr w{w}, [sp, #{address}]\nstr, w{w}, [sp, #{}]",
                handler.furthest_offset
            ));
            switch!(*w);
        }
        _ => {
            expr_code_gen(&node.children.as_ref().unwrap()[0], handler, w);
            expr_code_gen(&node.children.as_ref().unwrap()[1], handler, w);
            handler.furthest_offset += 32;
            handler.push_to_scope(format!(
                "\nldr w0, [sp, #{}]\nldr w1, [sp, #{}]",
                handler.furthest_offset - 16,
                handler.furthest_offset
            ));
            match &node.token {
                NodeType::Add => {
                    handler.push_to_scope(format!("\nadd w{w}, w0, w1").as_str());
                }
                NodeType::Sub => {
                    handler.push_to_scope(format!("\nsub w{w}, w0, w1"));
                }
                NodeType::Div => {
                    handler.push_to_scope(format!("\ndiv w{w}, w0, w1"));
                }
                NodeType::Mul => {
                    handler.push_to_scope(format!("\nmul w{w}, w0, w1"));
                }
                NodeType::BAnd => {
                    handler.push_to_scope(format!("\nand w{w}, w0, w1"));
                }
                NodeType::BOr => {
                    handler.push_to_scope(format!("\nor w{w}, w0, w1"));
                }
                NodeType::BXor => {
                    handler.push_to_scope(format!("\nxor w{w}, w0, w1"));
                }
                _ => panic!("Expected Expression"),
            };
            handler.insert_expr_literal();
            handler.push_to_scope(format!("\nstr w{w}, [sp, #{}]", handler.furthest_offset));
            switch!(**w);
        }
    }
    handler.push_to_scope(format!("\nldr w{w}, [sp, #{}]", handler.furthest_offset));
    handler.furthest_offset += 16;
}

pub fn function_declare_code_gen(node: &TokenNode, handler: &mut Handler, w: &mut i32) {
    if node.children.is_none() {
        panic!("Function children must be some");
    }
    handler.new_scope();

    if let NodeType::FunctionDecaration(name) = &node.token {
        handler
            .function_handler
            .insert(name, handler.curr_scope as i32);
        for child in node.children.as_ref().unwrap().iter() {
            // TODO: Simplify return types of a function with the scope return type
            if let NodeType::Type(_) = child.token {
                break;
            } else if let NodeType::Scope(_) = child.token {
                break;
            }
        }
    }
}

pub fn function_call_code_gen(node: &TokenNode, handler: &mut Handler, w: &mut i32) {}

pub fn if_code_gen(node: &TokenNode, handler: &mut Handler, w: &mut i32) {
    match &node.children {
        Some(children) => {
            // cmp x1, #n
            // beq label
            // node.children.unwrap()[0] is condition node, other child is scope

            condition_expr_code_gen(&children[0], handler, w);
            scope_code_gen(&children[1], handler, w, ScopeType::If);
        }
        None => {
            panic!("Expected Condition");
        }
    };
}

pub fn condition_expr_code_gen(node: &TokenNode, handler: &mut Handler, w: &mut i32) {
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
                handler,
                w,
            );
            condition_expr_code_gen(
                &node
                    .children
                    .as_ref()
                    .expect("more children in condition expr")[1],
                handler,
                w,
            );
            handler.push_to_scope(
                format!("\ncmp w0, w1\nbleq .L{}\nret", handler.curr_scope + 1).as_str(),
            );
            handler.new_scope();
        }
        NodeType::Id(id) => {
            let relative_path = handler.get_id(id).unwrap(); // relative path moves stack down without -

            handler.push_to_scope(format!("\nldr w{}, [sp, #{}]", w, relative_path).as_str());
            switch!(*w);
            // scopes.push_to_scope(format!("\nadd sp, {}, sp", relative_path).as_str());
        }
        NodeType::NumLiteral(num) => {
            handler.push_to_scope(format!("\nmov w{}, {}", w, num).as_str());
            switch!(*w);
        }
        _ => {
            panic!("Expected Condition");
        }
    };
    // code.push_str("\nbeq");
}

fn while_code_gen(node: &TokenNode, handler: &mut Handler, w: &mut i32) {
    match &node.children {
        Some(children) => {
            handler.push_to_scope(format!("\nb .L{}", handler.curr_scope + 1));
            handler.new_break_anchor();
            handler.new_scope();

            condition_expr_code_gen(&children[0], handler, w);
            scope_code_gen(&children[1], handler, w, ScopeType::While);
            handler.new_scope();
        }
        None => {
            panic!("Expected Condition")
        }
    }
}

fn asm_code_gen(node: &TokenNode, handler: &mut Handler) {
    match &node.token {
        NodeType::Asm(str) => handler.push_to_scope(str),
        _ => panic!("Expected Asm"),
    }
}

// TODO: Fix the program so this isn't needed(maybe pass a ret flag into scope_code_gen)
fn remove_scope_ret(handler: &mut Handler) {
    // Removes the last line which is ret
    let mut check = false;
    for i in (0..handler.scopes[handler.curr_scope].len() - 1).rev() {
        if handler.scopes[handler.curr_scope].chars().nth(i).unwrap() == '\n' {
            let len = handler.scopes.clone()[handler.curr_scope].len() - 2;
            handler.scopes[handler.curr_scope].truncate(len);
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
