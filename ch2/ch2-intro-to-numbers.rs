fn main() {
  let twenty = 20;                       //<1>
  let twenty_one: i32 = 21;              //<2>
  let twenty_two = 22i32;                //<3>

  let addition = twenty + twenty_one + twenty_two;
  println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

  let one_million: i64 = 1_000_000;      //<4>
  println!("{}", one_million.pow(2));    //<5>

  let forty_twos = [                     //<6>
    42.0,                                //<7>
    42f32,                               //<8>
    42.0_f32,                            //<9>
  ];

  println!("{:02}", forty_twos[0]);      //<10>
}
