use crate::compiler::parser::{AssignmentOpType, NodeType, ScopeType, TokenNode};
use std::collections::HashMap;

use super::lexer::RhTypes;

// This is technically somehow working right now and I have no idea why, I think it does things
// backwards but it's ok
/// This struct manages the stack and all scopes
/// It stores the furthest offset from the base of the stack
/// This model works on the assumption that the base of the stack shifts around as well
/// So a store looks like this
/// "store x1, [sp, #{}]", (furthest_offset + sp_offset)

/// Reusable code snippets
// const NEW_SCOPE_BYTE_ALIGN: &str = "\n.balign 4\n.L{}:";

const _MOV_EXPR_1: &str = "\nldr x19 [sp], #4";
const _MOV_EXPR_2: &str = "\nldr x20 [sp], #4";
const _ADD_EXPR: &str = "\nadd x19, x19, x20";

// const GET_ADR: &str = "\nldr {} [sp], #16"; // assuming it was 32 bit address formatting needed
// const STR_ADR: &str = "\nstr {}, [sp, #-16]!";

const _CMP_EXPR: &str = "\ncmp x19, x20"; // comparison flag set in the processor status register
                                          // const SYS_CALL_NUM: &str = "\nmov x0, {}"; // syscall_number
                                          // const SYS_CALL_ARG1: &str = "\nmov x0, {}"; // etc
const _SYS_CALL: &str = "\nsvc 0";
#[derive(Debug, Clone)]
pub struct FunctionSig {
    args: Vec<String>, // ids in order
    label: String,
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    table: HashMap<String, i32>, // id, offsert from current stack frame base
    function_table: HashMap<String, FunctionSig>, // function name, label
    parent: Option<Box<SymbolTable>>,
    furthest_offset: i32,
}

impl SymbolTable {
    fn new(parent: Option<Box<SymbolTable>>, arg_num: i32) -> SymbolTable {
        SymbolTable {
            table: HashMap::new(),
            function_table: HashMap::new(),
            parent,
            furthest_offset: 4 + arg_num, // stackframe_base(0), ret(4), a*. Measured in bytes (change if needed)
        }
    }

    fn get_id(&self, id: String) -> Option<&i32> {
        self.table.get(&id).to_owned()
    }

    fn new_id(&mut self, id: String, size: i32) {
        self.table.insert(id, -self.furthest_offset);

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
        let mut handler = Handler {
            scopes: vec![String::from("\n.global .main\n.align 4\n")],
            curr_scope: 0,
            break_anchors: vec![],
            sym_tree: SymbolTable::new(None, 0),
        };

        handler.curr_scope = handler.scopes.len();
        handler.scopes.push(String::new());

        handler
            .push_to_scope("\n.main:\n; x29 is our sfb\n;x15 is our sp\nmov x29, sp\nmov x15, sp");

        handler
    }

    /// Scopes

    fn new_scope(&mut self) {
        self.curr_scope = self.scopes.len();
        self.scopes.push(format!("\n.L{}:", self.curr_scope));
    }

    #[allow(dead_code)]
    fn prev_scope(&mut self) {
        self.curr_scope -= 1;
    }

    fn push_to_scope(&mut self, string: impl ToString) {
        self.scopes[self.curr_scope].push_str(string.to_string().as_str())
    }

    pub fn format_scopes(&self) -> String {
        let mut ret = String::new();

        let mut lines = self.scopes[0].lines();
        ret.push_str(format!("{}\n", lines.next().unwrap()).as_str());
        ret.push_str(format!("{}\n", lines.next().unwrap()).as_str());
        ret.push_str(format!("{}\n", lines.next().unwrap()).as_str());

        for scope in 1..self.scopes.len() {
            let mut lines = self.scopes[scope].lines();
            ret.push_str(format!("{}\n", lines.next().unwrap()).as_str());
            ret.push_str(format!("{}\n", lines.next().unwrap()).as_str());
            //ret.push_str(format!("{}\n", lines.next().unwrap()).as_str());

            for line in lines {
                ret.push_str(format!("\t{}\n", line).as_str());
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
                println!("\t{}", line);
            }
        }
    }

