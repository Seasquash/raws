use clap::{App, Arg, ArgMatches, SubCommand};
use rusoto_core::{Region, RusotoError};
use rusoto_sqs::*;
use std::env;
use std::error::Error;

fn sqs_subcommand_handler(
    sqs: SqsClient,
    arg_matches: &ArgMatches<'_>,
) -> Result<Vec<String>, Box<dyn Error>> {
    match arg_matches.subcommand_name() {
        Some("list-queues") => Ok(list_queue_handler(sqs)?),
        Some("list-messages") => Ok(list_message_handler(
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

fn list_queue_handler(sqs: SqsClient) -> Result<Vec<String>, RusotoError<ListQueuesError>> {
    let request = ListQueuesRequest::default();
    Ok(sqs
        .list_queues(request)
        .sync()?
        .queue_urls
        .unwrap_or_default()
        .iter()
        .map(|url| url.split("/").last().map(|x| x.into()))
        .filter_map(|m| m)
        .collect::<Vec<String>>()
    )
}

fn construct_queue_url(queue_name: &str) -> Result<String, Box<dyn Error>> {
    Ok(String::from(format!(
        "https://sqs.{region}.amazonaws.com/{account}/{queue_name}",
        region = env::var("AWS_DEFAULT_REGION").expect("AWS REGION NOT FOUND"),
        account = env::var("AWS_ACCOUNT").expect("AWS ACCOUNT NOT FOUND"),
        queue_name = queue_name
    )))
}

fn retrieve_messages(client: &SqsClient, request: &ReceiveMessageRequest) -> Result<Vec<Option<String>>, Box<dyn Error>> {
    Ok(client
        .receive_message(request.clone())
        .sync()?
        .messages
        .unwrap_or_default()
        .iter()
        .map(|message| message.clone().body)
        .collect::<Vec<Option<String>>>())
}

fn retrieve_all_messages(client: &SqsClient, request: &ReceiveMessageRequest, result: Vec<String>) -> Vec<String> {
    let msgs = retrieve_messages(client, request)
        .map(|messages| {
            messages
                .iter()
                .cloned()
                .filter_map(|m| m)
                .collect::<Vec<String>>()
        });
    // if Err or Vec is empty, return result
    // otherwise, call retrieve_all_messages passing results + Vec
    match msgs {
        Ok(v) => {
            if v.is_empty() {
                result
            } else {
                let new_result = result.into_iter().chain(v.into_iter()).collect();
                retrieve_all_messages(&client, &request, new_result)
            }
        },
        Err(e) => {
            println!("An error occurred: {}", e);
            result
        }
    }
}

// "https://sqs.ap-southeast-2.amazonaws.com/954088256298/rust-aws-integration"
fn list_message_handler(sqs: SqsClient, queue_name: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let request = ReceiveMessageRequest {
        queue_url: construct_queue_url(queue_name)?,
        max_number_of_messages: Some(10),
        ..Default::default()
    };
    let messages = retrieve_all_messages(&sqs, &request, vec!());
    Ok(messages)
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
            Ok(result) => println!("{:?}", result),
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
