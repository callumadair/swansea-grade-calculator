use std::io;

fn main() {
    println!("Choose either a third (1) or fourth (2) year degree to calculate the overall grade for: ");

    let mut option_input = String::new();
    io::stdin()
        .read_line(&mut option_input)
        .expect("Failed to read line!");
    let degree_choice: u8 = option_input.
        trim().
        parse().
        expect("Input is not a valid integer!");

    match degree_choice {
        1 => calculate_3yr(),

        2 => calculate_4yr(),

        _ => {
            println!("Invalid input, program exiting.");
            std::process::exit(-1);
        }
    }
}

fn calculate_4yr() {
    let mut array_tuple = fill_4yr();
    let overall_grade = weight_and_average_4yr(&mut array_tuple);

    println!("Overall grade for four-year advanced degree is: {}.", overall_grade);
}

fn fill_4yr() -> ([u16; 8], [u16; 8], [u16; 8]) {
    println!("Enter all grades for a given year on a single line delimited by spaces.");

    let mut fourth_yr_arr: [u16; 8] = [0; 8];
    println!("Please enter your module grades for fourth year.");
    fill_array(&mut fourth_yr_arr);

    let mut third_yr_arr: [u16; 8] = [0; 8];
    println!("Please enter your module grades for third year.");
    fill_array(&mut third_yr_arr);

    let mut second_yr_arr: [u16; 8] = [0; 8];
    println!("Please enter your module grades for second year.");
    fill_array(&mut second_yr_arr);

    return (fourth_yr_arr, third_yr_arr, second_yr_arr);
}

fn calculate_3yr() {
    println!("Enter all grades for a given year on a single line delimited by spaces.");
    let mut third_yr_arr: [u16; 8] = [0; 8];
    println!("Please enter your module grades for third year.");
    fill_array(&mut third_yr_arr);

    let mut second_yr_arr: [u16; 8] = [0; 8];
    println!("Please enter your module grades for second year on.");
    fill_array(&mut second_yr_arr);

    let mut array_tuple = (third_yr_arr, second_yr_arr);
    let overall_grade = weight_and_average_3yr(&mut array_tuple);

    println!("Overall grade for three-year standard degree is: {}.", overall_grade);
}

fn fill_array(arr: &mut [u16; 8]) -> &mut [u16; 8] {
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("failed to read grade");
    let split = input_line.split_whitespace();

    let str_vec = split.collect::<Vec<&str>>();

    for index in 0..8 {
        arr[index] = str_vec[index].parse().unwrap();
    }
    arr.sort();
    arr.reverse();

    return arr;
}

fn weight_and_average_4yr(grade_array_tuple: &mut ([u16; 8], [u16; 8], [u16; 8])) -> f32 {
    let mut sum: f32 = 0.0;
    let total_weight: f32 = 60.0;


    for index in 0..8 {
        if index < 6 {
            grade_array_tuple.0[index] *= 4;
            if index < 2 {
                grade_array_tuple.2[index] *= 2;
            }
            if index < 4 {
                grade_array_tuple.1[index] *= 3;
            } else {
                grade_array_tuple.1[index] *= 2;
            }
        } else {
            grade_array_tuple.0[index] *= 3;
            grade_array_tuple.1[index] *= 2;
        }
        sum += f32::from(grade_array_tuple.0[index]
            + grade_array_tuple.1[index]
            + grade_array_tuple.2[index]);
    }

    return sum / total_weight;
}

fn weight_and_average_3yr(grade_array_tuple: &mut ([u16; 8], [u16; 8])) -> f32 {
    let mut sum: f32 = 0.0;
    let total_weight: f32 = 33.0;

    for index in 0..8 {
        if index < 6 {
            grade_array_tuple.0[index] *= 3;
            if index < 3 {
                grade_array_tuple.1[index] *= 2;
            }
        } else if index < 8 {
            grade_array_tuple.0[index] *= 2;
        }

        sum += f32::from(grade_array_tuple.0[index]
            + grade_array_tuple.1[index]);
    }

    return sum / total_weight;
}
