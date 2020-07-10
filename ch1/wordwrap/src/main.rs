use regex;
#[macro_use]
use lazy_static::lazy_static;


fn main() {
  // No requirement for the main() function to be at the end of the file.

  // multi-line strings do not have special syntax
  let text_to_wrap = "";
  println!("{}", word_wrap(text_to_wrap, 70));
}


#[cfg(target_os = "windows")]
const NL: &'static str = "\r\n";

#[cfg(not(target_os = "windows"))]
const NL: &'static str = "\n";


fn word_wrap(original: &str, wrap_at: usize) -> String {
  let paragraphs = regex::Regex::new(r"(\r?\n\s?+\r?\n|\u{2029})")
    .expect("malformed regular expression"); // create a 

  let mut wrapped = String::with_capacity(original.len()); //

  let mut paragraphs = paragraphs
    .split(original) // 
    .peekable(); // "Peekable" iterators can view the next element without iterating to it. Iterators can be composed with adapters.

  while let Some(p) = paragraphs.next() {
    let mut space_remaining = wrap_at;
    let words = p.split_whitespace(); // The split_whitespace() method is aware of all Unicode whitespace characters.
    for w in words {
      let to_write = if w.len() >= wrap_at {
        space_remaining = wrap_at;
        format!("{}{}{}", NL, w, NL)
      } else if (w.len() + " ".len()) > space_remaining {
        space_remaining = wrap_at - w.len();
        format!("{}{} ", NL, w)
      } else {
        space_remaining -= w.len() + " ".len();
        format!("{} ", w)
      };
      wrapped.push_str(&to_write);
    }

    if paragraphs.peek().is_some() {
      wrapped.push_str("\n\n");
    }
  }
  wrapped
}

#[test]
fn test_empty_input_is_unaffected() {
  let result = word_wrap("", 80);
  assert_eq!(result, "");
}

#[test]
fn test_long_lines_are_wrapped() {
  let line = "A relatively long line of text that should wrap at a word boundary.";
  let expected = "A relatively long line of 
text that should wrap at a 
word boundary. ";
  let result = word_wrap(line, 30);
  assert_eq!(expected, result);
}

#[test]
fn test_paragraphs_are_treated_independently() {
  let line = "\
A sequence of paragraphs. 
  
One of the paragraphs is too long.";
  let expected = "\
A sequence of paragraphs. 

One of the paragraphs is too 
long. ";
  let result = word_wrap(line, 30);
  assert_eq!(expected, result);
}
