use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("failed");
    part1(&contents);
    part2(&contents);
}
fn part1(contents: &str) {
    let mut mat = Vec::new();
    for line in contents.lines() {
        let row: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        mat.push(row);
    }

    let m = mat.len();
    let n = mat[0].len();
    let mut res = vec![vec![0; n]; m];

    for i in 0..m {
        let mut max = mat[i][0];
        res[i][0] = 1;
        for j in 0..n {
            if mat[i][j] > max {
                res[i][j] = 1;
                max = mat[i][j];
            }
        }
        let mut max = mat[i][n - 1];
        res[i][n - 1] = 1;
        for j in 0..n {
            if mat[i][n - j - 1] > max {
                res[i][n - j - 1] = 1;
                max = mat[i][n - j - 1];
            }
        }
    }

    for j in 0..n {
        let mut max = mat[0][j];
        res[0][j] = 1;
        for i in 0..m {
            if mat[i][j] > max {
                res[i][j] = 1;
                max = mat[i][j];
            }
        }
        let mut max = mat[m - 1][j];
        res[m - 1][j] = 1;
        for i in 0..m {
            if mat[m - i - 1][j] > max {
                res[m - i - 1][j] = 1;
                max = mat[m - i - 1][j];
            }
        }
    }

    let mut total = 0;
    for i in 0..m {
        for j in 0..n {
            total += res[i][j];
        }
    }
    println!("{:?}", total);
}
fn part2(contents: &str) {
    let mut mat = Vec::new();
    for line in contents.lines() {
        let row: Vec<u32> = line.chars().map(|x| x.to_digit(10).unwrap()).collect();
        mat.push(row);
    }

    let m = mat.len();
    let n = mat[0].len();
    let mut max = 0;

    for i in 1..m - 1 {
        for j in 1..n - 1 {
            let target = mat[i][j];

            let mut u_i = i - 1;
            let mut u_d = 1;
            while u_i > 0 && mat[u_i][j] < target {
                u_i -= 1;
                u_d += 1;
            }

            let mut d_i = i + 1;
            let mut d_d = 1;
            while d_i < m - 1 && mat[d_i][j] < target {
                d_i += 1;
                d_d += 1;
            }

            let mut l_j = j - 1;
            let mut l_d = 1;
            while l_j > 0 && mat[i][l_j] < target {
                l_j -= 1;
                l_d += 1;
            }

            let mut r_j = j + 1;
            let mut r_d = 1;
            while r_j < n - 1 && mat[i][r_j] < target {
                r_j += 1;
                r_d += 1;
            }

            let res = u_d * l_d * d_d * r_d;
            if res > max {
                max = res;
            }
        }
    }
    println!("{:?}", max);
}
