use clap::{ArgAction, ArgMatches, arg, command, value_parser};
use miette::{Result, miette};
use std::path::PathBuf;

mod search;
mod writer;

use search::Searcher;
use writer::TemplateWriter;

fn handle_commands() -> ArgMatches {
    command!()
        .arg(
            arg!(-p --prepend "Prepend an additional search path to search through")
                .action(ArgAction::Append)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(-a --append "Append an additional search path to search through")
                .action(ArgAction::Append)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(<TEMPLATE> "The template to use").value_parser(value_parser!(PathBuf)))
        .arg(
            arg!([OUTPUT] "The path to output the template to")
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches()
}

fn main() -> Result<()> {
    let matches: ArgMatches = handle_commands();

    let mut prepend: Vec<PathBuf> = Vec::new();
    for path in matches.get_many::<PathBuf>("prepend").unwrap_or_default() {
        prepend.push(path.to_owned());
    }

    let mut append: Vec<PathBuf> = Vec::new();
    for path in matches.get_many::<PathBuf>("append").unwrap_or_default() {
        append.push(path.to_owned());
    }

    let searcher: Searcher =
        Searcher::new(prepend, append).ok_or(miette!(
            help = "in your current state, you are required to manually specify the search paths for templato to avoid this error. \
                Use either $TEMPLATO_SEARCH_PATHS or specify a search path manually on the command line to avoid receiving this error.",
            "the home directory could not be accessed"
        ))?;

    let template_path: &PathBuf = matches.get_one::<PathBuf>("TEMPLATE").unwrap();
    let template_str = template_path.display();
    let template: String = searcher
        .find(template_path)
        .ok_or(miette!("could not find template {template_str}"))?;

    let output_path: PathBuf = if let Some(path) = matches.get_one::<PathBuf>("OUTPUT") {
        path.to_owned()
    } else if let Some(path) = template_path.file_name() {
        path.into()
    } else {
        "template".into()
    };
    TemplateWriter::new(template).write(&output_path)?;
    println!(
        "templato: successfully wrote template to path \"{}\"",
        output_path.display()
    );

    Ok(())
}
