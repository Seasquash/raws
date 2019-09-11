use clap::{App, ArgMatches, SubCommand};
use rusoto_core::Region;
use rusoto_core::RusotoError;
use rusoto_sqs::*;
// use std::error::Error;
use failure::Error;

// subcommand_handler.... for sqs only
fn sqs_subcommand_handler(sqs: SqsClient, arg_matches: &ArgMatches<'_>) -> Result<(), Error> {
    match arg_matches.subcommand_name() {
        Some("list-queues") => Ok(list_queue_handler(sqs)?),
        Some("list-messages") => Ok(list_message_handler(sqs)?),
        _ => unimplemented!(),
    }
}

fn list_queue_handler(sqs: SqsClient) -> Result<(), RusotoError<ListQueuesError>> {
    let request = ListQueuesRequest::default();
    Ok(sqs
        .list_queues(request)
        .sync()?
        .queue_urls
        .unwrap_or_default()
        .iter()
        .map(|url| url.split("/").last().unwrap_or_default())
        .map(|name| println!("{}", name))
        .collect())
}

fn list_message_handler(sqs: SqsClient) -> Result<(), RusotoError<ReceiveMessageError>> {
    unimplemented!()
}

// list-queues handler
// list-message handler
// download-message handler

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
    let sqs = SqsClient::new(Region::ApSoutheast2);
    if let Some(sqs_matches) = matches.subcommand_matches("sqs") {
        match sqs_subcommand_handler(sqs, sqs_matches) {
            Err(e) => {
                dbg!(e);
                ()
            }
            _ => (),
        }
    }
    // match sqs_matches.subcommand_name() {
    //     Some("list-queues") => {
    //         let request = ListQueuesRequest::default();
    //         let result = sqs.list_queues(request).sync();
    //         match result {
    //             Ok(list_queues_results) => list_queues_results
    //                 .queue_urls
    //                 .unwrap_or_default()
    //                 .iter()
    //                 .map(|url| url.split("/").last().unwrap_or_default())
    //                 .map(|name| println!("{}", name))
    //                 .collect(),
    //             // TODO extract the Message from the error
    //             Err(rusoto_error) => {
    //                 dbg!(rusoto_error);
    //             }
    //         }
    //     }
    //     Some("list-messages") => {
    //         let request = ReceiveMessageRequest{
    //             queue_url: String::from("https://sqs.ap-southeast-2.amazonaws.com/954088256298/rust-aws-integration"),
    //             max_number_of_messages: Some(10),
    //             ..Default::default()
    //         };
    //         let result = sqs.receive_message(request).sync();
    //         match result {
    //             Ok(message_results) => message_results
    //                 .messages
    //                 .unwrap_or_default()
    //                 .iter()
    //                 .map(|message| {
    //                     if let Some(x) = &message.body {
    //                         println!("{}", x);
    //                     }
    //                 })
    //                 .collect(),
    //             Err(rusoto_error) => {
    //                 dbg!(rusoto_error);
    //             }
    //         }
    //     },
    //     Some("download-messages") => println!("download-messages"),
    //     _ => println!("NOT ALLOWED"),
    // }
    // }
}
