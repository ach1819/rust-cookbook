extern crate crossbeam;
extern crate crossbeam_channel;

use std::time::Duration;
use std::{thread, time};

use crossbeam_channel::bounded;
use crossbeam_channel::unbounded;

fn main() {
    spawn_short_lived_thread();
    parallel_pipeline();
    pass_data_between_two_threads();
}

fn spawn_short_lived_thread() {
    let arr = &[1, 25, -4, 10];
    let max = find_max(arr);
    assert_eq!(max, Some(25));
    println!("The max number in {:?} is {}", arr, max.unwrap());
}

fn find_max(arr: &[i32]) -> Option<i32> {
    const THRESHOLD: usize = 2;

    if arr.len() <= THRESHOLD {
        return arr.iter().cloned().max();
    }

    let mid = arr.len() / 2;
    let (left, right) = arr.split_at(mid);

    crossbeam::scope(|s| {
        let thread_l = s.spawn(|_| find_max(left));
        let thread_r = s.spawn(|_| find_max(right));

        let max_l = thread_l.join().unwrap()?;
        let max_r = thread_r.join().unwrap()?;

        Some(max_l.max(max_r))
    })
    .unwrap()
}

fn parallel_pipeline() {
    let (snd1, rcv1) = bounded(1);
    let (snd2, rcv2) = bounded(2);
    let n_msgs = 4;
    let n_workers = 2;

    crossbeam::scope(|s| {
        // Producer thread
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd1.send(i).unwrap();
                println!("Source sent {}", i);
            }

            // Close the channel - this is necesarry to exit!
            // the for-loop in the wroker
            drop(snd1);
        });

        // Parallel processing by 2 threads
        for _ in 0..n_workers {
            // Send to sink, receive from source
            let (sendr, recvr) = (snd2.clone(), rcv1.clone());
            // Spawn workers in separate threads
            s.spawn(move |_| {
                thread::sleep(Duration::from_millis(500));

                for msg in recvr.iter() {
                    println!("Worker {:?} received {}.", thread::current().id(), msg);
                    sendr.send(msg * 2).unwrap();
                }
            });
        }
        // Clone the channel, otherwise sink will never
        // exit the for-loop
        drop(snd2);

        for msg in rcv2.iter() {
            println!("Sink received {}", msg);
        }
    })
    .unwrap()
}

fn pass_data_between_two_threads() {
    println!("\npass_data_between_two_threads - starts");
    let (snd, rcv) = unbounded();
    let n_msgs = 5;

    crossbeam::scope(|s| {
        s.spawn(|_| {
            for i in 0..n_msgs {
                snd.send(i).unwrap();
                thread::sleep(time::Duration::from_millis(100))
            }
        });
    })
    .unwrap();

    for _ in 0..n_msgs {
        let msg = rcv.recv().unwrap();
        println!("Received {}", msg);
    }
}
