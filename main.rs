use std::time::Instant;
use std::io::{self, Write};

const INF: i32 = i32::MAX / 2;

fn solve(matrix: &[Vec<i32>]) -> (i32, Vec<usize>, f64) {
    let start_time = Instant::now();
    let n = matrix.len();
    
    let mut dp = vec![vec![INF; n]; 1 << n];
    let mut pre = vec![vec![-1i32; n]; 1 << n];
    
    dp[1][0] = 0;
    
    for mask in 0..(1 << n) {
        for i in 0..n {
            if (mask & (1 << i)) == 0 {
                continue;
            }
            
            let old_mask = mask ^ (1 << i);
            for j in 0..n {
                if (mask & (1 << j)) == 0 {
                    continue;
                }
                
                let cost = dp[old_mask][j] + matrix[j][i];
                if cost < dp[mask][i] {
                    dp[mask][i] = cost;
                    pre[mask][i] = j as i32;
                }
            }
        }
    }
    
    let mut min_cost = INF;
    let mut last = 0;
    for i in 0..n {
        let cost = dp[(1 << n) - 1][i] + matrix[i][0];
        if cost < min_cost {
            min_cost = cost;
            last = i;
        }
    }
    
    let mut mask = (1 << n) - 1;
    let mut path = Vec::new();
    
    while mask != 1 {
        path.push(last);
        let to = pre[mask][last] as usize;
        mask ^= 1 << last;
        last = to;
    }
    path.push(0);
    path.reverse();
    path.push(0);
    
    let elapsed_time = start_time.elapsed().as_secs_f64() * 1000.0;
    
    (min_cost, path, elapsed_time)
}

fn print_matrix(matrix: &[Vec<i32>]) {
    for row in matrix {
        println!("{}", row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "));
    }
}

fn get_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn create_matrix_from_input() -> Vec<Vec<i32>> {
    print!("Enter the number of cities: ");
    io::stdout().flush().unwrap();
    
    let n: usize = loop {
        match get_input().unwrap().parse() {
            Ok(num) if num > 0 => break num,
            _ => {
                print!("Please enter a valid positive number: ");
                io::stdout().flush().unwrap();
            }
        }
    };
    
    println!("Enter the distance matrix ({} x {} matrix):", n, n);
    println!("Enter each row separated by spaces:");
    
    let mut matrix = vec![vec![0; n]; n];
    
    for i in 0..n {
        loop {
            print!("Row {} (cities 0 to {}): ", i + 1, n - 1);
            io::stdout().flush().unwrap();
            
            let input = get_input().unwrap();
            let values: Result<Vec<i32>, _> = input
                .split_whitespace()
                .map(|s| s.parse::<i32>())
                .collect();
            
            match values {
                Ok(row) if row.len() == n => {
                    matrix[i] = row;
                    break;
                }
                Ok(_) => {
                    println!("Error: Please enter exactly {} values", n);
                }
                Err(_) => {
                    println!("Error: Please enter valid integers");
                }
            }
        }
    }
    
    matrix
}

fn run_predefined_tests() {
    let test_cases = vec![
        (
            "Test 1",
            vec![
                vec![0, 29, 20, 21],
                vec![29, 0, 15, 17],
                vec![20, 15, 0, 28],
                vec![21, 17, 28, 0],
            ],
        ),
        (
            "Test 2",
            vec![
                vec![0, 10, 15],
                vec![10, 0, 20],
                vec![15, 20, 0],
            ],
        ),
        (
            "Test 3",
            vec![
                vec![0, 12, 10, 19, 8],
                vec![12, 0, 3, 7, 6],
                vec![10, 3, 0, 2, 20],
                vec![19, 7, 2, 0, 4],
                vec![8, 6, 20, 4, 0],
            ],
        ),
    ];

    for (name, matrix) in test_cases {
        println!("{}", name);
        print_matrix(&matrix);
        let (cost, path, time) = solve(&matrix);
        println!("Minimum Cost: {}", cost);
        println!("Path: {}", path.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" -> "));
        println!("Time taken: {:.4} ms", time);
        println!("{}", "-".repeat(40));
    }
}

fn main() {
    println!("TSP Solver");
    println!("1. Enter custom matrix");
    println!("2. Run predefined test cases");
    print!("Choose option (1 or 2): ");
    io::stdout().flush().unwrap();
    
    let choice = get_input().unwrap();
    
    match choice.as_str() {
        "1" => {
            let matrix = create_matrix_from_input();
            println!("\nDistance Matrix:");
            print_matrix(&matrix);
            
            let (cost, path, time) = solve(&matrix);
            println!("Minimum Cost: {}", cost);
            println!("Path: {}", path.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" -> "));
            println!("Time taken: {:.4} ms", time);
        }
        "2" => {
            run_predefined_tests();
        }
        _ => {
            println!("Invalid option. Running predefined tests...");
            run_predefined_tests();
        }
    }
}