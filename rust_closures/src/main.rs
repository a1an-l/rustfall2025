fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        tracker += 1;
        println!("Tracker: {}", tracker);
    };

    update();
    update();
}

fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    vec.into_iter().map(f).collect()
}






fn main() {
    let operation = |a: i32, b: i32| {
        a * b
    };

    println!("Result: {}", operation(10, 5));

    track_changes();

        let numbers = vec![1, 2, 3];

    let doubled = process_vector(numbers.clone(), |x| {
        x * 2
    });

    let replaced = process_vector(numbers, |x| {
        if x > 2 { 0 } else { x }
    });

    println!("Doubled: {:?}", doubled);   // [2, 4, 6]
    println!("Replaced: {:?}", replaced); // [1, 2, 0]

}


