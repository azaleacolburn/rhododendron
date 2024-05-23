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

const _MOV_EXPR_1: &str = "\nldr x9 [sp], #8";
const _MOV_EXPR_2: &str = "\nldr x10 [sp], #8";
const _ADD_EXPR: &str = "\nadd x9, x9, x10";

// const GET_ADR: &str = "\nldr {} [sp], #16"; // assuming it was 32 bit address formatting needed
// const STR_ADR: &str = "\nstr {}, [sp, #-16]!";

const _CMP_EXPR: &str = "\ncmp x9, x0"; // comparison flag set in the processor status register
                                        // const SYS_CALL_NUM: &str = "\nmov x0, {}"; // syscall_number
                                        // const SYS_CALL_ARG1: &str = "\nmov x0, {}"; // etc
const _SYS_CALL: &str = "\nsvc #0x80";
#[derive(Debug, Clone)]
pub struct FunctionSig {
    args: Vec<(String, i32)>, // ids in order
    label: String,
}

#[derive(Debug, Clone)]
pub struct SymbolTable {
    table: HashMap<String, i32>, // id, offsert from current stack frame base
    function_table: HashMap<String, FunctionSig>, // function name, label
    parent: Option<usize>,       // the index in the
    furthest_offset: i32,
}

impl SymbolTable {
    fn new(parent: Option<usize>) -> SymbolTable {
        SymbolTable {
            table: HashMap::new(),
            function_table: HashMap::new(),
            parent,
            furthest_offset: 0,
        }
    }

