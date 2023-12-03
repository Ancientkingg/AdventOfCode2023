use std::fs;


fn get_input(file_path : &str) -> String {
    fs::read_to_string(file_path).expect("Error while reading file")
}

fn main() {
    let input = get_input("input/input.txt");
    let _example = get_input("input/example.txt");
    let _example2 = get_input("input/example2.txt");


    let answer_part1 = part1(&input);

    println!("The answer to part 1 is: {}", answer_part1);

    let answer_part2 = part2(&input);
    println!("The answer to part 2 is: {}", answer_part2);
}

fn part1(input: &str) -> i32 {
    let mut sum: i32 = 0;

    let lines = input.lines();
    for line in lines {
        let chars = line.chars();

        let mut digits: (Option<i32>, Option<i32>) = (None, None);
        
        for char in chars {
            if let Some(digit) = char.to_digit(10) {
                if digits.0 == None {
                    digits = (Some(digit as i32), Some(digit as i32));
                } else {
                    digits.1 = Some(digit as i32);
                }
            }
        }

        let current_digit = 'digit: {
            if let Some(digit1) = digits.1 {
                break 'digit digits.0.unwrap() * 10 + digit1;
            }
            digits.0.unwrap()
        };

        sum += current_digit as i32;
    }

    sum
}

fn part2(input: &str) -> i32 {
    let mut sum: i32 = 0;

    for line in input.lines() {
        let mut start_index: usize = 0;

        let mut digits: (Option<i32>, Option<i32>) = (None, None);
        
        while start_index < line.len() {

            let char = line.chars().nth(start_index).unwrap();

            if let Some(digit) = char.to_digit(10) {
                if digits.0 == None {
                    digits = (Some(digit as i32), Some(digit as i32));
                } else {
                    digits.1 = Some(digit as i32);
                }
                start_index += 1;
            } else {
                if let Some((digit, _index)) = convert_to_digit(&line[start_index..]) {
                    if digits.0 == None {
                        digits = (Some(digit as i32), Some(digit as i32));
                    } else {
                        digits.1 = Some(digit as i32);
                    }
                    start_index += 1;
                } else {
                    start_index += 1;
                }
            }
        }

        let current_digit = 'digit: {
            if let Some(digit1) = digits.1 {
                break 'digit digits.0.unwrap() * 10 + digit1;
            }
            digits.0.unwrap() 
        };

        sum += current_digit as i32;
    }
    sum
}

// Returns a tuple of (digit, index to move slice by)
fn convert_to_digit(input: &str) -> Option<(i32,usize)> {
    if input.starts_with("one") { return Some((1,3)) }
    if input.starts_with("two") { return Some((2,3)) }
    if input.starts_with("three") { return Some((3,5)) }
    if input.starts_with("four") { return Some((4,4)) }
    if input.starts_with("five") { return Some((5,4)) }
    if input.starts_with("six") { return Some((6,3)) }
    if input.starts_with("seven") { return Some((7,5)) }
    if input.starts_with("eight") { return Some((8,5)) }
    if input.starts_with("nine") { return Some((9,4)) }

    None
}