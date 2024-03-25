extern crate crossbeam;
extern crate crossbeam_channel;

use std::env;
use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::path::Path;
use std::sync::mpsc::{channel, RecvError};
use std::sync::Mutex;
use std::time::Duration;
use std::{thread, time};

use crossbeam_channel::bounded;
use crossbeam_channel::unbounded;

use image::{ImageBuffer, Pixel, Rgb};
use lazy_static::lazy_static;
use num::Complex;
use ring::digest::{Context, Digest, SHA256};
use threadpool::ThreadPool;
use walkdir::WalkDir;
use rayon::prelude::*;

fn main() {
    spawn_short_lived_thread();
    parallel_pipeline();
    pass_data_between_two_threads();
    maintain_global_mutable_state().unwrap();
    calculate_sha256().unwrap();
    draw_fractal_dispatching_work().unwrap();
    mutate_array_parallel();
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

// Just copy & past from github :)
// Function converting intensity values to RGB
// Based on http://www.efg2.com/Lab/ScienceAndEngineering/Spectra.htm
fn wavelength_to_rgb(wavelength: u32) -> Rgb<u8> {
    let wave = wavelength as f32;

    let (r, g, b) = match wavelength {
        380..=439 => ((440. - wave) / (440. - 380.), 0.0, 1.0),
        440..=489 => (0.0, (wave - 440.) / (490. - 440.), 1.0),
        490..=509 => (0.0, 1.0, (510. - wave) / (510. - 490.)),
        510..=579 => ((wave - 510.) / (580. - 510.), 1.0, 0.0),
        580..=644 => (1.0, (645. - wave) / (645. - 580.), 0.0),
        645..=780 => (1.0, 0.0, 0.0),
        _ => (0.0, 0.0, 0.0),
    };

    let factor = match wavelength {
        380..=419 => 0.3 + 0.7 * (wave - 380.) / (420. - 380.),
        701..=780 => 0.3 + 0.7 * (780. - wave) / (780. - 700.),
        _ => 1.0,
    };

    let (r, g, b) = (
        normalize(r, factor),
        normalize(g, factor),
        normalize(b, factor),
    );
    Rgb::from_channels(r, g, b, 0)
}

// Just copy & past from github :)
// Normalizes color intensity values within RGB range
fn normalize(color: f32, factor: f32) -> u8 {
    ((color * factor).powf(0.8) * 255.) as u8
}

// Maps Julia set distance estimation to intensity values
fn julia(c: Complex<f32>, x: u32, y: u32, width: u32, height: u32, max_iter: u32) -> u32 {
    let width = width as f32;
    let height = height as f32;

    let mut z = Complex {
        // scale and translate the point to image coordinates
        re: 3.0 * (x as f32 - 0.5 * width) / width,
        im: 2.0 * (y as f32 - 0.5 * height) / height,
    };

    let mut i = 0;
    for t in 0..max_iter {
        if z.norm() >= 2.0 {
            break;
        }
        z = z * z + c;
        i = t;
    }
    i
}

fn draw_fractal_dispatching_work() -> Result<(), RecvError> {
    println!("\ndraw_fractal_dispatching_work - starts");
    let (width, height) = (1920, 1080);
    let mut img = ImageBuffer::new(width, height);
    let iterations = 300;

    let c = Complex::new(-0.8, 0.156);

    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel();

    for y in 0..height {
        let tx = tx.clone();
        pool.execute(move || {
            for x in 0..width {
                let i = julia(c, x, y, width, height, iterations);
                let pixel = wavelength_to_rgb(380 + i * 400 / iterations);
                tx.send((x, y, pixel)).expect("Could not send data!");
            }
        });
    }

    for _ in 0..(width * height) {
        let (x, y, pixel) = rx.recv()?;
        img.put_pixel(x, y, pixel);
    }

    let _ = img.save("output.png");
    Ok(())
}


fn mutate_array_parallel() {
    println!("\nmutate_array_parallel - starts");
    let mut arr = [0, 7, 9, 11];
    arr.par_iter_mut().for_each(|p| *p -= 1);
    println!("{:?}", arr);
}
