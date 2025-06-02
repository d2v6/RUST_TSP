use std::time::Instant;

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

fn main() {
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
        (
            "Test 4",
            vec![
                vec![0, 100, 200, 150],
                vec![100, 0, 50, 75],
                vec![200, 50, 0, 25],
                vec![150, 75, 25, 0],
            ],
        ),
        (
            "Test 5",
            vec![
                vec![0, 20, 42, 25, 30, 34],
                vec![20, 0, 30, 34, 20, 25],
                vec![42, 30, 0, 10, 25, 15],
                vec![25, 34, 10, 0, 15, 30],
                vec![30, 20, 25, 15, 0, 20],
                vec![34, 25, 15, 30, 20, 0],
            ],
        ),
        (
            "Test 6",
            vec![
                vec![0, 10, 8, 9],
                vec![5, 0, 7, 6],
                vec![12, 11, 0, 4],
                vec![15, 13, 14, 0],
            ],
        ),
        (
            "Test 7 ",
            vec![
                vec![0, 1, 10, 10, 10],
                vec![1, 0, 1, 10, 10],
                vec![10, 1, 0, 1, 10],
                vec![10, 10, 1, 0, 1],
                vec![10, 10, 10, 1, 0],
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