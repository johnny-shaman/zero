// 文法要素をNode型として定義 --- (*1)
#[derive(Debug, Clone)]
pub enum Node {
    Np, // 何もしない
    Int(isize), // 数値
    Float(f64), // 数値
    String(str), // String
    Bit(bool), // Boolean
    Byte(char),
    Char(char),
    Pair(Box<Node>, Box<Node>),
    Op(char),
    Sets(str, Box<Node>), // set
    Gets(str), // get
    Lambda(str, Box<Node>) //^xy.xy,
}
impl Node {

    pub fn pair(l: Node, r: Node) -> Node {
        return Node::Pair(Box::new(l), Box::new(r));
    }

    pub fn lambda(l: str, r: Node) -> Node {
        return Node::Lambda(l, Box::new(r));
    }

    pub fn sets(l: str r: Node::Tuple) -> Node {
        Node::Sets(l, Box::new(r.r));
    }

}

