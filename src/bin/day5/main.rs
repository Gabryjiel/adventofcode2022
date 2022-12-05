use std::borrow::BorrowMut;

fn main() {
  task1(); 
}

fn task1() {
  let input = std::fs::read_to_string("./src/bin/day5/input.txt")
    .expect("Unable to read the file");

  let (data, commands) = input.split_once("\n\n").unwrap();

  let no_of_lines = 36;
  let mut crates_final:  Vec< Vec<char>> =  vec![];

  for _ in 1..no_of_lines {
    crates_final.push( vec![]);
  }

  data
    .split('\n')
    .map(|x| x.chars().enumerate())
    .for_each(|x| x.filter(|(index, value)| value.is_alphabetic()).for_each(|(index, value)| crates_final[index].push(value)));
  
  let mut crates_parsed:  Vec<Vec<char>> = crates_final.iter().filter(|x| x.len() > 0).clone().collect::<Vec<_>>();

  let commands_parsed = commands
    .split('\n')
    .map(|x| String::from_iter(x
      .chars()
      .filter(|x| !x.is_alphabetic())))
    .map(|x| String::from_iter(x.get(1..)))
    .map(|x| x
      .split(' ')
      .map(|x| x
        .parse::<usize>()
        .unwrap_or(0))
      .filter(|x| x > &0)
      .collect::<Vec<_>>())
    .map(|x| (x[0], x[1], x[2]))
    .collect::<Vec<_>>();

  for (amount, start, end) in commands_parsed {
    let mut crates_to_move = crates_parsed[start].get(0..amount).clone().iter().rev().clone().collect::<Vec<_>>().to_vec();

    let mut start_stack = crates_parsed[start].clone().get(0..amount).unwrap().to_vec();
    let mut end_stack = crates_parsed[end].clone().into_iter().chain(&crates_to_move.iter());

  }

  // let res = commands_parsed
  //   .iter()
  //   .for_each(|(amount, start, end)| {
  //     let mut crates_to_move = crates_parsed[*start][0..*amount].iter().rev().collect::<Vec<&char>>().as_mut().collect::<Vec<char>>();

  //     // crates_parsed[*start] = &crates_parsed[*start].get(*amount..);
  //     crates_parsed[*end] = crates_parsed[*end].append(crates_to_move);
  //   });
      

  println!("{:?}", commands_parsed);
}