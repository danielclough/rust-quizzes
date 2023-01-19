pub fn test(arr: &str) -> Vec<i32> {    
    let inputs  = read_input(arr);
    let mut answers: Vec<i32> = vec![];
    for input in inputs {
        answers.push(sockMerchant(input.n, &input.ar));
    };
    answers
}

#[derive(Clone)]
struct Input {
    n: i32,
    ar: Vec<i32>,
}

fn read_input(arr: &str) -> Vec<Input> {
    let lines: Vec<&str> = arr.split("\n").collect();
    let mut output = vec![Input {n:0,ar: vec![]}; lines.len()/2];
    let mut output_n = 0;

    for (i, line) in lines.iter().enumerate() {
        if i%2 == 0 {
            output[output_n].n = line.parse::<i32>().expect("number");
        }
        else if i%2 == 1 {
            output[output_n].ar = line.split(" ").map(|x| x.parse::<i32>().expect("number")).collect();
            output_n += 1;
        }
    }
    output
}

#[allow(non_snake_case)]
fn sockMerchant(n: i32, ar: &[i32]) -> i32 {
    println!("{} {:?}", n, ar);
    4
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn does_it_work(){
        let answer = vec![ 3, 4 ];

        // load file or panic
        let path = String::from("input/week3/sales_by_match.txt");
        let input = fs::read_to_string(&path).expect("Should have been able to read the file");
        
        assert_eq!(answer, test(&input));
        
    }
}