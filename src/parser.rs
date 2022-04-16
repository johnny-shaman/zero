use peg;
use crate::node::Node;

peg::parser!( pub grammar zero() for str {
  pub rule parse() -> Vec<Node> = v:sentences()

  rule sentences() -> Vec<Node> = sentence() ** end_of_line()
  rule sentence() -> Node = word() / value() / uop() / bop()

  rule uop() -> Node
    = "!"  _ r:value()   { Node::uop("!", r)  }
    / "-"  _ r:value()   { Node::uop("-", r)  }
    / "++" _ r:value()   { Node::uop("++", r) }
    / "--" _ r:value()   { Node::uop("--", r) }
    / l:proc() _ r:bop() { Node::uop(l, r)    }

  rule bop() -> Node
    = l:value() _ "="  _ r:value() { Node::bop("=", l, r)  }
    / l:value() _ "!"  _ r:value() { Node::bop("!", l, r)  }
    / l:value() _ "<"  _ r:value() { Node::bop("<", l, r)  }
    / l:value() _ ">"  _ r:value() { Node::bop(">", l, r)  }
    / l:value() _ ">=" _ r:value() { Node::bop(">=", l, r) }
    / l:value() _ "<=" _ r:value() { Node::bop("<=", l, r) }
    / l:value() _ "**" _ r:value() { Node::bop("**", l, r) }
    / l:value() _ "*"  _ r:value() { Node::bop("*", l, r)  }
    / l:value() _ "/"  _ r:value() { Node::bop("/", l, r)  }
    / l:value() _ "+"  _ r:value() { Node::bop("+", l, r)  }
    / l:value() _ "-"  _ r:value() { Node::bop("-", l, r)  }
    / l:value() _ "%"  _ r:value() { Node::bop("%", l, r)  }
    / l:value() _ "&"  _ r:value() { Node::bop("&", l, r)  }
    / l:value() _ "|"  _ r:value() { Node::bop("|", l, r)  }
    / l:value() _ "<<" _ r:value() { Node::bop("<<", l, r) }
    / l:value() _ ">>" _ r:value() { Node::bop(">>", l, r) }
    /proc()

  rule proc() -> Node 
    = l:word()" "r:word() {}

  rule value() -> Node
    = "("v:value() _ m:op() _ v:value()")" _ { v }
    / v:int() _   { Node::Int(v) }
    / v:float() _ { Node::Float(v) }
    / v:textQ() _ { Node::TextQ(v) }
    / v:textW() _ { Node::TextW(v) }
    / v:word() _  { Node::Get(v) }

  rule int() -> isize
    = n:$(['-']* ['0'..='9' | ',']+) { n.parse().unwrap() }

  rule float -> f64
    = n:$(['-']* ['0'..='9']* '.' ['0'..='9']+) { n.parse().unwrap() }

  rule textQ() -> String
    = v:$(["'"]+ .*? ["'"]+)
    { String::from(v) }

  rule textW() -> String
    = v:$(['"']+ .*? ['"']+)
    { String::from(v) }

  rule word() -> String // 変数名の定義 --- (*10)
    = v:$(['a'..='z'|'A'..='Z'|'_']+ ['0'..='9']*)
    { String::from(v) }
  
  rule comment() -> None
    = "//" + .*?

  rule end_of_line() = ['\n']+ _ // 文の区切り
  rule lf() = _ ['\n']* _ // 改行
  rule separate() = _ | ['\n']+
  rule _ = [' ' | '\t']* // 空白文字
});
