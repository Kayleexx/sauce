use clap::{CommandFactory, Parser, Subcommand};

use sauce::codegen::codegen;
use sauce::interpreter::eval::eval_program;
use sauce::lexer::Lexer;
use sauce::parser::SauceParser;
use sauce::typechecker::checker::typecheck_program;

#[derive(Parser)]
#[command(
    name = "sauce",
    about = "The Sauce programming language",
    version,
    long_about = r#"
███████╗ █████╗ ██╗   ██╗ ██████╗ ███████╗
██╔════╝██╔══██╗██║   ██║██╔════╝ ██╔════╝
███████╗███████║██║   ██║██║  ███╗█████╗  
╚════██║██╔══██║██║   ██║██║   ██║██╔══╝  
███████║██║  ██║╚██████╔╝╚██████╔╝███████╗
╚══════╝╚═╝  ╚═╝ ╚═════╝  ╚═════╝ ╚══════╝

Sauce v0.1.0 — pipeline-first, effect-aware
"#
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    #[command(flatten)]
    args: Args,
}

#[derive(Subcommand)]
enum Command {
    Run { filename: String },

    Check { filename: String },

    Build { filename: String },
}

#[derive(Parser)]
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

    /// Run the program using the interpreter
    #[arg(long)]
    run: bool,
}

fn main() {
    let cli = Cli::parse();

    if let Some(cmd) = &cli.command {
        match cmd {
            Command::Run { filename } => {
                run_pipeline(filename, Mode::Run);
            }
            Command::Check { filename } => {
                run_pipeline(filename, Mode::Check);
            }
            Command::Build { filename } => {
                run_pipeline(filename, Mode::Build);
            }
        }
        return;
    }

    let args = &cli.args;

    let filename = args.filename.as_ref().unwrap_or_else(|| {
        let _ = Cli::command().print_help();
        std::process::exit(0);
    });

    let src = read_file(filename);
    let tokens = lex(&src);

    if args.tokens {
        output("Tokens", &format!("{:#?}", &tokens), &args.output, "tokens");
        if !args.ast && !args.check && !args.run {
            return;
        }
    }

    let ast = parse(&tokens);

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

    codegen(&ast);
}

enum Mode {
    Run,
    Check,
    Build,
}

fn run_pipeline(filename: &str, mode: Mode) {
    let src = read_file(filename);
    let tokens = lex(&src);
    let ast = parse(&tokens);

    typecheck_program(&ast).unwrap_or_else(|e| {
        eprintln!("typecheck error: {e}");
        std::process::exit(1);
    });

    match mode {
        Mode::Check => {}
        Mode::Run => {
            eval_program(&ast).unwrap_or_else(|e| {
                eprintln!("runtime error: {e}");
                std::process::exit(1);
            });
        }
        Mode::Build => {
            codegen(&ast);
        }
    }
}

fn read_file(filename: &str) -> String {
    std::fs::read_to_string(filename).unwrap_or_else(|e| {
        eprintln!("Error reading {}: {}", filename, e);
        std::process::exit(1);
    })
}

fn lex(src: &str) -> Vec<sauce::lexer::SpannedToken> {
    let lexer = Lexer::new(src);
    lexer.collect::<Result<_, _>>().unwrap_or_else(|e| {
        eprintln!("lex error: {e}");
        std::process::exit(1);
    })
}

fn parse(tokens: &[sauce::lexer::SpannedToken]) -> sauce::ast::ast::Ast {
    let parser = SauceParser::new();
    parser.parse(tokens).unwrap_or_else(|e| {
        eprintln!("parse error: {e}");
        std::process::exit(1);
    })
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
