use crate::quizzes::{types::{QuizConfig, AnswerType}, utils::read_from_input_file};

pub fn config() -> QuizConfig {
    let output = QuizConfig {
        name: "breaking_the_records".to_string(),
        desc: "desc".to_string(),
        example: "example".to_string(),
        constraints: vec!["".to_string()],
        level: "level1".to_string(),
        answer: AnswerType::VecVecI32 { answer: vec![vec![2, 4], vec![4, 0]] },
    };
    output
}

pub fn quiz() -> Vec<Vec<i32>> {
    let scores = read_input();
    let mut answers: Vec<Vec<i32>> = vec![];
    for s in scores {
        answers.push(breakingRecords(&s));
    }
    answers
}

fn read_input() -> Vec<Vec<i32>> {
    let config = config();
    let in_from_file = read_from_input_file(&config.level, &config.name).to_owned();
    let lines:  Vec<&str> = in_from_file.split("\n").collect();
    let mut output = vec![];

    for (i, line) in lines.iter().enumerate() {
        let str_arr: Vec<&str> = line.split(' ').collect();
        let mut i32_arr: Vec<i32> = vec![];
        for ele in str_arr {
            i32_arr.push(ele.parse::<i32>().expect("number here"));
        }

        if i % 2 == 1 {
            output.push(i32_arr);
        }
    }
    output
}

#[allow(non_snake_case)]
fn breakingRecords(scores: &[i32]) -> Vec<i32> {
    // let n = &scores.len();
    let mut current_min = 0;
    let mut min = 0;
    let mut current_max = 0;
    let mut max = 0;
    for (i, score) in scores.iter().enumerate() {
        if i == 0 {
            current_min = score.to_owned();
            current_max = score.to_owned();
        } else if score < &current_min {
            min += 1;
            current_min = score.to_owned();
            println!("min: {} {} {}", score, min, current_min);
        } else if score > &current_max {
            max += 1;
            current_max = score.to_owned();
            println!("max: {} {} {}", score, max, current_max);
        }
    }
    let mut return_vec: Vec<i32> = vec![];
    return_vec.push(max);
    return_vec.push(min);

    println!("{} {}", max, min);
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
