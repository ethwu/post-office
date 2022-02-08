use clap::Parser;
use post_office::Expression;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// A post-tonal expression. Use curly braces (`{}`) for unordered
    /// collections and square brackets (`[]`) for ordered collections.
    expression: String,

    /// Which note pitch class 0 refers to. Defaults to C.
    #[clap(short, long, default_value = "C")]
    zero: String,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    println!("{}", Expression::from_str(&args.expression)?);

    Ok(())
}
