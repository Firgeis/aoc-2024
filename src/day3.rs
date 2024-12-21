use aoc_2024::lib::read_file;

#[allow(dead_code)]
pub fn part1() -> Result<i32, String> {
  let input = read_file("resources/day3.txt").map_err(|err| err.to_string())?;

  let mut chars = input.chars();
  let mut mults = vec![];
  let mut enabled = true;

  while let Some(c) = chars.next() {
    match start_word(&mut chars, c) {
      None => continue,
      Some(parser_result) => {
        match parser_result {
          ParserResult::Enabled(x) => enabled = x,
          ParserResult::Number(x) => if enabled { mults.push(x) }
        }
      },
    }
  }

  let result = mults.iter().copied().reduce(|acc, x| acc + x);

  match result {
    None => Err("Empty".to_string()),
    Some(x) => Ok(x)
  }
}

struct MulParser<I> where I: Iterator {
  chars: I,
  ints: Vec<Vec<char>>
}

enum ParserResult {
  Number(i32),
  Enabled(bool)
}

fn start_word<I>(chars: I, current:char) -> Option<ParserResult>
where I: Iterator<Item = char>
{

  match current {
    x if x == 'd' => {
      let mul_parser = MulParser {
        chars: chars,
        ints: vec![]
      };

      continue_with(mul_parser, 'o', 
      |m| continue_with_or(m, '(', 'n', 
      Box::new(|m| continue_with(m, ')', 
      |_| Some(ParserResult::Enabled(true)))), 
      Box::new(|m| continue_with(m, '\'', 
        |m| continue_with(m, 't', |_| Some(ParserResult::Enabled(false)))))))
    },
    x if x == 'm' => {
      let mul_parser = MulParser {
        chars: chars,
        ints: vec![]
      };

      continue_with(mul_parser, 'u', 
      |m| continue_with( m, 'l', 
      |m| continue_with( m, '(', 
      |m| parse_int_until(m, ',', 
      |m| parse_int_until(m, ')',
      |m| calculate_result(m))))))
    },
    _ => None
  }
}

fn continue_with<I, F>(mut parser: MulParser<I>, with: char, f: F) -> Option<ParserResult>
where I: Iterator<Item = char>, F: FnOnce(MulParser<I>) -> Option<ParserResult> {
  let next = parser.chars.next();

  match next {
    None => None,
    Some(x) => if x == with { f(parser) } else { return None }
  }
}

fn continue_with_or<I>(mut parser: MulParser<I>, with: char, or_with: char, f: Box<dyn FnOnce(MulParser<I>) -> Option<ParserResult>>, or_f: Box<dyn FnOnce(MulParser<I>) -> Option<ParserResult>>) -> Option<ParserResult>
where I: Iterator<Item = char> {
  let next = parser.chars.next();

  match next {
    Some(x) if x == with => f(parser),
    Some(x) if x == or_with => or_f(parser),
    _ => None
  }
}

fn parse_int_until<I, F>(mut parser: MulParser<I>, until: char, f: F) -> Option<ParserResult>
where I: Iterator<Item = char>, F: FnOnce(MulParser<I>) -> Option<ParserResult> {
  let mut next = parser.chars.next();
  parser.ints.push(vec![]);

  while next.is_some() {
    let next_val = next.unwrap();

    if next_val.is_digit(10) {
      match parser.ints.last_mut() {
        None => return None,
        Some(x) => x.push(next_val)
      };
      next = parser.chars.next();
    } else if next_val != until {
      return None
    } else if next_val == until {
      break;
    }
  }

  f(parser)
}

fn calculate_result<I>(parser: MulParser<I>) -> Option<ParserResult> 
where I: Iterator<Item = char> {
  let first_int = String::from_iter(parser.ints[0].iter()).parse::<i32>().map_or_else(|_| None, |x| Some(x));
  let second_int = String::from_iter(parser.ints[1].iter()).parse::<i32>().map_or_else(|_| None, |x| Some(x));

  match (first_int, second_int) {
    (Some(x), Some(y)) => Some(ParserResult::Number(x * y)),
    _ => None
  }
}
