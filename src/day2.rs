use std::num::ParseIntError;
use aoc_2024::lib;

#[allow(dead_code)]
pub fn part1() -> Result<i32, String> {
  run_report(false)
}

#[allow(dead_code)]
pub fn part2() -> Result<i32, String> {
  run_report(true)
}

fn run_report(dampener: bool) -> Result<i32, String> {
  let input = lib::read_file("resources/day2.txt").map_err(|err| err.to_string())?;
  let lines = input.split('\n');

  let mut safe_reports = 0;
  for line in lines {
    let split: Result<Vec<i32>, ParseIntError> = line.split(' ').map(|x| x.parse::<i32>()).into_iter().collect();
    let parsed = split.map_err(|e| e.to_string())?;
    if level_is_safe(parsed, dampener, false) {
      safe_reports += 1;
    }
  }

  Ok(safe_reports)
}

#[derive(PartialEq)]
enum LevelDirection { Increasing(), Decreasing() }

fn level_is_safe(vec: Vec<i32>, dampener: bool, dampener_state: bool) -> bool {
  let level_direction = match (vec[0], vec[1]) {
    (x, y) if x > y => LevelDirection::Decreasing(),
    (x, y) if x < y => LevelDirection::Increasing(),
    _ => return trigger_dampener(vec, dampener, dampener_state)
  };

  for (index, value) in vec.iter().enumerate().skip(1) {
    let prev_value = vec[index - 1];
    let diff = (*value - prev_value).abs();

    if diff < 1 || diff > 3 {
      return trigger_dampener(vec, dampener, dampener_state)
    }

    if level_direction == LevelDirection::Increasing() && *value <= prev_value {
      return trigger_dampener(vec, dampener, dampener_state)
    }

    if level_direction == LevelDirection::Decreasing() && *value >= prev_value {
      return trigger_dampener(vec, dampener,dampener_state)
    }
  }

  return true;
}

fn trigger_dampener (vec: Vec<i32>, dampened: bool, dampener_state: bool) -> bool {
  if dampened && !dampener_state {
    for index in 0..vec.len() {
      let mut new_vec= vec.clone();
      new_vec.remove(index);
      let result = level_is_safe(new_vec, true, true);
      if !result {
        continue;
      } else {
        return true;
      }
    }
  }

  return false;
}