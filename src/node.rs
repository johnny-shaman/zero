// 文法要素をNode型として定義 --- (*1)
#[derive(Debug, Clone)]
pub enum Node {
    Nop, // 何もしない
    Number(i64), // 数値を表す
    Calc(char, Box<Node>, Box<Node>), // 計算式
    Print(Box<Node>), // print文(計算出力)
    PrintStr(String), // print文(定数出力)
    SetVar(String, Box<Node>), // 変数代入
    GetVar(String), // 変数参照
}
impl Node {
    // 手軽にNode::Calc型を返す関数 --- (*2)
    pub fn calc(op: char, l: Node, r: Node) -> Node {
        Node::Calc(op, Box::new(l), Box::new(r))
    }
}
