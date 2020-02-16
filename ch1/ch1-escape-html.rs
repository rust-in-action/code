fn escape_html(maybe_html: &str) -> String {
  let mut out = String::with_capacity(maybe_html.len());

  for c in maybe_html.chars() {
    match c {
      '<' => out.push_str("&lt;"),
      '>' => out.push_str("&gt;"),
      '&' => out.push_str("&amp;"),
      '\'' => out.push_str("&apos;"),
      '"' => out.push_str("&quot;"),
      _   => out.push(c),
    };
  }

  out
}

fn main() {
  let html = "<p>\"Hello, World!\"</p>";
  let escaped_html = escape_html(html);
  println!("{}", escaped_html);
}
