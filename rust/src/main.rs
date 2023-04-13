#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
include!("jsonParsing.rs");

fn sumCommand(arr1: &[f64], arr2: &[f64]) -> f64 {
    let mut result = Vec::with_capacity(arr1.len() + arr2.len());

    result.extend_from_slice(&arr1);
    result.extend_from_slice(&arr2);

    let nativeSum: f64 = result.iter().sum();
    println!("native sum: {}", nativeSum);

    unsafe {
        let size = arr1.len() / 2;
        let a = allocACollection(size.try_into().unwrap());
        let b = allocBCollection(size.try_into().unwrap());

        for n in 0..size {
            let mut element = a.data.offset(n.try_into().unwrap());
            (*element).first = arr1[n * 2];
            (*element).second = arr1[n * 2 + 1];
            println!("A: {{ {} , {} }}", (*element).first, (*element).second);
        }

        for n in 0..size {
            let mut element = b.data.offset(n.try_into().unwrap());
            (*element).first = arr2[n * 2];
            (*element).second = arr2[n * 2 + 1];
            println!("B: {{ {} , {} }}", (*element).first, (*element).second);
        }

        let sumOfAAndB = sum(a, b);
        println!("sum: {}", sumOfAAndB);

        return sumOfAAndB;
    }
}

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

            let first_data = parseDataFromFile(first_data_path);
            let second_data = parseDataFromFile(second_data_path);

            let result_value = sumCommand(first_data.data.as_slice(), second_data.data.as_slice());

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
