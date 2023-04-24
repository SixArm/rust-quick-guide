use console::Term;
use dialoguer::Input;
use indicatif::ProgressBar;

fn main() -> std::io::Result<()> {

    // console example
    let term = Term::stdout();
    term.write_line("Hello, World!")?;

    // dialoguer example
    let input = Input::<String>::new().interact_text()?;
    term.write_line(&input)?;

    // indicatif example
    let bar = ProgressBar::new(10);
    for _ in 0..10 { bar.inc(1); }
    bar.finish();

    Ok(())
}
