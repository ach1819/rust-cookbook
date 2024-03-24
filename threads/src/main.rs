extern crate crossbeam;
extern crate crossbeam_channel;

use std::env;
use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::Mutex;
use std::time::Duration;
use std::{thread, time};

use crossbeam_channel::bounded;
use crossbeam_channel::unbounded;

use lazy_static::lazy_static;
use ring::digest::{Context, Digest, SHA256};
use threadpool::ThreadPool;
use walkdir::WalkDir;

fn main() {
    spawn_short_lived_thread();
    parallel_pipeline();
    pass_data_between_two_threads();
    maintain_global_mutable_state().unwrap();
    calculate_sha256().unwrap();
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

lazy_static! {
    static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn insert(fruit: &str) -> Result<(), &'static str> {
    let mut db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;
    db.push(fruit.to_string());
    Ok(())
}

fn maintain_global_mutable_state() -> Result<(), &'static str> {
    println!("\nmaintain_global_mutable_state - starts");
    insert("apple")?;
    insert("orange")?;
    insert("peach")?;
    {
        let db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;
        db.iter()
            .enumerate()
            .for_each(|(i, item)| println!("{}: {}", i, item))
    }
    insert("grape")?;
    Ok(())
}

fn compute_digest<P: AsRef<Path>>(filepath: P) -> Result<(Digest, P), Error> {
    let mut buf_reader = BufReader::new(File::open(&filepath)?);
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = buf_reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }
    Ok((context.finish(), filepath))
}

fn calculate_sha256() -> Result<(), Error> {
    println!("\ncalculate_sha256 - starts");
    let pool = ThreadPool::new(num_cpus::get());

    let (tx, rx) = channel();
    let path = env::var("PATH_TO_SHA").unwrap_or(String::from(""));

    if path == "" {
        println!("No path to calculate sha");
        return Ok(());
    }

    for entry in WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| !e.path().is_dir())
    {
        let path = entry.path().to_owned();
        let tx = tx.clone();
        pool.execute(move || {
            let digest = compute_digest(path);
            tx.send(digest).expect("Could not send data!");
        });
    }
    drop(tx);
    for t in rx.iter() {
        let (sha, path) = t?;
        println!("{:?} {:?}", sha, path);
    }
    Ok(())
}