    /// Breaks

    fn insert_break(&mut self) {
        self.push_to_scope(format!("\n; break statement\nb .L{}", self.curr_scope + 1));
        self.new_scope();
    }

    fn new_break_anchor(&mut self) {
        self.break_anchors.push(self.curr_scope);
    }

    /// Frames + Symbol Tables

    /// This function must be breaking some borrow checker rule
    fn new_stack_frame(&mut self, arg_num: i32) {
        let new_st = SymbolTable::new(Some(Box::new(self.sym_tree.clone())), arg_num); // TODO: Check how this can be done, we might need to use bo

        self.sym_tree = new_st;
    }

    // Panics if id doesn't exist
    fn get_id(&self, id: impl ToString) -> Option<&i32> {
        let sym_res = self.sym_tree.get_id(id.to_string());

        if sym_res.is_none() {
            return match &self.sym_tree.parent {
                Some(parent) => Some(parent.get_id(id.to_string()).expect("Symbol not found")),
                None => return None,
            };
        }

        sym_res
    }

    fn new_id(&mut self, id: impl ToString, size: i32) {
        self.sym_tree.new_id(id.to_string(), size);
    }

    fn new_4_byte(&mut self, id: impl ToString) {
        self.sym_tree.new_id(id.to_string(), 4);
    }

    fn new_function(&mut self, name: impl ToString, args: Vec<String>, scope: i32) {
        self.sym_tree.new_id(name.to_string(), 4); // 16 bit instuction pointer
        self.new_stack_frame(args.len() as i32);
        let new_scope_label = format!(".L{}", scope);
        let function_sig = FunctionSig {
            args,
            label: new_scope_label, // figure out how to determine label (probably based on a property in handler)
        };
        self.sym_tree
            .function_table
            .insert(name.to_string(), function_sig);
    }
}

// Ask Andrew how to enter into main after trying to do it with code_gen, not parsing
pub fn main(node: &TokenNode) -> String {
    let mut handler = Handler::new();
    // use this later
    // var_name : pos_on_stack
    // Stores the variable name and their absolute position on the stack

    // Stores the variable name and their relative position on the stack(from the top of the stack
    // at the begining of runtime). This should be on a per_scope level
    if node.token == NodeType::Program {
        scope_code_gen(
            &node.children.as_ref().unwrap()[0],
            &mut handler,
            ScopeType::Program,
        );
    }
    handler.push_to_scope("\n\n; exit program gracefully\nmov x0, #0\nmov x16, #1\nsvc #0x80");

    handler.format_scopes()
    // code.trim().to_string()
}

pub fn scope_code_gen(node: &TokenNode, handler: &mut Handler, scope_type: ScopeType) {
    println!("Scope Node: {:?}", node);
    for child_node in node.children.as_ref().expect("Scope to have children") {
        match &child_node.token {
            NodeType::Declaration(arg) => {
                declare_code_gen(&child_node, handler, arg.0.clone(), arg.1.clone());
            }
            NodeType::Assignment(_) => {
                assignment_code_gen(&child_node, handler);
            }
            NodeType::If => {
                if_code_gen(&child_node, handler);
            }
            NodeType::While => {
                while_code_gen(&child_node, handler);
            }
            NodeType::Loop => {}
            NodeType::FunctionCall(id) => {
                function_call_code_gen(&child_node, handler, id.to_string())
            }
            NodeType::FunctionDecaration(id) => {
                function_declare_code_gen(&child_node, handler, id.to_string())
            }
            NodeType::Break => handler.insert_break(),
            NodeType::Return => {
                if scope_type == ScopeType::Function {
                    return return_statement_code_gen(&child_node, handler);
                }
            }
            NodeType::Asm(str) => asm_code_gen(&child_node, handler, str.to_string()),
            NodeType::PutChar => putchar_code_gen(&child_node, handler),
            NodeType::Assert => assert_code_gen(&child_node, handler),
            _ => {}
        };
    }
    if scope_type == ScopeType::While {
        handler.push_to_scope(format!(
            "\n; break statement\nb .L{}",
            handler.curr_scope + 1
        ));
    }
}

