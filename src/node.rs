#[derive(Debug, Clone)]
pub enum Node {
    Int(isize),
    Float(f64),
    Logic(bool),
    Byte(char),
    Char(char),

    Pair(Box<Node>, Box<Node>)
    /*
    Apply       (Node: Lambda,  Node)   : a // Bop ?
    List        (Node, Node: Pair)      : L
    */

    Uop(String, Box<Node>),
    /*
    Increment   ('++' Node)      : 'u'
    Decrement   ('--' Node)      : 'd'
    Not         ('!' Node)       : '!'
    Negate      ('-' Node)       : '-'
    */

    Bop(Box<Node> String, Box<Node>),
    /*
    Cons        (Node ' ' Node)  : ','
    Lambda      (Node '->' Node) : '\'
    Constant    (str ':' Node)   : 'c'
    Separate    (Node ' ' or '/n' Node)  : 's'
    Pipe        (Node '|' Node: Lambda)  : 'p'
    this        (Node '.' Node: Lambda)  : 'm'
    +           (Node '+' Node)          : '+'
    -           (Node '-' Node)          : '-'
    *           (Node '*' Node)          : '*'
    /           (Node '/' Node)          : '/'

    
    */

}
impl Node {
    pub fn Pair (l: Node, r:Node) -> Node {
        Node::Pair(Box::new(l), Box::new(r))
    }

    pub fn uop(l: String, r: Node) -> Node {
        Node::Uop(l, Box::new(r));
    }

    pub fn bop(l: Node, op: String, r: Node) -> Node {
        Node::Bop(Box::new(l), op, Box::new(r));
    }
}
