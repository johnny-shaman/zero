#[derive(Debug, Clone)]
pub enum Node {
    Np,
    Int(isize),
    Float(f64),
    String(str),
    Bit(bool),
    Byte(char),
    Char(char),
    Op(str),
    Pair(Box<Node>, Box<Node>),
    Sets(str, Box<Node>),
    Gets(str),
}
impl Node {

    pub fn pair(l: Node, r: Node) -> Node {
        return Node::Pair(Box::new(l), Box::new(r));
    }

    pub fn sets(l: str r: Node::Tuple) -> Node {
        return Node::Sets(l, Box::new(r.r));
    }

}
