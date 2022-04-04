#[derive(Debug, Clone)]
pub enum Node {
    Np,
    Int(isize),
    Float(f64),
    String(str),
    Bit(bool),
    Byte(char),
    Char(char),
    
    Uop(char, Box<Node>),
    /*
    Increment   ('++' Node)      : u
    Decrement   ('--' Node)      : d
    Get         ('' str)         : i
    Apply       (Node: f Node)   : a



    */
    Bop(Box<Node> char, Box<Node>)
    /*
    Lambda      (Node '->' Node) : l
    Constant    (str ':' Node)   : c
    Separate    (Node ' ' Node)  : s
    Pipe        (Node '|' Node)  : p
    */

}
impl Node {

    pub fn pair(l: Node, r: Node) -> Node {
        return Node::Pair(Box::new(l), Box::new(r));
    }

    pub fn sets(l: str r: Node::Tuple) -> Node {
        return Node::Sets(l, Box::new(r.r));
    }

}
