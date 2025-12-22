use clap::{CommandFactory, Parser};
use sauce::lexer::Lexer;
use sauce::parser::SauceParser;
use sauce::typechecker::checker::typecheck_program;
use sauce::interpreter::eval::eval_program;

#[derive(Parser)]
#[command(name = "sauce", about = "Sauce language compiler")]
struct Args {
    /// Input source file
    filename: Option<String>,

    /// Output prefix for files
    #[arg(short, long)]
    output: Option<String>,

    /// Output lexed tokens
    #[arg(long, visible_alias = "lex")]
    tokens: bool,

    /// Output parsed AST
    #[arg(long, visible_alias = "parse")]
    ast: bool,

    /// Typecheck the program
    #[arg(long)]
    check: bool,

    /// Run the program
    #[arg(long)]
    run: bool,
}

fn main() {
    let args = Args::parse();
    let filename = args.filename.as_ref().unwrap_or_else(|| {
        let _ = Args::command().print_help();
        std::process::exit(0);
    });

    let src = std::fs::read_to_string(&filename).unwrap_or_else(|e| {
        eprintln!("Error reading {}: {}", filename, e);
        std::process::exit(1);
    });

    let lexer = Lexer::new(&src);
    let tokens: Vec<_> = lexer.collect::<Result<_, _>>().unwrap_or_else(|e| {
        eprintln!("lex error: {e}");
        std::process::exit(1);
    });

    if args.tokens {
        output("Tokens", &format!("{:#?}", &tokens), &args.output, "tokens");

        if !args.ast && !args.check && !args.run {
            return;
        }
    }

    let parser = SauceParser::new();
    let ast = parser.parse(&tokens).unwrap_or_else(|e| {
        eprintln!("parse error: {e}");
        std::process::exit(1);
    });

    if args.ast {
        output("AST", &format!("{:#?}", &ast), &args.output, "ast");

        if !args.check && !args.run {
            return;
        }
    }

    typecheck_program(&ast).unwrap_or_else(|e| {
        eprintln!("typecheck error: {e}");
        std::process::exit(1);
    });

    if args.check {
        return;
    }

    if args.run {
        eval_program(&ast).unwrap_or_else(|e| {
            eprintln!("runtime error: {e}");
            std::process::exit(1);
        });
        return;
    }

    // fallback: codegen (future)
    todo!("codegen");
}

fn output(label: &str, content: &str, prefix: &Option<String>, suffix: &str) {
    if let Some(p) = prefix {
        let filename = format!("{}.{}", p, suffix);
        if let Err(e) = std::fs::write(&filename, content) {
            eprintln!("Error writing to {}: {}", filename, e);
            std::process::exit(1);
        }
    } else {
        println!("{}:\n{}", label, content);
    }
}
