use clap::{Arg, App, SubCommand};

fn main() {
  let authors : &str = &vec!("Justin Lam", "Paolo Napolitano").join(", ");
  let matches =
    App::new("raws")
      .version("0.1")
      .author(authors)
      .about("AWS SQS stuff")
      .subcommand(SubCommand::with_name("sqs")
        .subcommand(SubCommand::with_name("list-queues"))
      )
      .get_matches();

   if let Some(matches) = matches.subcommand_matches("sqs") {
        println!("sqs");
        if let Some(_) = matches.subcommand_matches("list-queues") {
          println!("list queues");
        } else {
          println!("...");
        }
    }
}
