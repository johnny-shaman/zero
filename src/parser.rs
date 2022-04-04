use peg;
use crate::node::Node;

peg::parser!( pub grammar zero() for str {
  pub rule parse() -> Vec<Node> = v:sentences()

  rule sentences() -> Vec<Node> = sentence() ** end_of_line()
  rule sentence() -> Node
  = Pair() {Node::Pair}
  / Op() {Node::Op}
  /  _ { Node::Np }
  
  rule let() -> Node
    = w:word() _ ":" _ v:calc()
    { Node::SetVar(w, Box::new(v))}
  
  rule op() -> Node
    = 

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

  rule val() -> Node
    = "(" _ v:calc() _ ")" _ { v }
    / v:number() _ { Node::Number(v) }
    / v:word() _ { Node::GetVar(v) }

  rule number() -> i64
    = n:$(['0'..='9']+) { n.parse().unwrap() }

  rule word() -> String // 変数名の定義 --- (*10)
    = v:$(['a'..='z'|'A'..='Z'|'_']+ ['0'..='9']*)
    { String::from(v) }
  
  rule text() -> String
    = v:$(['"' | "'"]+ .*? ['"' | "'"]+)
    {}

  rule end_of_line() = [';' | '\n']+ _ // 文の区切り
  rule lf() = _ ['\n']* _ // 改行
  rule _ = [' ' | '\t']* // 空白文字
});
