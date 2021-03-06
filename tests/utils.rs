use std::sync::Arc;
use std::thread;
use std::time::Duration;

use cadence::prelude::*;
use cadence::StatsdClient;

pub const NUM_THREADS: u64 = 100;
pub const NUM_ITERATIONS: u64 = 1_000;

pub fn run_arc_threaded_test(client: StatsdClient, num_threads: u64, iterations: u64) {
    let shared_client = Arc::new(client);

    let threads: Vec<_> = (0..num_threads)
        .map(|_| {
            let local_client = Arc::clone(&shared_client);

            thread::spawn(move || {
                for i in 0..iterations {
                    local_client.count("some.counter", i as i64).unwrap();
                    local_client.time("some.timer", i).unwrap();
                    local_client.gauge("some.gauge", i).unwrap();
                    local_client.meter("some.meter", i).unwrap();
                    local_client.histogram("some.histogram", i).unwrap();
                    thread::sleep(Duration::from_millis(1));
                }
            })
        })
        .collect();

    for t in threads {
        t.join().unwrap();
    }
}
