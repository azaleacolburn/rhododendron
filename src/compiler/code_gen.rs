use crate::compiler::parser::{NodeType, ScopeType, TokenNode};
use std::collections::HashMap;

// This is technically somehow working right now and I have no idea why, I think it does things
// backwards but it's ok
/// This struct manages the stack and all scopes
/// It stores the furthest offset from the base of the stack
/// This model works on the assumption that the base of the stack shifts around as well
/// So a store looks like this
/// "store x1, [sp, #{}]", (furthest_offset + sp_offset)

/// Reusable code snippets
// const NEW_SCOPE_BYTE_ALIGN: &str = "\n.balign 4\n.L{}:";

const MOV_EXPR_1: &str = "\nldr x19 [sp], #16";
const MOV_EXPR_2: &str = "\nldr x20 [sp], #16";
const ADD_EXPR: &str = "\nadd x19, x19, x20";

// const GET_ADR: &str = "\nldr {} [sp], #16"; // assuming it was 32 bit address formatting needed
// const STR_ADR: &str = "\nstr {}, [sp, #-16]!";

const CMP_EXPR: &str = "\ncmp x19, x20"; // comparison flag set in the processor status register

// const SYS_CALL_NUM: &str = "\nmov x0, {}"; // syscall_number
// const SYS_CALL_ARG1: &str = "\nmov x0, {}"; // etc
const SYS_CALL: &str = "\nsvc 0";

#[derive(Debug, Clone)]
pub struct SymbolTable {
    table: HashMap<String, i32>, // id, offsert from current stack frame base
    function_table: HashMap<String, String>, // function name, label number
    parent: Option<&'static SymbolTable>,
    children: Vec<SymbolTable>, // This probably won't work
    furthest_offset: i32,
}

impl SymbolTable {
    fn new(parent: Option<&SymbolTable>, arg_num: i32) -> SymbolTable {
        SymbolTable {
            table: HashMap::new(),
            function_table: HashMap::new(),
            parent,
            children: vec![],
            furthest_offset: 1 + arg_num, // stackframe_base, ret, a*. Measured in bytes (change if needed)
        }
    }

    fn add_child(&mut self, arg_num: i32) {
        let child = SymbolTable::new(Some(&self), arg_num);
        self.children.push(child);
    }

    fn get_id(&self, id: String) -> Option<&i32> {
        self.table.get(&id).to_owned()
    }

