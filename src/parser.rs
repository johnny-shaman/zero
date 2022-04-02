use peg;
use crate::node::Node;
// トマト言語の文法定義 --- (*1)
peg::parser!( pub grammar tomato() for str {    
    // ルートとなる規則を定義 --- (*2)
    pub rule parse() -> Vec<Node> 
        = v:sentences()
    // プログラムで複文が書けるようにする --- (*3)
    rule sentences() -> Vec<Node>
        = sentence() ** end_of_line()
    // 文を定義 --- (*4)
    rule sentence() -> Node
        = say() / let() / _ { Node::Nop }
    
    // print文の定義 --- (*5)
    rule say() -> Node
        = "say" _ "\"" v:$([^ '"']*) "\""
        { Node::PrintStr(v.to_string()) }
        / "say" _ v:calc()
        { Node::Print(Box::new(v)) }
        
    // 代入文の定義 --- (*8)
    rule let() -> Node
        = w:word() _ ":" _ v:calc() 
        { Node::SetVar(w, Box::new(v))}

    // 計算処理 --- (*9)
    rule calc() -> Node = comp()
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

    rule end_of_line() = [';' | '\n']+ _ // 文の区切り
    rule lf() = _ ['\n']* _ // 改行
    rule _ = [' ' | '\t']* // 空白文字
});

