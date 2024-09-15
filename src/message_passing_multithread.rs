use std::{sync::mpsc, thread, time::Duration};

pub fn message_passing() {
    // NOTE: mspc stands for multiple-producers-single-consumer in our case single consure is the main thread
    let (transmitter, reciver) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(1000));
        let _ = transmitter.send("Hi from spawned thread");
    });

    // WARN: unwrap here assuming that only the Ok variant will return fron the Result pattern match
    // this recv() function here stop the execution till it recives a message (kinda like async await)
    let value = reciver.recv().unwrap();

    println!("{}", value);
    println!("Hi i'm on the main thread");
}

// NOTE: calculates the sum of number from 0 till 10^8
pub fn intensive_sum() {
    let (transmitter, reciver) = mpsc::channel();

    println!("calculating...");
    for i in 0..10 {
        let producer = transmitter.clone();

        thread::spawn(move || {
            let mut ans: u64 = 0;
            for j in (i * 10000000)..((i + 1) * 10000000) - 1 {
                ans = ans + j;
            }
            producer.send(ans).unwrap();
        });
    }
    drop(transmitter); // NOTE: make sure this transmitter is dropped since is no long in use

    let mut ans: u64 = 0;
    for val in reciver {
        println!("value recieved from thread {}", val);
        ans += val;
    }

    println!("calculation result multi-threaded: {}", ans);
}
