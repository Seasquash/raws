pub fn output_formatter(output: Vec<String>) -> String {
  let padding_size = 2;
  let max_length = output
      .iter()
      .fold(0, |acc, item| {
          if (item.len()) > acc
              {item.len()}
          else {acc}
      });
  let delimiter = "-".repeat(max_length + padding_size);
  output
      .iter()
      .fold(format!("|{}|", delimiter), |acc, item| {
          let right_empty_space = max_length - item.len();
          let right_filler = if right_empty_space > 0
              {" ".repeat(right_empty_space)}
          else {"".into()};
          format!("{}\n| {} {}|\n|{}|", acc, item, right_filler, delimiter)
      })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_an_empty_vector() {
        let expected = "|--|";
        assert_eq!(output_formatter(vec!()), expected);
    }

    #[test]
    fn test_format_mix_length() {
        let strings = vec!("6 char".into(), "8 chars-".into());
        let expected = "|----------|\n| 6 char   |\n|----------|\n| 8 chars- |\n|----------|";
        assert_eq!(output_formatter(strings), expected);
    }
}
