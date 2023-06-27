pub(crate) mod cli;
pub(crate) mod commands;
pub(crate) mod converting;
pub(crate) mod json_parsing;
pub(crate) mod native_wrapper;
pub(crate) mod types;

use clap::Parser;

fn main() {
    println!("Hello, world!");

    let cli = cli::Cli::parse();

    let output = cli.output_path.expect("output_path not presented");

    match &cli.command {
        cli::Commands::Sum {
            first_data_path,
            second_data_path,
        } => {
            let a_collection = json_parsing::get_a_collection_from_file(
                &first_data_path
                    .clone()
                    .expect("first-data-path not presented"),
            );
            let b_collection = json_parsing::get_b_collection_from_file(
                &second_data_path
                    .clone()
                    .expect("second-data-path not presented"),
            );

            let result_value = unsafe { commands::sum(a_collection, b_collection) };

            let result_json = serde_json::json!({ "result": result_value });
            std::fs::write(output, serde_json::to_string_pretty(&result_json).unwrap()).unwrap();
        }
        cli::Commands::Nothing {} => {}
    }

}
