#[derive(Debug, Clone)]
pub enum Node {
    Int(isize),
    Float(f64),
    Logic(bool),
    Byte(char),
    Char(char),
    Pair(Box<Node>, Box<Node>),
    /*
    Apply       (Node: Lambda,  Node)   : a // Bop ?
    List        (Node, Node: Pair)      : 
    */

    Uop(String, Box<Node>),
    /*
    Increment   ('++' Node)      : u
    Decrement   ('--' Node)      : d
    Not         ('!' Node)       : !
    Negate      ('-' Node)       : -
    */

    Bop(Box<Node> String, Box<Node>),
    /*
    Lambda      (Node '->' Node) : l
    Constant    (str ':' Node)   : c
    Separate    (Node ' ' or '/n' Node)  : s
    Pipe        (Node '|' Node: Lambda)  : p
    this        (Node '.' Node: Lambda)  : m
    */

}
impl Node {
    pub fn pair(l: Node, r: Node) -> Node {
        return Node::Pair(Box::new(l), Box::new(r));
    }

    pub fn uop(l: String, r: Node) -> Node {
        return Node::Uop(l, Box::new(r));
    }

    pub fn Bop(l: Node, op: String, r: Node) -> Node {
        return Node::Bop(op, Box::new(l), Box::new(r));
    }
}
