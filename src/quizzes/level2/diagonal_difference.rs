use crate::quizzes::{types::QuizConfig, utils::read_from_input_file};

pub fn config() -> QuizConfig {
    let output = QuizConfig {
        name: "diagonal_difference".to_string(),
        level: "level2".to_string(),
    };
    output
}


pub fn quiz(arr: &str) -> i32 {
    let ints = read_input(arr);
    diagonalDifference(&ints)
}

fn read_input(arr: &str) -> Vec<Vec<i32>> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output = vec![];

    for (i, line) in lines.iter().enumerate() {
        let str_arr: Vec<&str> = line.split(' ').collect();
        let mut i32_arr: Vec<i32> = vec![];
        for ele in str_arr {
            i32_arr.push(ele.parse::<i32>().expect("number here"));
        }

        if i != 0 {
            output.push(i32_arr);
        }
    }
    output
}

#[allow(non_snake_case)]
fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    // println!("{:?}", arr);
    let mut primary_diag = 0;
    let mut secondary_diag = 0;
    for (i, a) in arr.iter().enumerate() {
        primary_diag = primary_diag + a[i];
        secondary_diag = secondary_diag + a[a.len() - i - 1];
    }
    let output = secondary_diag - primary_diag;
    output.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_it_work() {
        let answer = 15;
        let config = config();
        let input = read_from_input_file(&config.level, &config.name);

        assert_eq!(answer, quiz(&input));
    }
}
