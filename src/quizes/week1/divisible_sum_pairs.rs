pub fn test(arr: &str) {
    let lines: Vec<&str> = arr.split("\n").collect();
    // prepare n and k vars
    // n == ar length
    let mut n = 0;
    // k == divisor
    let mut k = 0;
    
    for (i, line) in lines.iter().enumerate() {
        if i%2 == 0 {
            let str_arr: Vec<&str> = line.split(' ').collect();
            n = str_arr[0].parse::<i32>().expect("number here");
            k = str_arr[1].parse::<i32>().expect("number here");
        }
        // prepare ar and call function
        if i%2 == 1 {
            let mut ar:Vec<i32> = vec![];
            let arr: Vec<&str> = line.split(' ').collect();

            for a in arr {
                ar.push(a.parse::<i32>().expect("number here"))
            }
            divisibleSumPairs(n, k, &ar);
        }
    }
}

#[allow(non_snake_case)]
fn divisibleSumPairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    // determine the number of (i,j) pairs where i<j and ar[i] + ar[j] is divisible by k. 
    // pairs == number of pairs
    let _junk = n;
    let mut pairs = 0;
    for (i, a) in ar.iter().enumerate() {
        for (j, b ) in ar.iter().enumerate() {
            if j>i && (a + b) % k == 0 {
                pairs +=1
            }
        }        
    }
    println!("{}", pairs);
    pairs
}