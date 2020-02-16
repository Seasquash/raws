use clap::{App, Arg, ArgMatches, SubCommand};
use rusoto_core::Region;
use rusoto_sqs::*;
use std::error::Error;

mod commands;
use commands::sqs::{list_message, list_queue};

fn sqs_subcommand_handler(
    sqs: SqsClient,
    arg_matches: &ArgMatches<'_>,
) -> Result<Vec<String>, Box<dyn Error>> {
    // list-queues
    // list-messages <queue-url>
    // download-messages <queue-url>
    match arg_matches.subcommand_name() {
        Some("list-queues") => Ok(list_queue::handler(sqs)?),
        Some("list-messages") => Ok(list_message::handler(
            sqs,
            arg_matches
                .subcommand_matches("list-messages")
                .unwrap()
                .value_of("queue-name")
                .expect("Queue name not provided"),
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
                    SubCommand::with_name("list-messages")
                        .arg(Arg::with_name("queue-name").required(true).index(1)),
                )
                .subcommand(SubCommand::with_name("download-messages")),
        )
        .get_matches();
    let sqs = SqsClient::new(Region::ApSoutheast2);
    if let Some(sqs_matches) = matches.subcommand_matches("sqs") {
        match sqs_subcommand_handler(sqs, sqs_matches) {
            Err(e) => {
                dbg!(e);
                ()
            }
            Ok(result) => println!("{:?}", result),
        }
    }
}
