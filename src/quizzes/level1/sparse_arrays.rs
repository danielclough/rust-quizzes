use crate::quizzes::{types::{QuizConfig, AnswerType}, utils::read_from_input_file};

pub fn config() -> QuizConfig {
    let output = QuizConfig {
        name: "sparse_arrays".to_string(),
        desc: "desc".to_string(),
        example: "example".to_string(),
        constraints: vec!["".to_string()],
        level: "level1".to_string(),
        answer: AnswerType::VecVecI32 { answer: vec![vec![2, 1, 0], vec![1, 0, 1], vec![1, 3, 4, 3, 2]] },
    };
    output
}


pub fn quiz() -> Vec<Vec<i32>> {
    let input_arr = read_input();
    let mut output: Vec<Vec<i32>> = vec![];
    for input in input_arr {
        output.push(matchingStrings(&input.strings, &input.queries));
    }
    output
}

struct Input {
    strings: Vec<String>,
    queries: Vec<String>,
}

fn read_input() -> Vec<Input> {
    let config = config();
    let in_from_file = read_from_input_file(&config.level, &config.name).to_owned();
    let lines:  Vec<&str> = in_from_file.split("\n").collect();
    //  var to indicate beginning of series
    let mut left_in_cycle = -1;
    // prepare n and q vars
    // n == ar length (reduce each round)
    let mut n = 0;
    // q == querry length  (reduce each round)
    let mut q = 0;
    // vecs to hold strings and queries
    let mut strings: Vec<String> = vec![];
    let mut queries: Vec<String> = vec![];
    // strut for input
    let mut input_arr: Vec<Input> = vec![];
    for line in lines {
        if left_in_cycle == -1 {
            n = line.parse::<i32>().expect("i32 here");
            left_in_cycle = n;
        } else if n > 0 {
            strings.push(line.to_string());
            n -= 1;
            if n == 0 {
                left_in_cycle = -2;
            }
        } else if left_in_cycle == -2 {
            q = line.parse::<i32>().expect("i32 here");
            left_in_cycle = q;
        } else if q > 0 {
            queries.push(line.to_string());
            q -= 1;
            if q == 0 {
                left_in_cycle = -1;
                // call function at end of cycle
                input_arr.push(Input {
                    strings: strings.to_owned(),
                    queries: queries.to_owned(),
                });
                strings = vec![];
                queries = vec![];
            }
        }
    }
    input_arr
}

#[allow(non_snake_case)]
fn matchingStrings(strings: &[String], queries: &[String]) -> Vec<i32> {
    let length = queries.len();
    let mut return_vec: Vec<i32> = vec![0; length];
    for (i, q) in queries.iter().enumerate() {
        for s in strings {
            if q == s {
                return_vec[i] += 1;
                // println!("{:?}{:?} {} {}",q, s, i, j);
            }
        }
    }
    println!("{:?}", return_vec);
    return_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_work() {
        assert_eq!(config().answer, AnswerType::VecVecI32 { answer: quiz() } );
    }
}
