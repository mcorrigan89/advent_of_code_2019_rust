fn add_numbers(code_array: &mut Vec<i32>, pos1: usize, pos2: usize, pos3: usize) -> &mut Vec<i32> {
  code_array[pos3] = code_array[pos1] + code_array[pos2];
  code_array
}

fn multiply_numbers(
  code_array: &mut Vec<i32>,
  pos1: usize,
  pos2: usize,
  pos3: usize,
) -> &mut Vec<i32> {
  code_array[pos3] = code_array[pos1] * code_array[pos2];
  code_array
}

struct IntCodeReturn<'a> {
  code_array: &'a mut Vec<i32>,
  halt: bool,
}

fn handle_int_code(code: i32, index: usize, code_array: &mut Vec<i32>) -> IntCodeReturn {
  match code {
    1 => IntCodeReturn {
      code_array: add_numbers(code_array, code_array[index + 1] as usize, code_array[index + 2] as usize, code_array[index + 3] as usize),
      halt: false,
    },
    2 => IntCodeReturn {
      code_array: multiply_numbers(code_array, code_array[index + 1] as usize, code_array[index + 2] as usize, code_array[index + 3] as usize),
      halt: false,
    },
    99 => IntCodeReturn {
      code_array: code_array,
      halt: true,
    },
    _ => IntCodeReturn {
      code_array,
      halt: false,
    },
  }
}

fn read_code(code_array: &mut Vec<i32>) -> &mut Vec<i32> {
  let result = code_array.clone().into_iter().enumerate().fold(
    IntCodeReturn {
      code_array,
      halt: false,
    },
    |a, (idx, val)| {
      if !a.halt && idx % 4 == 0 {
        handle_int_code(val, idx, a.code_array)
      } else {
        a
      }
    },
  );
  result.code_array
}

fn program_answer(code_array: &mut Vec<i32>) -> i32 {
  let code = read_code(code_array);
  code[0]
}

pub fn run_program(code_array: &mut Vec<i32>, noun: i32, verb: i32) -> i32 {
  code_array[1] = noun;
  code_array[2] = verb;
  program_answer(code_array)
}

pub struct NounVerbPair {
  pub noun: i32,
  pub verb: i32
}

pub fn brute_force_get_answer(code_array: &mut Vec<i32>, answer: i32) -> NounVerbPair {
  let mut noun_verb_pair = NounVerbPair{ noun: 0, verb: 0 };
  for noun in 0..code_array.len() - 1 {
    for verb in 0..code_array.len() - 1 {
      let mut cloned_array = code_array.clone();
      let result = run_program(&mut cloned_array, noun as i32, verb as i32);
      if result == answer {
        noun_verb_pair = NounVerbPair{ noun: noun as i32, verb: verb as i32 };
      }
    }
  }
  noun_verb_pair
}
// Tests

#[cfg(test)]
#[test]
fn add_numbers_test() {
  let mut code_array = vec![4, 5, 6];
  let result = add_numbers(&mut code_array, 0, 1, 2);
  let expected = vec![4, 5, 9];
  assert_eq!(result.clone(), expected);
}

#[test]
fn multiply_numbers_test() {
  let mut code_array = vec![4, 5, 6];
  let result = multiply_numbers(&mut code_array, 0, 1, 2);
  let expected = vec![4, 5, 20];
  assert_eq!(result.clone(), expected);
}

#[test]
fn handle_int_code_pass_through_test() {
  let mut code_array = vec![3, 4, 5, 6];
  let result = handle_int_code(3, 0, &mut code_array);
  let expected = vec![3, 4, 5, 6];
  assert_eq!(result.halt, false);
  assert_eq!(result.code_array.clone(), expected);
}

#[test]
fn handle_int_code_add_test() {
  let mut code_array = vec![1, 1, 1, 1];
  let result = handle_int_code(1, 0, &mut code_array);
  let expected = vec![1, 2, 1, 1];
  assert_eq!(result.halt, false);
  assert_eq!(result.code_array.clone(), expected);
}

#[test]
fn handle_int_code_multiply_test() {
  let mut code_array = vec![2, 2, 2, 2];
  let result = handle_int_code(2, 0, &mut code_array);
  let expected = vec![2, 2, 4, 2];
  assert_eq!(result.halt, false);
  assert_eq!(result.code_array.clone(), expected);
}

#[test]
fn handle_int_code_halt_test() {
  let mut code_array = vec![99, 4, 5, 6];
  let result = handle_int_code(99, 0, &mut code_array);
  let expected = vec![99, 4, 5, 6];
  assert_eq!(result.halt, true);
  assert_eq!(result.code_array.clone(), expected);
}

#[test]
fn read_works() {
  let mut code_array = vec![1, 0, 0, 0, 99];
  let result = read_code(&mut code_array);
  let expected = vec![2, 0, 0, 0, 99];
  assert_eq!(result.clone(), expected);
}
