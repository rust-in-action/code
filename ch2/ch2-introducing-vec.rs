fn main() {
  let ctx_lines = 2;
  let needle = "oo";
  let haystack = "\
Every face, every shop,
bedroom window, public-house, and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

  let mut tags: Vec<usize> = vec![];               // <1>
  let mut ctx: Vec<Vec<(
               usize, String)>> = vec![];          // <2>

  for (i, line) in haystack.lines().enumerate() {  // <3>
    if line.contains(needle) {
      tags.push(i);

      let v = Vec::with_capacity(2*ctx_lines + 1); // <4>
      ctx.push(v);
    }
  }

  if tags.is_empty() {                             // <5>
    return;
  }

  for (i, line) in haystack.lines().enumerate() {  // <6>
    for (j, tag) in tags.iter().enumerate() {
      let lower_bound =
	      tag.saturating_sub(ctx_lines);           // <7>
      let upper_bound =
	      tag + ctx_lines;

      if (i >= lower_bound) && (i <= upper_bound) {
          let line_as_string = String::from(line); // <8>
          let local_ctx = (i, line_as_string);
          ctx[j].push(local_ctx);
      }
    }
  }

  for local_ctx in ctx.iter() {
    for &(i, ref line) in local_ctx.iter() {       // <9>
      let line_num = i + 1;
      println!("{}: {}", line_num, line);
    }
  }
}
