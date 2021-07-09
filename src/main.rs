use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg};

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
        .get_matches();
    kimai_report::run(matches.value_of("config_path").map(|p| p.into())).unwrap();
}
