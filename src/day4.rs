use aoc_2024::lib;

#[allow(dead_code)]
pub fn part1() -> Result<i32, String> {
  let vecs = get_vecs()?;

  let searcher = Searcher {
    directions: [(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)],
    target_word: ['X', 'M', 'A', 'S'],
    vecs: vecs.clone()
  };

  let mut total_found = 0;
  for (x, vec) in vecs.iter().enumerate() {
    for (y, char) in vec.iter().enumerate() {
      if *char == searcher.target_word[0] {
        total_found += start_search((x.try_into().unwrap(), y.try_into().unwrap()), &searcher)
      }
    }
  }

  Ok(total_found)
}

fn get_vecs() -> Result<Vec<Vec<char>>, String> {
  let input = lib::read_file("resources/day4.txt").map_err(|e| e.to_string())?;
  let lines = input.split("\n").fold(Vec::new(), |mut acc, line| {
    let line_chars: Vec<char> = line.chars().collect();
    acc.push(line_chars);
    acc
  });
  return Ok(lines);
}

struct Searcher {
  directions: [(i32, i32); 8],
  vecs: Vec<Vec<char>>,
  target_word: [char; 4]
}

fn start_search(position: (i32, i32), searcher: &Searcher) -> i32 {
  let mut total_found = 0;
  for direction in searcher.directions {
    total_found += search_direction(direction, position, 1, searcher);
  }
  total_found
}

fn search_direction(direction: (i32, i32), position: (i32, i32), curr_target_index: usize, searcher: &Searcher) -> i32 {
  let moved_position = (position.0 + direction.0, position.1 + direction.1);

  if moved_position.0 < 0 
  || moved_position.1 < 0 
  || moved_position.0 as usize >= searcher.vecs[0].len() 
  || moved_position.1 as usize >= searcher.vecs.len() {
    return 0;
  }

  let char_at_position = searcher.vecs[moved_position.0 as usize][moved_position.1 as usize];

  if char_at_position == searcher.target_word[curr_target_index] {
    if curr_target_index < 3 {
      search_direction(direction, moved_position, curr_target_index + 1, searcher)
    } else {
      1
    }
  } else {
    0
  }
}

#[allow(dead_code)]
pub fn part2() -> Result<i32, String> {
  let vecs = get_vecs()?;

  let mut total_found = 0;
  for (x, vec) in vecs.iter().enumerate() {
    for (y, char) in vec.iter().enumerate() {
      if *char == 'A' {
        total_found += search_x((x.try_into().unwrap(), y.try_into().unwrap()), &vecs)
      }
    }
  }

  Ok(total_found)
}

fn search_x(position: (i32, i32), vecs: &Vec<Vec<char>>) -> i32 {
  if position.0 - 1 < 0 || position.1 -1 < 0 
  || (position.0 + 1) as usize >= vecs[0].len() 
  || (position.1 + 1) as usize >= vecs.len() {
    return 0;
  }

  if (vecs[(position.0 - 1) as usize][(position.1 - 1) as usize] == 'M' 
  && vecs[(position.0 + 1) as usize][(position.1 + 1) as usize] == 'S')
  || (vecs[(position.0 - 1) as usize][(position.1 - 1) as usize] == 'S' 
  && vecs[(position.0 + 1) as usize][(position.1 + 1) as usize] == 'M') {
    if (vecs[(position.0 - 1) as usize][(position.1 + 1) as usize] == 'M' 
    && vecs[(position.0 + 1) as usize][(position.1 - 1) as usize] == 'S')
    || (vecs[(position.0 - 1) as usize][(position.1 + 1) as usize] == 'S' 
    && vecs[(position.0 + 1) as usize][(position.1 - 1) as usize] == 'M') {
      return 1;
    }
  }

  return 0
}