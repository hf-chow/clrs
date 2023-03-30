fn main() {
    test_1();
    test_2();
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
fn linear_search (input: &[String; 32], target: str) {
    for i in input.iter() {
        if input[i] == target {
            return i 
        } else {
        println!("Search target is not in array");
        return None
        }
    }
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
    
    println!("{:?}", output);

    assert!(output == sorted, "Insertion sort test failed!");
    println!("Test 2 Success!");
}
