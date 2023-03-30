fn main() {
    test_1();
    test_2();
    test_3();
}

fn insertion_sort<T: Ord> (input: &mut [T]) -> &mut [T]{
    for i in 1..input.len() {
        let mut j = i;
        while j > 0 && input[j - 1] > input[j] {
            input.swap(j, j - 1);
            j -= 1;
        }
    }
    return input
}

fn rev_insertion_sort<T: Ord> (input: &mut [T]) -> &mut[T] {
    for i in 1..input.len() {
        let mut j = i;
        while j > 0 && input[j] > input[j-1] {
            input.swap(j, j-1);
            j -= 1;
        }
    }
    return input
}

// Incomplete
fn linear_search<T: PartialEq> (input: &[T], target: &T) -> i32{
    let mut ret_key = -1;
    for (key, val) in input.iter().enumerate() {
        if val == target {
            ret_key = key as i32;
            return ret_key 
        }
    }
    return ret_key
}

// Test for insertion sort
fn test_1() {
    let mut test = [3, 6, 1, 2, 4, 4, 0];
    let sorted = [0, 1, 2, 3, 4, 4, 6];
    let output = insertion_sort(&mut test);
    
    assert!(output == sorted, "Insertion sort test failed!");
    println!("Test 1 Success!");
}

// Test for reversed insertion sort
fn test_2() {
    let mut test = [3, 6, 1, 2, 4, 4, 0];
    let sorted = [6, 4, 4, 3, 2, 1, 0];
    let output = rev_insertion_sort(&mut test);

    assert!(output == sorted, "Insertion sort test failed!");
    println!("Test 2 Success!");
}

// Test for linear search
fn test_3() {
    let mut test = [3, 6, 1, 2, 4, 4, 0];
    let output = linear_search(&mut test, &3);
    let ans = 0;
    
    assert!(output == ans, "Linear search test failed!");
    println!("Test 3 Success!");
}

