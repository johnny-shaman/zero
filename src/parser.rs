use peg;
use crate::node::Node;

peg::parser!( pub grammar zero() for str {
  pub rule parse() -> Vec<Node> = v:sentences()

  rule sentences() -> Vec<Node> = sentence() ** end_of_line()
  rule sentence() -> Node = word() / value() / bop() / aop() / mop()

  rule bop() -> Node
    = "!"  _ r:value()   { Node::bop("!", r)  }
    / "++" _ r:value()   { Node::bop("++", r) }
    / "--" _ r:value()   { Node::bop("--", r) }
    / l:proc() _ r:bop() { Node::bop(l, r)    }

  rule aop() -> Node
    = l:value() _ "!"    { Node::bop("!", r)  }
    / l:value() _ "++"   { Node::bop("++", r) }
    / l:value() _ "--"   { Node::bop("--", r) }
    / l:aop() _ r:proc() { Node::bop(l, r)    }

  rule bop() -> Node
    = l:op() _ r:word()
    { Node::Pair(Box::new(l), Box::new(r))}

  rule comp() -> Node
    = l:expr() "=" _ r:comp() { Node::calc('=', l, r) }
    / l:expr() "!" _ r:comp() { Node::calc('!', l, r) }
    / l:expr() ">" _ r:comp() { Node::calc('>', l, r) }
    / l:expr() "<" _ r:comp() { Node::calc('<', l, r) }
    / l:expr() ">=" _ r:comp() { Node::calc('g', l, r) }
    / l:expr() "<=" _ r:comp() { Node::calc('l', l, r) }
    / expr()

  rule expr() -> Node
    = l:term() "+" _ r:calc() { Node::calc('+', l, r) }
    / l:term() "-" _ r:calc() { Node::calc('-', l, r) }
    / term()

  rule term() -> Node
    = l:val() "*" _ r:term() { Node::calc('*', l, r) }
    / l:val() "/" _ r:term() { Node::calc('/', l, r) }
    / l:val() "%" _ r:term() { Node::calc('%', l, r) }
    / val()

  rule value() -> Node
    = "("v:value() _ m:op() _ v:value()")" _ { v }
    / v:int() _   { Node::Int(v) }
    / v:float() _ { Node::Float(v) }
    / v:text() _  { Node::Text(v) }
    / v:word() _  { Node::Get(v) }

  rule int() -> isize
    = n:$(['-']* ['0'..='9' | ',']+) { n.parse().unwrap() }

  rule float -> f64
    = n:$(['-']* ['0'..='9']* '.' ['0'..='9']+) { n.parse().unwrap() }

  rule text() -> String
    = v:$(['"' | "'"]+ .*? ['"' | "'"]+)
    { String::from(v) }

  rule word() -> String // 変数名の定義 --- (*10)
    = v:$(['a'..='z'|'A'..='Z'|'_']+ ['0'..='9']*)
    { String::from(v) }
  

  rule end_of_line() = ['\n']+ _ // 文の区切り
  rule lf() = _ ['\n']* _ // 改行
  rule _ = [' ' | '\t']* // 空白文字
});
