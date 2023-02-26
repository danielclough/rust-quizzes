use crate::quizzes::{types::QuizConfig, utils::read_from_input_file};

pub fn config() -> QuizConfig {
    let output = QuizConfig {
        name: "counting_valleys".to_string(),
        level: "level2".to_string(),
    };
    output
}


pub fn quiz(arr: &str) -> Vec<i32> {
    let input = read_input(arr);
    let mut answers: Vec<i32> = vec![];
    for i in input {
        let steps = i.steps;
        let path = i.path;

        answers.push(countingValleys(steps, &path));
    }
    answers
}

#[derive(Clone)]
struct Input {
    steps: i32,
    path: String,
}

fn read_input(arr: &str) -> Vec<Input> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output: Vec<Input> = vec![
        Input {
            steps: 0,
            path: String::from("")
        };
        lines.len() / 2
    ];
    let mut input_num = 0;
    for (i, line) in lines.iter().enumerate() {
        if i % 2 == 0 {
            output[input_num].steps = line.parse::<i32>().expect("expect number");
        } else if i % 2 == 1 {
            output[input_num].path = line.to_string();
            input_num += 1;
        }
    }
    output
}

#[allow(non_snake_case)]
fn countingValleys(steps: i32, path: &str) -> i32 {
    let _ = steps;
    let mut elevation = 0;
    let path_arr: Vec<&str> = path.split("").collect();
    let mut entered_valley = 0;

    for p in path_arr {
        let start_in_valley = if elevation < 0 { true } else { false };
        if p == "U" {
            elevation += 1;
        } else if p == "D" {
            elevation -= 1;
        };

        if start_in_valley == false && elevation < 0 {
            entered_valley += 1;
        };
    }
    println!("{}\n{}", entered_valley, elevation);
    entered_valley
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_work() {
        let answer = vec![1, 2];
        let config = config();
        let input = read_from_input_file(&config.level, &config.name);

        assert_eq!(answer, quiz(&input));
    }
}
