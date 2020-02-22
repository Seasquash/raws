pub fn output_formatter(output: Vec<String>) -> String {
  let padding_size = 2;
  let max_length = output
      .iter()
      .fold(0, |acc, item| {
          if (item.len()) > acc
              {item.len() + padding_size}
          else {acc}
      });
  let delimiter = "-".repeat(max_length);
  output
      .iter()
      .fold(format!("|{}|", delimiter), |acc, item| {
          let right_empty_space = max_length - item.len();
          let right_filler = if right_empty_space > padding_size
              {" ".repeat(right_empty_space - padding_size)}
          else {"".into()};
          format!("{}\n| {} {}|\n|{}|", acc, item, right_filler, delimiter)
      })
}
