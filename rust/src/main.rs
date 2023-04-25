pub(crate) mod commands;
pub(crate) mod converting;
pub(crate) mod json_parsing;
pub(crate) mod native_wrapper;
pub(crate) mod types;

fn main() {
    println!("Hello, world!");

    let cmd = clap::Command::new("command")
        .bin_name("command")
        .subcommand_required(true)
        .subcommand(
            clap::command!("sum")
                .arg(
                    clap::arg!(--"first-data-path" <PATH>)
                        .value_parser(clap::value_parser!(std::path::PathBuf)),
                )
                .arg(
                    clap::arg!(--"second-data-path" <PATH>)
                        .value_parser(clap::value_parser!(std::path::PathBuf)),
                ),
        )
        .subcommand(clap::command!("donothing"))
        .arg(
            clap::arg!(--"output-path" <PATH>)
                .value_parser(clap::value_parser!(std::path::PathBuf)),
        );

    let matches = cmd.get_matches();

    let output_path = matches
        .get_one::<std::path::PathBuf>("output-path")
        .expect("output not presented");

    match matches.subcommand() {
        Some(("sum", matches)) => {
            let first_data_path = matches
                .get_one::<std::path::PathBuf>("first-data-path")
                .expect("first-data-path not presented");
            let second_data_path = matches
                .get_one::<std::path::PathBuf>("second-data-path")
                .expect("second-data-path not presented");

            let a_collection = json_parsing::get_a_collection_from_file(first_data_path);
            let b_collection = json_parsing::get_b_collection_from_file(second_data_path);

            let result_value = unsafe { commands::sum(&a_collection, &b_collection) };

            let result_json = serde_json::json!({ "result": result_value });
            std::fs::write(
                output_path,
                serde_json::to_string_pretty(&result_json).unwrap(),
            )
            .unwrap();
        }
        Some(("donothing", _)) => {
            println!("do nothing");
        }
        _ => unreachable!("clap should ensure we don't get here"),
    };
}
