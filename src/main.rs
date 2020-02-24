use clap::{App, Arg, ArgMatches, SubCommand};
use rusoto_core::Region;
use rusoto_sqs::*;
use std::error::Error;

mod commands;
mod output;
use commands::sqs::{ list_message, list_queue, download_message, info_queue };
use output::formatters::output_formatter;

fn sqs_subcommand_handler(
    sqs: SqsClient,
    arg_matches: &ArgMatches<'_>,
) -> Result<Vec<String>, Box<dyn Error>> {
    // list-queues
    // list-messages <queue-url>
    // download-messages <queue-url>
    match arg_matches.subcommand_name() {
        Some("list-queues") => Ok(list_queue::handler(sqs)?),
        Some("info-queue") => Ok(info_queue::handler(
            sqs,
            arg_matches
                .subcommand_matches("info-queue")
                .unwrap()
                .value_of("queue-name")
                .expect("Queue name not provided"),
        )?),
        Some("list-messages") => Ok(list_message::handler(
            sqs,
            arg_matches
                .subcommand_matches("list-messages")
                .unwrap()
                .value_of("queue-name")
                .expect("Queue name not provided"),
        )?),
        Some("download-messages") => Ok(download_message::handler(
            sqs,
            arg_matches
                .subcommand_matches("download-messages")
                .unwrap()
                .value_of("queue-name")
                .expect("Queue name not provided"),
            arg_matches
                .subcommand_matches("download-messages")
                .unwrap()
                .is_present("delete")
        )?),
        _ => unimplemented!(),
    }
}

fn main() {
    let authors: &str = &vec!["Justin Lam", "Paolo Napolitano"].join(", ");
    let matches = App::new("raws")
        .version("0.1")
        .author(authors)
        .about("AWS SQS stuff")
        .subcommand(
            SubCommand::with_name("sqs")
                .subcommand(SubCommand::with_name("list-queues"))
                .subcommand(
                    SubCommand::with_name("info-queue")
                        .arg(Arg::with_name("queue-name").required(true).index(1)),
                )
                .subcommand(
                    SubCommand::with_name("list-messages")
                        .arg(Arg::with_name("queue-name").required(true).index(1)),
                )
                .subcommand(
                    SubCommand::with_name("download-messages")
                        .arg(Arg::with_name("queue-name").required(true).index(1))
                        .arg(Arg::from_usage("--delete").allow_hyphen_values(true)),
                )
        )
        .get_matches();
    let sqs = SqsClient::new(Region::ApSoutheast2);
    if let Some(sqs_matches) = matches.subcommand_matches("sqs") {
        match sqs_subcommand_handler(sqs, sqs_matches) {
            Err(e) => {
                dbg!(e);
            }
            Ok(result) => {
                if result.is_empty()
                {println!("No results")}
                else {println!("{}", output_formatter(result))}
            }
        }
    }
}
