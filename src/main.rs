use clap::{App, Arg, SubCommand};
use rusoto_core::Region;
use rusoto_core::RusotoError;
use rusoto_sqs::*;

fn main() {
    let authors: &str = &vec!["Justin Lam", "Paolo Napolitano"].join(", ");
    let matches = App::new("raws")
        .version("0.1")
        .author(authors)
        .about("AWS SQS stuff")
        .subcommand(
            SubCommand::with_name("sqs")
                .subcommand(SubCommand::with_name("list-queues"))
                .subcommand(SubCommand::with_name("list-messages"))
                .subcommand(SubCommand::with_name("download-messages")),
        )
        .get_matches();

    // list-queues
    // list-messages <queue-url>
    // download-messages <queue-url>
    if let Some(sqs_matches) = matches.subcommand_matches("sqs") {
        match sqs_matches.subcommand_name() {
            Some("list-queues") => {
                let sqs = SqsClient::new(Region::ApSoutheast2);
                let request = ListQueuesRequest::default();
                let result: Result<ListQueuesResult, RusotoError<ListQueuesError>> =
                    sqs.list_queues(request).sync();
                match result {
                    Ok(list_queues_results) => list_queues_results
                        .queue_urls
                        .unwrap_or_default()
                        .iter()
                        .map(|url| url.split("/").last().unwrap_or_default())
                        .map(|name| println!("{}", name))
                        .collect(),
                    // TODO extract the Message from the error
                    Err(rusoto_error) => {
                        dbg!(rusoto_error);
                    }
                }
            }
            Some("list-messages") => println!("list-messages"),
            Some("download-messages") => println!("download-messages"),
            _ => println!("NOT ALLOWED"),
        }
        // if let Some(_) = sqs_matches.subcommand_matches("list-queues") {
        //   println!("list queues");
        // } else {
        //   println!("...");
        // }
    }
}
