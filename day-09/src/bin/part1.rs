fn resolve(input: &str) -> i32{
    let history_triangles: Vec<Vec<Vec<i32>>> = input.lines()
        .map(|history| {
            let mut triangle: Vec<Vec<i32>> = vec![];
            triangle.push(history.split_whitespace()
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<i32>>());

            let mut last_row: &Vec<i32> = &triangle[triangle.len() - 1];
            while triangle[triangle.len() - 1].iter().any(|number| *number != triangle[triangle.len() - 1][0]) {
                let mut new_row: Vec<i32> = vec![];
                for i in 0..last_row.len() - 1 {
                    new_row.push(last_row[i + 1] - last_row[i]);
                }
                triangle.push(new_row);
                last_row = &triangle[triangle.len() - 1];
            }
            
            return triangle;
        })
        .collect();

        let mut result: i32 = 0;
        for triangle in &history_triangles {
            let mut predict = 0;
            for i in (0..triangle.len()).rev() {   
                predict = predict + triangle[i][triangle[i].len() - 1];
            }
            result += predict;
        }
        return result;
}

fn main() {
    let input_file: &str = include_str!("../input.txt");
    let result = resolve(&input_file);
    println!("Value: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input: &str = 
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";
        assert_eq!(resolve(input), 114);
    }
}