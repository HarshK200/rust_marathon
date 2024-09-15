use std::thread;
use std::time::Duration;

// NOTE: As long as the main thread is alive even if other threads panic the program doesn't
// panic/crash. But if main thread panics the program dies
pub fn mulitthreading() {
    // WARN: infinite loop for cheching multithreading working in htop or top
    // let heavy_thread = thread::spawn(|| while true {});
    // let heavy_thread1 = thread::spawn(|| while true {});
    // let _ = heavy_thread.join();

    let extra_thread = thread::spawn(|| {
        // let v = vec![0, 1];
        for i in 0..10 {
            // ERR: intentionally making the thread panic to see what happens
            // println!("Hi {} sec. on spawned thread", v[i]);

            println!("Hi {} sec. on spawned thread", i);
            thread::sleep(Duration::from_millis(500)); // NOTE: sleeps for half-a-second
        }
    });

    let v = vec![0, 12, 3];
    let _new_thread = thread::spawn(move || {
        println!("vector v: {:?}", v);
    });

    for i in 0..10 {
        println!("Hi {} sec. on main thread", i);
        thread::sleep(Duration::from_millis(1000)); // NOTE: sleeps for one-second
    }
}
