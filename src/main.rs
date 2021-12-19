use std::env;
use combinations::Combinations;
use itertools::Itertools;


/// Sums an array and compares it to a set value.
///
/// This function takes in an array of integers and calculates the sum. 
/// It then compares this value to the requested total i.e. the total 
/// that you wish to find. 
/// 
/// * `comb_array` - A vector of integers 
/// * `requested_total` - The total that you wish to find 
fn compare(comb_array: Vec<i32>, requested_total: i32) -> Option<Vec<i32>> {
	let sum_of_array: i32 = comb_array.iter().sum();
	if sum_of_array == requested_total {
		return Some(comb_array);
	}
	None
}

/// Creates an array of integers from another int by spliting. 
///
/// This function takes in an integer and splits it into characters 
/// and returns an array of that integers with each numbers an element 
/// in that array.  
///
/// * `num` - The integer to be changed
fn num_digits(num: i32) -> Vec<i32> {
    let mul = if num < 0 { -1 } else { 1 };
    num.to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| (x as i32) * mul)
        .collect()
}

/// Checks if an array is contained within another array.
///
/// Returns true or false depending
///
/// * `array_to_check` - Array to be checked
/// * `filter` - Array that we are going to check is array_to_check
fn add_filter(array_to_check: &Vec<i32>, filter: &Vec<i32>) -> bool{
    let mut filter_checked: Vec<i32> = Vec::new();
    filter_checked.append(&mut array_to_check.clone());
    filter_checked.append(&mut filter.clone());

    let checked: Vec<_> = filter_checked.into_iter().unique().collect();

    if checked.len() == array_to_check.len(){
        return true;
    } false
}


fn main() {
    let args: Vec<String> = env::args().collect();

    let num: i32 = args[1].parse().unwrap();
    let length: usize = args[2].parse().unwrap();
    let filter: i32 = args[3].parse().unwrap();

    println!("\nTotal Number: {:?}", num);
    println!("Length: {:?}\n", length);

    let available_nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    let computed: Vec<_> = Combinations::new(available_nums, length).collect();

	// Find all the combos that get to the correct sum
    let mut combo_results: Vec<Option<Vec<i32>>> = Vec::new();

    if filter > 0 {
        let filters = num_digits(filter);
        for comb_array in computed {
            if add_filter(&comb_array, &filters) {
                let result = compare(comb_array, num);
                if result != None {
                    combo_results.push(result);
                }
            }
        }
    } else {
        for comb_array in computed {
            let result = compare(comb_array, num);
            if result != None {
                combo_results.push(result);
            }
        }
    }

	// Print the results
	for combo in combo_results {
		println!("{:?}", combo.unwrap());
	}
}