/// Returns the generated code
/// Modifies the sp
pub fn declare_code_gen(node: &TokenNode, handler: &mut Handler, name: String, _t: RhTypes) {
    println!("Declare Node: {:?}", node.token);
    println!("Node children: {:?}", node.children);
    handler.push_to_scope("\n\n; variable declaration");
    expr_code_gen(
        &node.children.as_ref().expect("Node to have children")[0],
        handler,
        9,
    );
    handler.new_4_byte(&name);
    handler.push_to_scope("\n");
    //handler.push_to_scope(format!("\nstr x19, [sp, #-4]!"));
}

pub fn assignment_code_gen(node: &TokenNode, handler: &mut Handler) {
    handler.push_to_scope("\n; variable assignment");
    println!("Assignment Node: {:?}", node.token);

    let (name, op) = match &node.token {
        NodeType::Assignment(name) => name,
        _ => panic!("Given given invalid assignment node"),
    };
    let relative_stack_position = handler.get_id(name).expect("Undefined Identifier").clone();

    expr_code_gen(&node.children.as_ref().unwrap()[0], handler, 9);
    handler.push_to_scope("\nldr x9, [x28], #4");

    // this load the old value in case we need it
    if *op != AssignmentOpType::Eq {
        handler.push_to_scope(format!("\nldr x10, [x29, #{relative_stack_position}]"));
        match op {
            AssignmentOpType::AddEq => handler.push_to_scope("\nadd x9, x9, x10"),
            AssignmentOpType::SubEq => handler.push_to_scope("\nsub x9, x9, x10"),
            AssignmentOpType::MulEq => handler.push_to_scope("\nmul x9, x9, x10"),
            AssignmentOpType::DivEq => handler.push_to_scope("\ndiv x9, x9, x10"),
            AssignmentOpType::BOrEq => handler.push_to_scope("\nbor x9, x9, x10"),
            AssignmentOpType::BAndEq => handler.push_to_scope("\nand x9, x9, x10"),
            AssignmentOpType::BXorEq => handler.push_to_scope("\nxor x9, x9, x10"),
            _ => panic!("Unexpected Operator"),
        };
    }

    handler.push_to_scope(format!("\nstr x9, [x29, #{relative_stack_position}]!"));
}

// Leaves the result on TOS
fn expr_code_gen(node: &TokenNode, handler: &mut Handler, x: i32) {
    match &node.token {
        NodeType::NumLiteral(val) => {
            handler.push_to_scope(format!("\nmov x{x}, #{val}"));
            handler.push_to_scope(format!("\nstr x{x}, [x15, #-4]!"));
        }
        NodeType::Id(name) => {
            let offset = handler.get_id(name).expect("Undefined identifier");
            handler.push_to_scope(format!(
                "\nldr x{x}, [x29, #{offset}]\nstr x{x}, [x15, #-4]!",
            ));
        }
        _ => {
            expr_code_gen(&node.children.as_ref().unwrap()[0], handler, 19);
            expr_code_gen(&node.children.as_ref().unwrap()[1], handler, 20);
            handler.push_to_scope("\n\n; load from stack\nldr x9, [x15], #4\nldr x10, [x15], #4");
            match &node.token {
                NodeType::Add => handler.push_to_scope(format!("\nadd x9, x9, x10")),
                NodeType::Sub => handler.push_to_scope(format!("\nsub x9, x9, x10")),
                NodeType::Div => handler.push_to_scope(format!("\ndiv x9, x9, x10")),
                NodeType::Mul => handler.push_to_scope(format!("\nmul x9, x9, x10")),
                NodeType::BAnd => handler.push_to_scope(format!("\nand x9, x9, x10")),
                NodeType::BOr => handler.push_to_scope(format!("\nor x9, x9, x10")),
                NodeType::BXor => handler.push_to_scope(format!("\nxor x9, x9, x10")),
                _ => panic!("Expected Expression"),
            };
            handler.push_to_scope("\nstr x9, [x15, #-4]!");
        }
    }
}

