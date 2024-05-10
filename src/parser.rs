use std::{collections::HashMap, vec};

use regex::Regex;

#[derive(Debug)]
pub struct Incidence {
  incidences: usize,
  id: u16,
}

type ItemMatchV2 = HashMap<(usize, usize), Incidence>;

pub struct Parser {
  global: usize,
  pub merges: ItemMatchV2,
  pub inverse_merges: HashMap<usize, (usize, usize)>
}

impl Parser {
  pub fn new() -> Self {
    Parser {
      global: 1,
      merges: HashMap::new(),
      inverse_merges: HashMap::new()
    }
  }

  pub fn save_merge(&mut self, key: &(usize, usize), value: &mut usize) {
    match self.merges.get_mut(key) {
      Some(global_result) => {
        global_result.incidences = *value;
      }
      None => {
        let id = 255 + self.global as u16;
        self.merges.insert(*key, Incidence {
          id,
          incidences: *value // this actually should be 1
        });
        self.inverse_merges.insert(id as usize, *key);
        self.global += 1;
      }
    }
  }

  pub fn parse(&mut self, target: &Vec<usize>) -> &ItemMatchV2 {
    let mut dict: HashMap<(usize, usize), usize> = HashMap::new();
    let mut target_bytes = target.iter().peekable();
    while let Some(current_byte) = target_bytes.next() {
      if let Some(next_byte) = target_bytes.peek() {
        match dict.get_mut(&(*current_byte, **next_byte)) {
          Some(result) => {
            *result += 1;
            self.save_merge(&(*current_byte, **next_byte), result);
            break;
          }
          None => {
            dict.insert((*current_byte, **next_byte), 1);
          }
        }
      }
    }
    &self.merges
  }

  pub fn replace(&mut self, target: &Vec<usize>) -> Vec<usize> {
    let mut target_as_bytes = target.iter().peekable();
    let mut result: Vec<usize> = Vec::new();
    while let Some(current_byte) = target_as_bytes.next() {
      if let Some(next_byte) = target_as_bytes.peek() {
        match self.merges.get(&(*current_byte, **next_byte)) {
          Some(global_result) => { // by definition only 2 >= are here
            result.push(global_result.id as usize);
            target_as_bytes.next();
          }
          _ => {
            // if is not electable for replacement
            result.push(*current_byte);
          }
        }
      } else {
        // if is the last char
        result.push(*current_byte);
      }
    }
    result
  }
}

pub fn decode(target: &str, vocab: Vec<String>) -> Vec<String> {
  let regex_pattern = vocab
    .iter()
    .map(|s| regex::escape(&s))
    .collect::<Vec<_>>()
    .join("|");
  let regex = Regex::new(&regex_pattern).unwrap();

  let mut result = Vec::new();
  let mut last_end = 0;

  for capture in regex.captures_iter(target) {
      let start = capture.get(0).unwrap().start();
      let end = capture.get(0).unwrap().end();

      if start > last_end {
          result.push(target[last_end..start].to_string());
      }

      result.push(target[start..end].to_string());

      last_end = end;
  }

  if last_end < target.len() {
      result.push(target[last_end..].to_string());
  }

  result
}

pub fn resolve_references(data: &HashMap<usize, (usize, usize)>, key: usize) -> Vec<u8> {
  if let Some(&(first, second)) = data.get(&key) {
      let mut resolved_values: Vec<u8> = vec![first as u8, second as u8];

      if data.contains_key(&first) {
          let first_resolved = resolve_references(data, first);
          let last = resolved_values[1];
          resolved_values[0] = first_resolved[0];
          resolved_values[1] = first_resolved[1];
          resolved_values.push(last);
      }

      if data.contains_key(&second) {
          let second_resolved = resolve_references(data, second);
          resolved_values.pop(); // Remove the original second value
          resolved_values.extend_from_slice(&second_resolved); // Append resolved second values
      }

      resolved_values
  } else {
      vec![]
  }
}