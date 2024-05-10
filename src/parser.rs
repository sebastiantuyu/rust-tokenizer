use std::collections::HashMap;

use regex::Regex;

type ItemMatch = HashMap<usize, HashMap<usize, Incidence>>;

#[derive(Debug)]
pub struct Incidence {
  // merged_from: String,
  incidences: usize,
  id: u16,
}

pub struct Parser {
  global: usize,
  pub merges: HashMap<usize, String>
}

impl Parser {
  pub fn new() -> Self {
    Parser {
      global: 0,
      merges: HashMap::new()
    }
  }

  pub fn get_merges(&mut self) -> HashMap<usize, String> {
    let filtered_merges: HashMap<usize, String> = HashMap::new();
    for (merge_key, merge_value) in self.merges.iter() {

    }
    filtered_merges
  }

  pub fn check_merges(&mut self, t: &usize) -> String {
    if *t >= 255 {
      return self.merges.get(t).unwrap().to_string();
    }
    return String::from_utf8(vec![*t as u8]).unwrap();
  }

  pub fn parse(&mut self, target: &Vec<usize>) -> ItemMatch {
    let mut dict: ItemMatch = HashMap::new();
    let mut target_bytes = target.iter().peekable();
    while let Some(current_byte) = target_bytes.next() {
      match dict.get_mut(current_byte) {
        Some(value) => {
          if let Some(next_byte) = target_bytes.peek() {
            if let Some(exists) = value.get_mut(*next_byte) {
              exists.incidences += 1;
            }
          }
        }
        _ => {
          if let Some(next_byte) = target_bytes.peek() {
            let mut target: HashMap<usize, Incidence> = HashMap::new();
            let id =  255 + self.global;
            if *current_byte >= 255 || **next_byte >= 255 {
              let current_result = self.check_merges(current_byte);
              let next_result = self.check_merges(next_byte);
              self.merges.insert(id, current_result + next_result.as_str());
            } else {
              match String::from_utf8(vec![*current_byte as u8, **next_byte as u8]) {
                Ok(result) => {
                  self.merges.insert(id, result);
                }
                Err(err) => {
                  eprintln!("{}", err);
                }
              }

            }
            let i = Incidence {
              incidences: 1,
              id: id as u16,
            };
            self.global += 1;
            target.insert(**next_byte as usize, i);
            dict.insert(*current_byte, target);
          }
        }
      }
    }
    dict
  }

  pub fn replace(&mut self, target: &Vec<usize>, matched: &ItemMatch) -> Vec<usize> {
    let mut target_as_bytes = target.iter().peekable();
    let mut replaced: Vec<usize> = Vec::new();
    while let Some(current_byte) = target_as_bytes.next() {
      if let Some(next_concat) = matched.get(current_byte)
      {
        if let Some(next_current_byte) = target_as_bytes.peek() {
          if let Some(next_next_concat) = next_concat.get(next_current_byte) {
            if next_next_concat.incidences > 1 {
              dbg!(&next_next_concat);
              replaced.push(next_next_concat.id as usize);
              target_as_bytes.next();
            } else {
              replaced.push(*current_byte as usize);
            }
          } else {
            replaced.push(*current_byte as usize);
          }
        } else {
          replaced.push(*current_byte as usize);
        }
      }
    }

    replaced
  }


  pub fn encode(&mut self, target: &str, vocab: Vec<String>) -> Vec<String> {
    let vocab_regex: Vec<String> = vocab
      .iter()
      .map(|s| regex::escape(&s))
      .collect();
    let regex_pattern = vocab_regex.join("|");
    let regex = Regex::new(&regex_pattern).unwrap();
    dbg!(&regex);

    // let stack: Vec<&str> = regex.split(target).collect();
    // let mut to_evaluate = target.chars().peekable();
    // let mut stack: Vec<String> = Vec::new();
    // let mut inner_stack: String = "".to_string();


    // while let Some(c) = to_evaluate.next() {
    //   if vocab.contains(&inner_stack.to_string()) {
    //     stack.push(inner_stack.to_string());
    //     inner_stack = "".to_string();
    //   }
    //   inner_stack = inner_stack + &c.to_string();
    // }

    regex
      .split(target)
      .map(|x| x.to_string())
      .collect()
  }
}