pub fn function_declare_code_gen(node: &TokenNode, handler: &mut Handler, name: String) {
    if node.children.is_none() {
        panic!("Function children must be some");
    }

    let children = node.children.as_ref().unwrap();
    handler.new_scope();
    handler.push_to_scope("\n; function declaration");
    let function_scope = handler.curr_scope;
    let mut args: Vec<String> = vec![];

    for child in children.iter() {
        if let NodeType::Declaration((id, t)) = &child.token {
            println!("Function Declare Argument Found");
            let size = match t {
                RhTypes::Char => 4,
                RhTypes::Int => 4,
            };
            handler.new_id(id, size); // This shuld be enough since each arg should be on the top of the stack
                                      // TODO: figure out what other code needs to go here (id any)
            args.push(id.clone());
        } else if let NodeType::Scope(_) = child.token {
            println!("SCOPE TIME");
            scope_code_gen(child, handler, ScopeType::Function);
            break;
        }
    }
    handler.push_to_scope("\n\n; x15 <- x29\n; x29 <- &old_sfb\nmov x15, x29\nldr x29, [x29]\nret");
    handler.new_function(name.clone(), args, function_scope as i32);
    handler.curr_scope = function_scope - 1;
}

pub fn function_call_code_gen(node: &TokenNode, handler: &mut Handler, name: String) {
    // This assembly should:
    // Store the address of the current stack fb on the top of the stack
    // Decrement the sp by 32
    // load the address of the new stack frame base into the sfb register
    handler.push_to_scope("\n\n; place address of previous sfb on stack as the value of the new sfb\nstr x29, [x15, #-4]!"); // might change, check pointer size

    println!("Function call node\n");

    node.print(&mut 0);

    println!("");

    if node.children.is_none() {
        panic!("Function has no children");
    }

    let children = node.children.as_ref().unwrap();

    // This asm should:
    // Store each 16-bit argument on the stack
    // Store the 16-bit size of the return value on the stack

    let function_sig = handler
        .sym_tree
        .function_table
        .get(&name)
        .expect("Invalid function name")
        .clone();
    handler.push_to_scope("\nldr x10, [x29]"); // x29 contains previous sfb address
                                               // Place the return label on the stack
    handler.push_to_scope(format!(
        "\nmov x9, #{}\nstr x9, [x15, #-4]!",
        handler.curr_scope
    ));
    println!("Node children {:?}", children);
    println!("Function sig: {:?}", function_sig);
    for i in 1..children.len() {
        match children[i].clone().token {
            NodeType::Id(_id) => {
                let arg_name = function_sig.args[i].clone();
                let offset = *handler
                    .get_id(arg_name)
                    .expect("Invalid id: INTERNAL FUNCTION ERROR");
                handler.push_to_scope(format!("\nldr x9, [x29], #{offset}\nstr x9, [x15, #-4]!"));
                // validate this code
            }
            NodeType::NumLiteral(num) => {
                handler.push_to_scope(format!("\nmov x9, #{num}\nstr x19, [x15, #-4]!"))
            }
            NodeType::Type(t) => {
                let size = match t {
                    RhTypes::Int => 4,
                    RhTypes::Char => 4,
                };
                handler.push_to_scope(format!("\nmov x9, #{size}\nstr x9, [x15, #-4]!"))
                // this does not need to be 16 but is for convinience
            }
            _ => panic!("Expected ID or literal"),
        }
    }
    handler.push_to_scope(format!("\nb {}", function_sig.label));
}

pub fn if_code_gen(node: &TokenNode, handler: &mut Handler) {
    handler.push_to_scope("\n\n; if statement");
    match &node.children {
        Some(children) => {
            // cmp x1, #n
            // beq label
            // node.children.unwrap()[0] is condition node, other child is scope

            condition_expr_code_gen(&children[0], handler, 9);
            handler.push_to_scope("\n; scope of if statement");
            scope_code_gen(&children[1], handler, ScopeType::If);
            handler.curr_scope -= 1;
        }
        None => {
            panic!("Expected Condition");
        }
    };
}

