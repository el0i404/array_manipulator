fn main() {
    let numbers = vec![1, 5, 3, 9, 2, 8, 4, 7, 6];
    let array = [10, 20, 30, 40, 50];

    println!("=== Array Manipulator ===");

    // Test with vector
    println!("Vector: {:?}", numbers);
    test_operations(&numbers);

    // Test with array
    println!("\nArray: {:?}", array);
    test_operations(&array);

    // Original data should still be available
    println!("\nOriginal vector: {:?}", numbers);
    println!("Original array: {:?}", array);
}

fn test_operations(data: &[i32]) {
    println!("Length: {}", data.len());

    if let Some(max) = find_max(data) {
        println!("Maximum: {}", max);
    }

    if let Some(min) = find_min(data) {
        println!("Minimum: {}", min);
    }

    println!("Sum: {}", calculate_sum(data));
    println!("Average: {:.2}", calculate_average(data));

    let target = 5;
    if let Some(index) = find_element(data, target) {
        println!("Found {} at index {}", target, index);
    } else {
        println!("{} not found", target);
    }

    println!("Even numbers: {:?}", filter_even(data));
    println!("Numbers > 5: {:?}", filter_greater_than(data, 5));
}

// TODO: Implement these functions
// They should all work with slices and not take ownership

fn find_max(data: &[i32]) -> Option<i32> {
    // Find maximum value in slice
    data.iter().cloned().max()
}

fn find_min(data: &[i32]) -> Option<i32> {
    // Find minimum value in slice
    data.iter().cloned().min()
}

fn calculate_sum(data: &[i32]) -> i32 {
    // Calculate sum of all elements
    let mut sum = 0;
    data.iter().for_each(|&x| sum += x);
    sum
}

fn calculate_average(data: &[i32]) -> f64 {
    // Calculate average (handle empty slice)
    0.0
}

fn find_element(data: &[i32], target: i32) -> Option<usize> {
    // Find index of target element
    data.iter().position(|&x| x == target)
    // data.iter().find(|&x| *x == target)
}

fn filter_even(data: &[i32]) -> Vec<i32> {
    // Return vector of even numbers
    // Note: This creates new data, but original slice is unchanged
    let mut even_numbers:Vec<i32> = vec![];

     data.iter().cloned().for_each(|x| {
        if x % 2 != 0 {
            even_numbers.push(x)
        }
    });

    even_numbers
}

fn filter_greater_than(data: &[i32], threshold: i32) -> Vec<i32> {
    // Return vector of numbers greater than threshold
    let mut greaters= vec![];

    data.iter().cloned().for_each(|x| {
        if x > threshold {
            greaters.push(x)
        }
    });

    greaters
}

// Bonus functions:
fn find_second_largest(data: &[i32]) -> Option<i32> {
    // Find second largest unique value
    None
}

fn calculate_median(data: &[i32]) -> Option<f64> {
    // Calculate median (you'll need to sort a copy)
    None
}

fn find_subslice(data: &[i32], pattern: &[i32]) -> Option<usize> {
    // Find starting index of pattern in data
    None
}
