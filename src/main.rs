use std::io;

fn main() {
    let test_vec = vec![1, 2, 4, 5, 7, 32, 23];
    task_1();
    task_2();
    task_3(test_vec.clone());
    task_4(test_vec.clone());
    task_5(test_vec.clone());
    task_6(3, 4);
    task_7();
    task_9();
    task_10(3, -10, 10);
    task_11(5, 5, -10, 10);
    task_12(3, -10, 10);
    task_13(test_vec);
}

fn task_1() {
    let mut tup = [0; 5];
    for i in 0..5 {
        tup[i] = rand::random_range(1..=10);
    }
    for i in 0..tup.len() {
        println!("{}", tup[i]);
    }
}

fn task_2() {
    let tuple: [i32; 10] = [54, 7, -41, 2, 4, 2, 89, 33, -5, 12];
    let selection_sort_result = {
        let mut tuple_tmp = tuple;
        let n = tuple_tmp.len();

        for i in 0..n {
            let mut min_index = i;

            for j in (i + 1)..n {
                if tuple_tmp[j] < tuple_tmp[min_index] {
                    min_index = j;
                }
            }
            tuple_tmp.swap(i, min_index);
        }
        tuple_tmp
    };

    let bubble_sort_result = {
        let mut tuple_tmp = tuple;
        let n = tuple_tmp.len();

        for i in 0..n {
            let mut swapped = false;

            for j in 0..(n - 1 - i) {
                if tuple_tmp[j] > tuple_tmp[j + 1] {
                    tuple_tmp.swap(j, j + 1);
                    swapped = true;
                }
            }

            if !swapped {
                break;
            }
        }
        tuple_tmp
    };
    let mut quick_sort_result = tuple;
    quick_sort(&mut quick_sort_result);
    println!(
        "{:?}\n{:?}\n{:?}",
        selection_sort_result, bubble_sort_result, quick_sort_result
    )
}

fn task_3(tup: Vec<i32>) {
    let for_sum = {
        let mut sum = 0;
        for i in 0..tup.len() {
            sum += tup[i]
        }
        sum
    };
    let foreach_sum = {
        let mut sum = 0;
        for i in &tup {
            sum += i
        }
        sum
    };
    println!("{:?}\n{:?}", for_sum, foreach_sum)
}

fn task_4(mut tup: Vec<i32>) -> Vec<i32> {
    let n = tup.len();

    for i in 0..n / 2 {
        tup.swap(i, n - i - 1);
    }
    println!("{:?}", tup);
    tup
}

fn task_5(tup: Vec<i32>) -> i32 {
    let mut counter = 0;
    for i in &tup {
        if *i > 0 {
            counter += 1
        };
    }
    println!("{}", counter);
    counter
}

fn task_6(rows: usize, cols: usize) {
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; cols]; rows];
    for i in 0..rows {
        for n in 0..cols {
            matrix[i][n] = parse_number(&format!("введите [{i}][{n}]"));
        }
    }

    for i in 0..rows {
        for n in 0..cols {
            print!("[{:02}]", matrix[i][n])
        }
        println!();
    }
}

fn task_7() {
    let rows = 3;
    let cols = 4;
    let matrix = random_matrix(rows, cols, -50, 50);

    for i in 0..rows {
        let mut sum = 0;
        for n in 0..cols {
            print!("[{:?}]", matrix[i][n]);
            sum += matrix[i][n];
        }
        println!(" | sum {}", sum);
    }
}

fn task_9() -> i64 {
    let rows = 3;
    let cols = 4;
    let matrix = random_matrix(rows, cols, -10, 10);
    let mut empty = true;
    let mut result = 1;
    for i in 0..rows {
        for n in 0..cols {
            if matrix[i][n] % 2 == 0 {
                empty = false;
                result *= matrix[i][n];
            }
        }
    }
    if !empty {
        println!("{}", result);
        result
    } else {
        println!("no even numbers");
        -1
    }
}

fn task_10(size: usize, min_value: i64, max_value: i64) -> i64 {
    let matrix = random_matrix(size, size, min_value, max_value);
    let mut sum = 0;
    for i in 0..size {
        sum += matrix[i][i]
    }
    println!("{}", sum);
    sum
}

fn task_11(rows: usize, cols: usize, min_value: i64, max_value: i64) -> i64 {
    let matrix = random_matrix(rows, cols, min_value, max_value);
    let mut sum = 0;
    for i in 0..rows {
        for n in 0..cols {
            if i.abs_diff(n) == 2 {
                sum += matrix[i][n]
            }
        }
    }
    println!("{}", sum);
    sum
}

fn task_12(size: usize, min_value: i64, max_value: i64) -> Vec<Vec<i64>> {
    let matrix = random_matrix(size, size, min_value, max_value);
    let mut products: Vec<i64> = vec![1; size];
    let mut zeros: Vec<usize> = vec![0; size];

    for i in 0..size {
        for n in 0..size {
            if matrix[i][n] == 0 {
                zeros[i] += 1;
            } else {
                products[i] *= matrix[i][n];
            }
        }
    }

    let mut result = vec![vec![0; size]; size];

    for i in 0..size {
        for n in 0..size {
            result[i][n] = match zeros[i] {
                0 => products[i] / matrix[i][n],
                1 if matrix[i][n] == 0 => products[i],
                _ => 0,
            };
        }
    }

    println!("{:?}", result);
    result
}

fn task_13(tup: Vec<i32>) -> Vec<i32> {
    let length = tup.len();
    if length <= 1 {
        return tup;
    }
    let mut result = vec![0; length];

    result[0] = (tup[0] + tup[1]).div_euclid(2);
    for i in 1..length - 1 {
        result[i] = (tup[i - 1] + tup[i + 1] + tup[i]).div_euclid(3);
    }
    result[length - 1] = (tup[length - 1] + tup[length - 2]).div_euclid(2);
    println!("{:?}", result);
    result
}

// Модули
fn random_matrix(rows: usize, cols: usize, min_value: i64, max_value: i64) -> Vec<Vec<i64>> {
    let mut matrix: Vec<Vec<i64>> = vec![vec![0; cols]; rows];
    for i in 0..rows {
        for n in 0..cols {
            matrix[i][n] = rand::random_range(min_value..=max_value);
        }
    }
    matrix
}

fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }

    let pivot = arr[arr.len() / 2];

    let mut left = 0;
    let mut right = arr.len() - 1;

    loop {
        while arr[left] < pivot {
            left += 1;
        }

        while arr[right] > pivot {
            right -= 1;
        }

        if left >= right {
            break;
        }

        arr.swap(left, right);

        left += 1;
        right -= 1;
    }

    let (left_part, right_part) = arr.split_at_mut(left);

    quick_sort(left_part);
    quick_sort(right_part);
}

fn parse_number(msg: &str) -> i32 {
    loop {
        println!("{}", msg);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read string");
        match input.trim().parse::<i32>() {
            Ok(n) => {
                break n;
            }
            Err(_) => {
                println!("Not a number")
            }
        }
    }
}
