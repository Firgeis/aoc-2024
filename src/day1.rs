use aoc_2024::lib;

#[allow(dead_code)]
pub fn part1() -> Result<i32, String> {
  let vecs = get_vecs()?;
  let mut result = 0;
  for (index, value) in vecs.0.iter().enumerate() {
    let other = vecs.1.get(index).ok_or_else(|| "Wrong index")?;
    result = result + (value - other).abs();
  }

  Ok(result)
}

#[allow(dead_code)]
pub fn part2() -> Result<i32, String> {
  let vecs = get_vecs()?;

  let mut result = 0;
  for v in vecs.0.iter() {
    let times = i32::try_from(vecs.1.iter().skip_while(|x| *x < v).take_while(|x| *x == v).count()).map_err(|err| err.to_string())?;
    result += v * times;
  }

  Ok(result)
}

fn get_vecs() -> Result<(Vec<i32>, Vec<i32>), String> {
  let input = lib::read_file("resources/day1.txt").map_err(|err| err.to_string())?;
  let split = input.split('\n');
  let mut vec1 = Vec::new();
  let mut vec2 = Vec::new();

  for line in split {
    let parsed_line: Vec<&str> = line.split("   ").collect();
    let value_fst = parsed_line.first().ok_or_else(|| "Could not parse fst")?;
    let value_snd = parsed_line.last().ok_or_else(|| "Could not parse snd")?;
    let parsed_fst = value_fst.parse::<i32>().map_err(|err| err.to_string())?;
    let parsed_snd = value_snd.parse::<i32>().map_err(|err| err.to_string())?;
    vec1.push(parsed_fst);
    vec2.push(parsed_snd);
  }

  vec1.sort();
  vec2.sort();

  Ok((vec1, vec2))
}