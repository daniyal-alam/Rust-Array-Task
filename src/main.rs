use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    const ARRAY_SIZE: usize = 100;
    const COUNT: usize = 10;

    let result = Arc::new(Mutex::new([0; ARRAY_SIZE]));

    let mut handles = vec![];

    for thread_id in 0..COUNT {
        let result = Arc::clone(&result);

        let handle = thread::spawn(move || {
            let start = thread_id * (ARRAY_SIZE / COUNT);
            let end = (thread_id + 1) * (ARRAY_SIZE / COUNT);

            for i in start..end {
                let square = (i as i32 + 1) * (i as i32 + 1);
                result.lock().unwrap()[i] = square;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_result = result.lock().unwrap();

    println!("Squared array: {:?}", final_result);
}
