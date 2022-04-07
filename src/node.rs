#[derive(Debug, Clone)]
pub enum Node {
    Int(isize),
    Float(f64),
    Bit(bool),
    Byte(char),
    Char(char),
    
    Pair(Box<Node>, Box<Node>),
    /*
    Apply       (Node: Lambda,  Node)   : a // Bop ?
    List        (Node, Node: Pair)      : 
    */

    Bop(String, Box<Node>),
    /*
    Increment   ('++' Node)      : u
    Decrement   ('--' Node)      : d
    Not         ('!' Node)       : !
    Negate      ('-' Node)       : -
    */

    Aop(Box<Node>, String),
    /*
    Increment   (Node '++')      : u
    Decrement   (Node '--')      : d
    Factrial    (Node '!')       : !
    */

    Mop(Box<Node> String, Box<Node>),
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

    pub fn bop(l: String, r: Node) -> Node {
        return Node::Bop(l, Box::new(r));
    }

    pub fn aop(l: Node, r: String) -> Node {
        return Node::Aop(Box::new(l), r);
    }

    pub fn mop(l: Node, op: String, r: Node) -> Node {
        return Node::Mop(op, Box::new(l), Box::new(r));
    }
}
