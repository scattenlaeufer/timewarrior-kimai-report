use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, SubCommand};

mod wrapper;

fn main() {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("config_path")
                .long("config_path")
                .short("c")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("report_wrapper").subcommand(
                SubCommand::with_name("install").arg(
                    Arg::with_name("timew_path")
                        .long("timew_path")
                        .short("p")
                        .takes_value(true)
                        .help("Path to the Timewarrior directory"),
                ),
            ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("report_wrapper") {
        dbg!(matches);
        if let Some(matches) = matches.subcommand_matches("install") {
            wrapper::install_report(matches.value_of("timew_path").map(|p| p.into())).unwrap();
        } else {
            wrapper::test_wrapper();
        }
    } else {
        kimai_report::run(matches.value_of("config_path").map(|p| p.into())).unwrap();
    }
}