    fn get_function(&self, id: String) -> Option<&FunctionSig> {
        self.function_table.get(&id).to_owned()
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
    curr_frame: usize,
    sym_arena: Vec<SymbolTable>,
    break_anchors: Vec<usize>, // each number represents the insertuction pointer that can be broken to from the current scope
}

impl Handler {
    fn new() -> Self {
        let mut handler = Handler {
            scopes: vec![String::from("\n.global .main\n.align 4\n")],
            curr_scope: 0,
            break_anchors: vec![],
            sym_arena: vec![SymbolTable::new(None)],
            curr_frame: 0,
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
    fn new_stack_frame(&mut self) {
        self.sym_arena.push(SymbolTable::new(Some(self.curr_frame)));
        self.curr_frame = self.sym_arena.len() - 1;
    }

    // Panics if id doesn't exist
    fn get_id(&self, id: impl ToString, c: &mut i32) -> Option<&i32> {
        let sym_res = self.sym_arena[self.curr_frame].get_id(id.to_string());

        if sym_res.is_none() {
            *c += 1;
            return match &self.sym_arena[self.curr_frame].parent {
                Some(parent) => Some(
                    self.sym_arena[*parent]
                        .get_id(id.to_string())
                        .expect("Symbol not found"),
                ),
                None => return None,
            };
        }

        sym_res
    }

    fn get_function(&mut self, id: impl ToString) -> Option<&FunctionSig> {
        println!("Function sym table:\n{:?}", self.sym_arena);

        let mut i = self.curr_frame;
        let mut sym_ret = self.sym_arena[i].get_function(id.to_string());

        while sym_ret.is_none() {
            i = self.sym_arena[i].parent.expect("Symbol not found");
            sym_ret = self.sym_arena[i].get_function(id.to_string());
        }

        sym_ret
    }

    fn new_id(&mut self, id: impl ToString, size: i32) {
        self.sym_arena[self.curr_frame].new_id(id.to_string(), size);
    }

    fn new_8_byte(&mut self, id: impl ToString) {
        self.sym_arena[self.curr_frame].new_id(id.to_string(), 8);
    }

    fn new_expr_lit(&mut self) {
        self.sym_arena[self.curr_frame].furthest_offset += 8;
    }

    fn unload_expr_lit(&mut self) {
        self.sym_arena[self.curr_frame].furthest_offset -= 8;
    }

    fn new_function(&mut self, name: impl ToString, scope: i32, args: Vec<(String, i32)>) {
        let new_scope_label = format!(".L{}", scope);
        let function_sig = FunctionSig {
            args: args.clone(),
            label: new_scope_label, // figure out how to determine label (probably based on a property in handler)
        };
        self.sym_arena[self.curr_frame]
            .function_table
            .insert(name.to_string(), function_sig);

        self.new_stack_frame();

        //self.new_expr_lit(); // sfb

        for arg in args.into_iter() {
            self.new_expr_lit();
            self.new_id(arg.0, 0);
        }
    }
}

pub fn main(node: &TokenNode) -> String {
    let mut handler = Handler::new();
    if node.token == NodeType::Program {
        scope_code_gen(
            &node.children.as_ref().unwrap()[0],
            &mut handler,
            ScopeType::Program,
        );
    }
    handler.push_to_scope("\n\n; exit program gracefully\nmov x0, #0\nmov x16, #1\nsvc #0x80");

    handler.format_scopes()
}

pub fn scope_code_gen(node: &TokenNode, handler: &mut Handler, scope_type: ScopeType) {
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
            NodeType::_Loop => {}
            NodeType::FunctionCall(id) => {
                function_call_code_gen(&child_node, handler, id.to_string())
            }
            NodeType::FunctionDecaration((id, t)) => {
                function_declare_code_gen(&child_node, handler, id.to_string(), t.clone())
            }
            NodeType::Break => handler.insert_break(),
            NodeType::Return => {
                // return type shouldn't matter for now
                return return_statement_code_gen(&child_node, handler);
            }
            NodeType::Asm(str) => asm_code_gen(&child_node, handler, str.to_string()),
            NodeType::PutChar => putchar_code_gen(&child_node, handler),
            NodeType::Assert => assert_code_gen(&child_node, handler),
            _ => {}
        };
    }
    if scope_type == ScopeType::While {
        //handler.push_to_scope(format!(
        //    "\n; break statement\nb .L{}",
        //    handler.curr_scope + 1
        //));
    }
}

/// Returns the generated code
/// Modifies the sp
pub fn declare_code_gen(node: &TokenNode, handler: &mut Handler, name: String, _t: RhTypes) {
    println!("Sym Arena:\n{:?}", handler.sym_arena[handler.curr_frame]);
    handler.push_to_scope(format!(
        "\n\n; var dec: {}, offset: {} (wrong for arrays)",
        name,
        handler.sym_arena[handler.curr_frame].furthest_offset + 8
    ));
    expr_code_gen(
        &node.children.as_ref().expect("Node to have children")[0],
        handler,
    );

    handler.new_id(&name, 0);
    handler.push_to_scope("\n");
}

pub fn assignment_code_gen(node: &TokenNode, handler: &mut Handler) {
    handler.push_to_scope("\n; variable assignment");
    println!("Assignment Node: {:?}", node.token);
    let children = &node
        .children
        .as_ref()
        .expect("Assignment token need children");
    assert_eq!(children.len(), 2);

    let op = match &node.token {
        NodeType::Assignment(op) => op,
        _ => panic!("Given given invalid assignment node"),
    };
    let mut c = 0;

    let relative_stack_position: i32 = match &children[0].token {
        NodeType::DeRef => return deref_assignment_code_gen(node, handler, op.clone()),
        NodeType::Id(name) => handler
            .get_id(name, &mut c)
            .expect("Undefined Identifier")
            .clone(),
        _ => panic!("Can only assign to deref or id"),
    };

    // TODO: Figure out how to handle expressions

    expr_code_gen(&node.children.as_ref().unwrap()[1], handler);
    handler.push_to_scope("\nldr x10, [x15], #8");
    handler.unload_expr_lit();

    // this load the old value in case we need it
    if *op != AssignmentOpType::Eq {
        for i in 0..c {
            handler.push_to_scope(format!(
                "\n\n; getting var from prev scope: {}\nstr x29, [x15, #-8]!\nldr x29, [x29]",
                i + 1
            ));
        }
        handler.push_to_scope(format!("\nldr x9, [x29, #{relative_stack_position}]"));
        for _ in 0..c {
            handler.push_to_scope("\nldr x29, [x15], #8");
        }
        match op {
            AssignmentOpType::AddEq => handler.push_to_scope("\nadd x9, x9, x10"),
            AssignmentOpType::SubEq => handler.push_to_scope("\nsub x9, x9, x10"),
            AssignmentOpType::MulEq => handler.push_to_scope("\nmul x9, x9, x10"),
            AssignmentOpType::DivEq => handler.push_to_scope("\nudiv x9, x9, x10"),
            AssignmentOpType::BOrEq => handler.push_to_scope("\nbor x9, x9, x10"),
            AssignmentOpType::BAndEq => handler.push_to_scope("\nand x9, x9, x10"),
            AssignmentOpType::BXorEq => handler.push_to_scope("\nxor x9, x9, x10"),
            _ => panic!("Unexpected Operator"),
        };
    }
    for i in 0..c {
        handler.push_to_scope(format!(
            "\n\n; getting var from prev scope: {}\nstr x29, [x15, #-8]!\nldr x29, [x29]",
            i + 1
        ));
    }
    handler.push_to_scope(format!("\nstr x9, [x29, #{relative_stack_position}]"));
    for _ in 0..c {
        handler.push_to_scope("\nldr x29, [x15], #8");
    }

    //handler.push_to_scope(format!("\nstr x9, [x29], #{relative_stack_position}"));
}

fn deref_assignment_code_gen(node: &TokenNode, handler: &mut Handler, op: AssignmentOpType) {
    let children = node
        .children
        .as_ref()
        .expect("deref assignment node must have children");
    println!("node:\n{:?}", children[0]);
    handler.push_to_scope("\n\n; deref assignment");
    expr_code_gen(
        &children[0]
            .children
            .as_ref()
            .expect("deref must have children in assignment")[0],
        handler,
    );
    expr_code_gen(&node.children.as_ref().unwrap()[1], handler);
    handler.push_to_scope("\nldr x10, [x15], #8 ; pop res\nldr x11, [x15], #8 ; pop adr");
    handler.unload_expr_lit();
    handler.unload_expr_lit();

    // this load the old value in case we need it
    if op != AssignmentOpType::Eq {
        handler.push_to_scope("\nldr x9, [x11]");
        match op {
            AssignmentOpType::AddEq => handler.push_to_scope("\nadd x9, x9, x10"),
            AssignmentOpType::SubEq => handler.push_to_scope("\nsub x9, x9, x10"),
            AssignmentOpType::MulEq => handler.push_to_scope("\nmul x9, x9, x10"),
            AssignmentOpType::DivEq => handler.push_to_scope("\nudiv x9, x9, x10"),
            AssignmentOpType::BOrEq => handler.push_to_scope("\nbor x9, x9, x10"),
            AssignmentOpType::BAndEq => handler.push_to_scope("\nand x9, x9, x10"),
            AssignmentOpType::BXorEq => handler.push_to_scope("\nxor x9, x9, x10"),
            _ => panic!("Unexpected Operator"),
        };
    } else {
        handler.push_to_scope("\nmov x9, x10");
    }

    handler.push_to_scope("\nstr x9, [x11]");
}

// Leaves the result on TOS
fn expr_code_gen(node: &TokenNode, handler: &mut Handler) {
    match &node.token {
        NodeType::NumLiteral(val) => {
            handler.push_to_scope(format!("\nmov x9, #{val}"));
            handler.push_to_scope("\nstr x9, [x15, #-8]!");
            handler.new_expr_lit();
        }
        NodeType::Id(name) => {
            let mut c = 0;
            let offset = handler
                .get_id(name, &mut c)
                .expect("Undefined identifier")
                .clone();
            for i in 0..c {
                handler.push_to_scope(format!(
                    "\n; getting var from prev scope: {}\nstr x29, [x15, #-8]!\nldr x29, [x29]",
                    i + 1
                ));
            }
            handler.push_to_scope(format!("\nldr x9, [x29, #{offset}]"));
            for _ in 0..c {
                handler.push_to_scope("\nldr x29, [x15], #8");
            }
            handler.push_to_scope("\nstr x9, [x15, #-8]!");
            handler.new_expr_lit();
        }
        NodeType::FunctionCall(name) => {
            function_call_code_gen(&node, handler, name.to_string());
            handler.push_to_scope("\n; assume ret is TOS");
            handler.new_expr_lit();
        }
        NodeType::Adr(id) => {
            let mut c = 0;
            let relative_offset = handler.get_id(id, &mut c).unwrap().abs();
            handler.push_to_scope(format!("\n; getting the adr of: {id}"));

            for i in 0..c {
                handler.push_to_scope(format!(
                    "\n; getting var from prev scopeL {}\nstr x29, [x15, #-8]!\nldr x29, [x29]",
                    i + 1
                ));
            }
            handler.push_to_scope(format!("\nsub x9, x29, #{relative_offset}"));
            for _ in 0..c {
                handler.push_to_scope("\nldr x29, [x15], #8");
            }

            handler.push_to_scope(format!("\nstr x9, [x15, #-8]!"));
            handler.new_expr_lit();
        }
        NodeType::DeRef => {
            deref_code_gen(&node, handler);
        }
        NodeType::Array(n) => {
            let children = node
                .children
                .as_ref()
                .expect("Array node should have children");
            let zeros: i32 = *n - children.len() as i32;
            handler.push_to_scope("\n; new array\nsub x11, x15, #8; anchor ptr\n");
            for node in children.iter() {
                expr_code_gen(&node, handler);
                handler.push_to_scope("\n")
            }
            for _ in 0..zeros {
                handler.push_to_scope("\n; empty array section\nmov x9, #0\nstr x9, [x15, #-8]!");
                handler.new_expr_lit();
            }
            handler.push_to_scope("\nstr x11, [x15, #-8]! ; str array anchor TOS");
            handler.new_expr_lit();
        }
        _ => {
            expr_code_gen(&node.children.as_ref().unwrap()[0], handler);
            expr_code_gen(&node.children.as_ref().unwrap()[1], handler);
            handler.push_to_scope("\n\n; load from stack\nldr x10, [x15], #8\nldr x9, [x15], #8");
            handler.unload_expr_lit();
            handler.unload_expr_lit();
            match &node.token {
                NodeType::Add => handler.push_to_scope("\nadd x9, x9, x10"),
                NodeType::Sub => handler.push_to_scope("\nsub x9, x9, x10"),
                NodeType::Div => handler.push_to_scope("\nudiv x9, x9, x10"),
                NodeType::Mul => handler.push_to_scope("\nmul x9, x9, x10"),
                NodeType::MNeg => handler.push_to_scope("\nmneg x9, x9, x10"),
                NodeType::BAnd => handler.push_to_scope("\nand x9, x9, x10"),
                NodeType::BOr => handler.push_to_scope("\nor x9, x9, x10"),
                NodeType::BXor => handler.push_to_scope("\nxor x9, x9, x10"),
                _ => panic!("Expected Expression"),
            };
            handler.push_to_scope("\nstr x9, [x15, #-8]!");
            handler.new_expr_lit();
        }
    }
}

pub fn function_declare_code_gen(
    node: &TokenNode,
    handler: &mut Handler,
    name: String,
    t: RhTypes,
) {
    let children = node
        .children
        .as_ref()
        .expect("Function node must have children");
    let orig_scope = handler.curr_scope;
    let orig_frame = handler.curr_frame;
    println!("orig_frame: {}", handler.curr_frame);
    handler.new_scope();
    handler.push_to_scope(format!("\n; function declaration: {name}\n"));

    let function_scope = handler.curr_scope;
    let mut args: Vec<(String, i32)> = vec![];

    for child in 0..children.len() - 1 {
        if let NodeType::Declaration((id, t)) = &children[child].token {
            let size = match t {
                RhTypes::Char => 8,
                RhTypes::Int => 8,
                RhTypes::Void => 8,
            };
            // TODO: figure out what other code needs to go here (id any)
            args.push((id.clone(), size));
        }
    }
    handler.new_function(name.clone(), function_scope as i32, args.clone());
    handler.push_to_scope("\n; save link reg\nstr lr, [x15, #-8]!");
    handler.new_expr_lit();
    handler.new_id("lr", 0); // link register

    let scope_child = &children[children.len() - 1];
    if let NodeType::Scope(_) = scope_child.token {
        scope_code_gen(&scope_child, handler, ScopeType::Function(t.clone()));
    }

    if t == RhTypes::Void {
        handler.push_to_scope(format!(
            "
; void function return\nldr lr, [x29, #{}]
add x15, x29, #8\nldr x29, [x29]\nret
            ",
            handler
                .get_id("lr", &mut 0)
                .expect("LR not placed in table")
        ));
    }

    handler.curr_scope = orig_scope;
    handler.curr_frame = orig_frame;
}

pub fn function_call_code_gen(node: &TokenNode, handler: &mut Handler, name: String) {
    handler.push_to_scope("\n\n; place old sfb\nstr x29, [x15, #-8]!\nmov x10, x15");

    let children = node.children.as_ref().expect("Function has no children");

    let function_sig = handler
        .get_function(&name)
        .expect("Function id not found")
        .clone();

    for i in 0..function_sig.args.len() {
        expr_code_gen(&children[i], handler);
        // unloading is fine, since they get 'loaded' in the declare code
        handler.unload_expr_lit();
    }

    handler.push_to_scope("\nmov x29, x10");
    handler.push_to_scope(format!("\nbl {}", function_sig.label));
}

pub fn if_code_gen(node: &TokenNode, handler: &mut Handler) {
    let orig_frame = handler.curr_frame;
    let children = &node.children.as_ref().expect("Expected Condition");

    handler.push_to_scope(format!(
        "\n\n; if statement\nb .L{}",
        handler.scopes.len() + 1
    ));

    // Here we essentially reserve a new scope for after the if statement
    let after_if_scope = handler.scopes.len();
    handler.new_scope();
    handler.curr_scope -= 1;

    condition_expr_code_gen(&children[0], handler, after_if_scope);
    handler.curr_scope -= 1; // I think this is fine
    handler.push_to_scope(format!("\nb .L{}", after_if_scope));
    handler.curr_scope += 1; // change to anchor otherwise??? (won't work)

    //handler.new_stack_frame();
    handler.push_to_scope(
        "\n; scope of if statement\n\n; place old sfb\nstr x29, [x15, #-8]!\nmov x29, x15",
    );

    scope_code_gen(&children[1], handler, ScopeType::If);
    handler.push_to_scope(format!(
        "\n\n; if return\nadd x15, x29, #8\nldr x29, [x29]\nb .L{}",
        after_if_scope
    ));
    handler.curr_scope = after_if_scope;
    handler.push_to_scope("\n; after if statement scope");
    //handler.curr_frame = orig_frame;
}

pub fn deref_code_gen(node: &TokenNode, handler: &mut Handler) {
    let children = node
        .children
        .as_ref()
        .expect("DeRef node should have children");
    assert_eq!(children.len(), 1);
    expr_code_gen(&children[0], handler);
    handler
        .push_to_scope("\n\n; deref expr\nldr x9, [x15], #8\nldr x10, [x9]\nstr x10, [x15, #-8]!");
    // stack cancels out
}

pub fn condition_expr_code_gen(node: &TokenNode, handler: &mut Handler, else_scope: usize) {
    let children: Vec<TokenNode> = node.children.as_ref().unwrap_or(&Vec::new()).to_vec();
    println!("condition token: {:?}", node.token);
    match &node.token {
        NodeType::AndCmp => {
            let mut anchor = handler.curr_scope;
            condition_expr_code_gen(&children[0], handler, else_scope);
            handler.curr_scope = anchor;
            handler.scopes.pop();
            handler.push_to_scope(format!("\nb .L{}", else_scope)); // FIXME: GOING TO CONDITIONAL AFTER SCOPE
            handler.new_scope();
            anchor = handler.curr_scope;
            condition_expr_code_gen(&children[1], handler, else_scope);
            handler.curr_scope = anchor;
            handler.scopes.pop();
            handler.push_to_scope(format!("\nb .L{}", else_scope));
            handler.new_scope();
        }
        // This makes putting booleans into conditions illegal
        // TODO: Fix this or add it to parsing restraints
        NodeType::OrCmp => {
            let anchor = handler.curr_scope;
            condition_expr_code_gen(&children[0], handler, else_scope);
            handler.curr_scope = anchor;
            handler.scopes.pop();
            condition_expr_code_gen(&children[1], handler, else_scope);
            handler.curr_scope = anchor;
            handler.scopes.pop();
            handler.push_to_scope(format!("\nb .L{}", else_scope));
            handler.new_scope();
        }
        NodeType::NeqCmp => {
            condition_expr_code_gen(&children[0], handler, else_scope);
            condition_expr_code_gen(&children[1], handler, else_scope);

            handler.push_to_scope(format!(
                "\nldr x9, [x15], #8\nldr x10, [x15], #8\ncmp x9, x10\nbne .L{}",
                handler.scopes.len(),
            ));
            handler.new_scope();
        }
        NodeType::EqCmp => {
            condition_expr_code_gen(&children[0], handler, else_scope);
            condition_expr_code_gen(&children[1], handler, else_scope);

            handler.push_to_scope(format!(
                "\nldr x9, [x15], #8\nldr x10, [x15], #8\ncmp x9, x10\nbeq .L{}",
                handler.scopes.len(),
            ));
            handler.new_scope();
        }
        _ => {
            expr_code_gen(&node, handler);
        }
    };
}

fn while_code_gen(node: &TokenNode, handler: &mut Handler) {
    let orig_frame = handler.curr_frame;
    let children = &node.children.as_ref().expect("Expected Condition");

    handler.push_to_scope(format!(
        "\n\n; while statement\nb .L{}",
        handler.scopes.len() + 1
    ));
    let anchor = handler.curr_scope;

    // Reserve after while loop scope
    handler.new_scope();
    let after_while_scope = handler.curr_scope;
    handler.curr_scope = anchor;

    handler.new_scope();
    let condition_scope = handler.curr_scope;
    condition_expr_code_gen(&children[0], handler, after_while_scope);
    handler.curr_scope -= 1; // I think this is fine
    handler.push_to_scope(format!("\nb .L{}", after_while_scope));
    handler.curr_scope += 1; // change to anchor otherwise??? (won't work)
    handler.new_stack_frame();
    handler.push_to_scope(
        "\n; scope of while statement\n\n; place old sfb\nstr x29, [x15, #-8]!\nmov x29, x15\nstr lr, [x15, #-8]!\n\n",
    );
    handler.new_expr_lit();
    handler.new_id("lr", 0);

    let mut c = 0;

    scope_code_gen(&children[1], handler, ScopeType::If);
    handler.push_to_scope(format!(
        "\n\n; while return\nldr lr, [x29, #{}]\nadd x15, x29, #8\nldr x29, [x29]\nb .L{}",
        handler.get_id("lr", &mut c).unwrap(),
        condition_scope
    ));

    assert_eq!(c, 0);

    handler.curr_scope = after_while_scope;
    handler.push_to_scope("\n; after while statement scope");
    handler.curr_frame = orig_frame;
}

fn assert_code_gen(node: &TokenNode, handler: &mut Handler) {
    // Exit the program with error
    let children = node.children.as_ref().expect("Assert missing children");
    if children.len() != 1 {
        panic!("Assert incorrect children: {}", children.len());
    }
    // Split condition into the scope-hoping and the comparisons for convenience
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
    );
    handler.push_to_scope("\n; function return\nldr x9, [x15], #8");
    let mut c = 0;
    let offset = handler
        .get_id("lr", &mut c)
        .expect("Lr not placed in frame")
        .clone();
    assert_eq!(c, 0);
    handler.push_to_scope(format!("\nldr lr, [x29, #{}]", offset));

    handler.push_to_scope("\nadd x15, x29, #8\nldr x29, [x29]\nstr x9, [x15, #-8]!\nret");
    handler.unload_expr_lit();
    //handler.curr_frame = handler.sym_arena[handler.curr_frame]
    //    .parent
    //    .expect("Functions must have parent scopes");
}

fn asm_code_gen(_node: &TokenNode, handler: &mut Handler, str: String) {
    handler.push_to_scope(format!("\n{}", str));
}

fn putchar_code_gen(node: &TokenNode, handler: &mut Handler) {
    let children = node
        .children
        .as_ref()
        .expect("Putchar node has no children");

    expr_code_gen(&children[0], handler);
    // We can assume that the result goes on top of the stack
    handler.push_to_scope("\n\n; putchar\nmov x0, #1 ; stdout\nmov x1, x15 ; put from TOS\nmov x2, #1 ; print 1 char\nmov x16, #4 ; write\nsvc #0x80\n; unload the TOS\nadd x15, x15, #8\n");
    handler.unload_expr_lit();
}
