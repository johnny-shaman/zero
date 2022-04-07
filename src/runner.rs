use std::collections::HashMap;
use crate::parser::zero;
use crate::node::Node;

struct Context {
    vars: HashMap<str, T>,
}

// 構文木を一つ実行する --- (*2)
fn run_node(ctx: &mut Context, node: Node) -> Node {
    // どのタイプのノードかを判定
    match node {
        Node::Int(v: isize) => v,
        Node::Float(v: u64) => v,
        Node::
        Node::Pair(l, r) => (l, r), // Tuple
        Node::Bop(op, ) => {
            
        },

        Node::GetVar(name) => { // 変数の値を得る --- (*4)
            match ctx.vars.get(&name) {
                Some(v) => *v,
                None => 0,
            }
        },

        Node::SetVar(name, node) => { // 変数の代入
            let val = run_node(ctx, *node);
            ctx.vars.insert(name, val);
            val
        },
        Node::PrintStr(v) => { println!("{}", v); 0}, // --- (*7)
        Node::Print(node) => { // print文 --- (*8)
            let v = run_node(ctx, *node);
            println!("{}", v);
            v
        },
        _ => 0,
    }
}


// 演算子に基づいて計算を行う --- (*9)
fn calc_op(val_l:i64, val_r:i64) -> i64 {
    match op {
        '+' => val_l + val_r,
        '-' => val_l - val_r,
        '*' => val_l * val_r,
        '/' => val_l / val_r,
        '%' => val_l % val_r,
        '=' => if val_l == val_r {1} else {0},
        '!' => if val_l != val_r {1} else {0},
        '>' => if val_l > val_r {1} else {0},
        'g' => if val_l >= val_r {1} else {0},
        '<' => if val_l < val_r {1} else {0},
        'l' => if val_l <= val_r {1} else {0},
        _ => 0,
    }
}
// 繰り返しNodeを実行 --- (*10)
fn run_nodes(ctx: &mut Context, nodes: &Vec<Node>) -> i64 {
    let mut result = 0;
    nodes.iter().for_each(|node| {
        result = run_node(ctx, node.clone())});
    result
}
// 手軽にプログラムを実行する関数 --- (*11)
pub fn run(src: &str) -> i64 {
    let nodes = zero::parse(src).unwrap();
    let mut ctx = Context{vars:HashMap::new()};
    run_nodes(&mut ctx, &nodes)
}

