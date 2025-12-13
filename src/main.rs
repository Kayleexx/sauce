use sauce::lexer::Lexer;
use sauce::parser::SauceParser;

fn main() {
    let src_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "example.sauce".to_string());

    let src = std::fs::read_to_string(&src_path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", src_path));

    let lexer = Lexer::new(&src);
    let tokens: Vec<_> = lexer.collect::<Result<_, _>>().unwrap_or_else(|e| {
        eprintln!("lex error: {e}");
        std::process::exit(1);
    });

    let parser = SauceParser::new();
    let ast = parser.parse(&tokens).unwrap_or_else(|e| {
        eprintln!("parse error: {e}");
        std::process::exit(1);
    });

    println!("{:#?}", ast);
}