    fn new_id(&mut self, id: String, size: i32) {
        self.table.insert(id, self.furthest_offset);

        self.furthest_offset += size;
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
            sym_tree: SymbolTable::new(None, 0),
        }
    }

    /// Scopes

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

    /// Breaks

    fn insert_break(&mut self) {
        self.push_to_scope(format!("\nb .L{}", self.curr_scope + 1));
        self.new_scope();
    }

    fn new_break_anchor(&mut self) {
        self.break_anchors.push(self.curr_scope);
    }

    /// Frames + Symbol Tables

    /// This function must be breaking some borrow checker rule
    fn new_stack_frame(&mut self, arg_num: i32) {
        let new_st = SymbolTable::new(Some(&self.sym_tree), arg_num); // TODO: Check how this can be done, we might need to use box

        // FIXME: Finish to switch out new_st properly and make a fully-doubly linked list

        self.sym_tree = new_st;
    }

    // Panics if id doesn't exist
    fn get_id(&self, id: impl ToString) -> Option<&i32> {
        let sym_res = self.sym_tree.get_id(id.to_string());

        if sym_res.is_none() {
            return match self.sym_tree.parent {
                Some(parent) => Some(parent.get_id(id.to_string()).expect("Symbol not found")),
                None => return None,
            };
        }

        sym_res
    }

    fn new_id(&mut self, id: impl ToString, size: i32) {
        self.sym_tree.new_id(id.to_string(), size);
    }

    fn new_16(&mut self, id: impl ToString) {
        self.sym_tree.new_id(id.to_string(), 16);
    }

    fn new_expr_literal(&mut self) {
        self.sym_tree.furthest_offset += 16; // Make some macro mechanism to force this to be freed
    }

    fn new_function(&mut self, name: String, arg_num: i32) {
        self.sym_tree.new_id(name, 16); // 16 bit instuction pointer
        self.new_stack_frame(arg_num);
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

pub fn scope_code_gen(node: &TokenNode, handler: &mut Handler, x: &mut i32, scope_type: ScopeType) {
    println!("Scope Node: {:?}", node);
    for child_node in node.children.as_ref().expect("Scope to have children") {
        match &child_node.token {
            NodeType::Declaration(name) => {
                declare_code_gen(
                    &child_node,
                    handler,
                    x,
                    name.as_ref()
                        .expect("valid name to have been given")
                        .clone(),
                );
            }
            NodeType::Assignment(_) => {
                assignment_code_gen(&child_node, handler, x);
            }
            NodeType::If => {
                if_code_gen(&child_node, handler, x);
            }
            NodeType::While => {
                while_code_gen(&child_node, handler, x);
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
pub fn declare_code_gen(node: &TokenNode, handler: &mut Handler, x: &mut i32, name: String) {
    println!("Declare Node: {:?}", node.token);
    expr_code_gen(
        &node.children.as_ref().expect("Node to have children")[0],
        handler,
        w,
    );
    handler.new_16(&name);
    handler.push_to_scope(
        format!(
            "\nstr x19, [sp, #{}]",
            handler
                .get_id(&name)
                .expect("variable wasn't pushed to stack")
        )
        .as_str(),
    );
}

pub fn assignment_code_gen(node: &TokenNode, handler: &mut Handler, x: &mut i32) {
    println!("assignment node: {:?}", node.token);

    let name = match &node.token {
        NodeType::Assignment(name) => name.as_ref().unwrap(),
        _ => panic!("Given given invalid assignment node"),
    };

    expr_code_gen(&node.children.as_ref().unwrap()[1], handler, x);
    let relative_stack_position = handler.get_id(name).expect("Undefined Identifier");

    match node.children.as_ref().unwrap()[0].token {
        NodeType::Eq => handler.push_to_scope("\nstr x19, [sp, #-16]!"),
        NodeType::AddEq => {
            handler.push_to_scope("\nldr x20, [sp], #16\nadd x19, x19, x20\nstr x19, [sp, #-16]!")
        }
        NodeType::SubEq => {
            handler.push_to_scope("\nldr x20, [sp], #16\nsub x19, x19, x20\nstr x19, [sp, #-16]!")
        }
        NodeType::MulEq => {
            handler.push_to_scope("\nldr x20, [sp], #16\nmul x19, x19, x20\nstr x19, [sp, #-16]!")
        }
        NodeType::DivEq => {
            handler.push_to_scope("\nldr x20, [sp], #16\ndiv x19, x19, x20\nstr x19, [sp, #-16]!")
        }
        NodeType::BOrEq => {
            handler.push_to_scope("\nldr x20, [sp], #16\nbor x19, x19, x20\nstr x19, [sp, #-16]!")
        }
        NodeType::BAndEq => {
            handler.push_to_scope("\nldr x20, [sp], #16\nand x19, x19, x20\nstr x19, [sp, #-16]!")
        }
        NodeType::BXorEq => {
            handler.push_to_scope("\nldr x20, [sp], #16\nxor x19, x19, x20\nstr x19, [sp, #-16]!")
        }
        _ => {
            panic!("Expected Assignment")
        }
    };
}

fn expr_code_gen(node: &TokenNode, handler: &mut Handler, x: &mut i32) {
    match &node.token {
        NodeType::NumLiteral(val) => {
            handler.new_expr_literal();
            handler.push_to_scope(format!("\nstr #{val}, [sp, #-16]!"));
        }
        NodeType::Id(name) => {
            handler.new_expr_literal();
            let offset = handler.get_id(name).expect("Undefined identifier");
            handler.push_to_scope(format!(
                "\nldr x{x}, [r29] #{offset}\nstr, x{x}, [sp, #-16]!",
            ));
            switch!(*x);
        }
        _ => {
            expr_code_gen(&node.children.as_ref().unwrap()[0], handler, x);
            expr_code_gen(&node.children.as_ref().unwrap()[1], handler, x);
            match &node.token {
                NodeType::Add => {
                    handler.push_to_scope(format!("\nadd x{x}, x19, x20").as_str());
                }
                NodeType::Sub => {
                    handler.push_to_scope(format!("\nsub x{x}, x19, x20"));
                }
                NodeType::Div => {
                    handler.push_to_scope(format!("\ndiv x{x}, x19, x20"));
                }
                NodeType::Mul => {
                    handler.push_to_scope(format!("\nmul x{x}, x19, x20"));
                }
                NodeType::BAnd => {
                    handler.push_to_scope(format!("\nand x{x}, x19, x20"));
                }
                NodeType::BOr => {
                    handler.push_to_scope(format!("\nor x{x}, x19, x20"));
                }
                NodeType::BXor => {
                    handler.push_to_scope(format!("\nxor x{x}, x19, x20"));
                }
                _ => panic!("Expected Expression"),
            };
            handler.new_expr_literal();
            handler.push_to_scope("\nstr x{x}, [sp, #16]!");
            switch!(*x);
        }
    }
    handler.push_to_scope("\nldr x{x}, [sp, #{}]");
    handler.sym_tree.furthest_offset += 16;
}

pub fn function_declare_code_gen(node: &TokenNode, handler: &mut Handler, x: &mut i32) {
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

pub fn function_call_code_gen(node: &TokenNode, handler: &mut Handler, x: &mut i32) {}

pub fn if_code_gen(node: &TokenNode, handler: &mut Handler, x: &mut i32) {
    match &node.children {
        Some(children) => {
            // cmp x1, #n
            // beq label
            // node.children.unwrap()[0] is condition node, other child is scope

            condition_expr_code_gen(&children[0], handler, x);
            scope_code_gen(&children[1], handler, w, ScopeType::If);
        }
        None => {
            panic!("Expected Condition");
        }
    };
}

pub fn condition_expr_code_gen(node: &TokenNode, handler: &mut Handler, x: &mut i32) {
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
                format!("\ncmp x19, x20\nbleq .L{}\nret", handler.curr_scope + 1).as_str(),
            );
            handler.new_scope();
        }
        NodeType::Id(id) => {
            let relative_path = handler.get_id(id).unwrap(); // relative path moves stack down without -

            handler.push_to_scope(format!("\nldr w{}, [sp, #{}]", w, relative_path).as_str());
            switch!(*x);
            // scopes.push_to_scope(format!("\nadd sp, {}, sp", relative_path).as_str());
        }
        NodeType::NumLiteral(num) => {
            handler.push_to_scope(format!("\nmov w{}, {}", w, num).as_str());
            switch!(*x);
        }
        _ => {
            panic!("Expected Condition");
        }
    };
    // code.push_str("\nbeq");
}

fn while_code_gen(node: &TokenNode, handler: &mut Handler, x: &mut i32) {
    match &node.children {
        Some(children) => {
            handler.push_to_scope(format!("\nb .L{}", handler.curr_scope + 1));
            handler.new_break_anchor();
            handler.new_scope();

            condition_expr_code_gen(&children[0], handler, x);
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
