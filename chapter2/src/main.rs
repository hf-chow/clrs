fn main() {
    test_1();
    
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

// Test for insertion_sort
fn test_1() {
    let mut test = [3, 6, 1, 2, 4, 4, 0];
    let sorted = [0, 1, 2, 3, 4, 4, 6];
    let output = insertion_sort(&mut test);
    
    println!("{:?}", output);

    assert!(output == sorted, "Insertion sort test failed!");
}