pub fn condition_expr_code_gen(node: &TokenNode, handler: &mut Handler, x: i32) {
    println!("condition expr node: {:?}", node.token);
    let children: Vec<TokenNode> = node.children.as_ref().unwrap_or(&Vec::new()).to_vec();
    match &node.token {
        NodeType::AndCmp => {
            condition_expr_code_gen(&children[0], handler, 9);
            condition_expr_code_gen(&children[1], handler, 10);
        }
        // This makes putting booleans into conditions illegal
        // TODO: Fix this or add it to parsing restraints
        NodeType::OrCmp => {
            condition_expr_code_gen(&children[0], handler, 9);
            handler.curr_scope -= 1;
            handler.scopes.pop();
            condition_expr_code_gen(&children[1], handler, 10);
        }
        NodeType::NeqCmp => {
            condition_expr_code_gen(&children[0], handler, 9);
            condition_expr_code_gen(&children[1], handler, 10);

            handler.push_to_scope(format!("\ncmp x9, x10\nbne .L{}", handler.curr_scope + 1));
            handler.new_scope();
        }
        NodeType::EqCmp => {
            condition_expr_code_gen(&children[0], handler, 9);
            condition_expr_code_gen(&children[1], handler, 10);

            handler.push_to_scope(format!("\ncmp x9, x10\nbeq .L{}", handler.curr_scope + 1));
            handler.new_scope();
        }
        NodeType::Id(id) => {
            let relative_offset = handler.get_id(id).unwrap(); // relative path moves stack down without -

            // This instruction grabs a specific address without modifying the register
            handler.push_to_scope(format!("\nldr x{x}, [x29, #{relative_offset}]",));
            // scopes.push_to_scope(format!("\nadd sp, {}, sp", relative_path).as_str());
        }
        NodeType::NumLiteral(num) => {
            handler.push_to_scope(format!("\nmov x{}, {}", x, num));
        }
        _ => {
            panic!("Expected Condition");
        }
    };
}

fn while_code_gen(node: &TokenNode, handler: &mut Handler) {
    match &node.children {
        Some(children) => {
            handler.push_to_scope(format!("\nb .L{}", handler.curr_scope + 1));
            handler.new_break_anchor();
            handler.new_scope();

            condition_expr_code_gen(&children[0], handler, 19);
            scope_code_gen(&children[1], handler, ScopeType::While);
            handler.new_scope();
        }
        None => {
            panic!("Expected Condition")
        }
    }
}

fn assert_code_gen(node: &TokenNode, handler: &mut Handler) {
    // Exit the program with error
    let children = node.children.as_ref().expect("Assert missing children");
    if children.len() != 1 {
        panic!("Assert incorrect children: {}", children.len());
    }
    condition_expr_code_gen(&children[0], handler, 19);
    handler.push_to_scope("\nmov x0, #1\nmov x8, #93\nsvc #0")
}

fn return_statement_code_gen(node: &TokenNode, handler: &mut Handler) {
    handler.push_to_scope("\n\n; evaluate return statement and place on stack");
    expr_code_gen(
        &node
            .children
            .as_ref()
            .expect("Return statement has no expression")[0],
        handler,
        19,
    );
    handler.push_to_scope("\nldr x9, [28]");
    handler.push_to_scope(
        "\n\n; x15 <- x29\n; x29 <- &old_sfb\nmov x15, x29\nldr x29, [x29]\nstr x9, [x15, #-4]!\nret",
    );
}

fn asm_code_gen(_node: &TokenNode, handler: &mut Handler, str: String) {
    handler.push_to_scope(format!("\n{}", str));
}

fn putchar_code_gen(node: &TokenNode, handler: &mut Handler) {
    let children = node
        .children
        .as_ref()
        .expect("Putchar node has no children");
    expr_code_gen(&children[0], handler, 19);
    // We can assume that the result goes on top of the stack
    handler.push_to_scope("\n\n; putchar\nmov x0, #1 ; stdout\nmov x1, x15 ; put from TOS\nmov x2, #1 ; print 1 char\nmov x16, #4 ; write\nsvc #0x80\n");
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
