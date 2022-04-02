mod parser;
mod runner;
mod node;
use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("[USAGE] zero file.zero");
        return;
    }
    // ファイルを開く
    let filename = &args[1];
    let src = fs::read_to_string(filename).unwrap();
    runner::run(&src);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_run() {
        assert_eq!(runner::run("say 32"), 32);
        assert_eq!(runner::run("say 1+2*3"), 7);
        assert_eq!(runner::run("say \"abc\""), 0);
    }
}
