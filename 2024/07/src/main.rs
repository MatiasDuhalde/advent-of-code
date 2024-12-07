use std::env;
use std::fs;

fn read_input(filename: &str) -> Vec<(u64, Vec<u64>)> {
    let contents = fs::read_to_string(filename).unwrap();
    contents
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .map(|(a, b)| {
            (
                a.parse().unwrap(),
                b.split(" ").map(|x| x.parse().unwrap()).collect(),
            )
        })
        .collect()
}

fn get_calibration_result(operations: &Vec<(u64, Vec<u64>)>) -> u64 {
    let mut calibration_result = 0;
    for (expected_result, operands) in operations {
        if is_valid_operation(*expected_result, operands[0], &operands[1..].to_vec()) {
            calibration_result += expected_result;
        }
    }
    calibration_result
}

fn is_valid_operation(expected_result: u64, current_result: u64, operands: &Vec<u64>) -> bool {
    if operands.len() == 0 {
        return current_result == expected_result;
    }
    if is_valid_operation(
        expected_result,
        current_result + operands[0],
        &operands[1..].to_vec(),
    ) {
        true
    } else if is_valid_operation(
        expected_result,
        current_result * operands[0],
        &operands[1..].to_vec(),
    ) {
        true
    } else {
        false
    }
}

fn get_calibration_result_with_concatenation(operations: &Vec<(u64, Vec<u64>)>) -> u64 {
    let mut calibration_result = 0;
    for (expected_result, operands) in operations {
        if is_valid_operation_with_concatenation(
            *expected_result,
            operands[0],
            &operands[1..].to_vec(),
        ) {
            calibration_result += expected_result;
        }
    }
    calibration_result
}

fn is_valid_operation_with_concatenation(
    expected_result: u64,
    current_result: u64,
    operands: &Vec<u64>,
) -> bool {
    if operands.len() == 0 {
        return current_result == expected_result;
    }

    let power = operands[0].ilog10() + 1;

    if is_valid_operation_with_concatenation(
        expected_result,
        current_result * 10_u64.pow(power) + operands[0],
        &operands[1..].to_vec(),
    ) {
        true
    } else if is_valid_operation_with_concatenation(
        expected_result,
        current_result + operands[0],
        &operands[1..].to_vec(),
    ) {
        true
    } else if is_valid_operation_with_concatenation(
        expected_result,
        current_result * operands[0],
        &operands[1..].to_vec(),
    ) {
        true
    } else {
        false
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let operations = read_input(filename);
    // part one
    let calibration_result = get_calibration_result(&operations);
    println!("Calibration result: {}", calibration_result);
    // part two
    let calibration_result_with_concatenation =
        get_calibration_result_with_concatenation(&operations);
    println!(
        "Calibration result: {}",
        calibration_result_with_concatenation
    );
}
