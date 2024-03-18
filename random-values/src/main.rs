use rand::distributions::{Distribution, Uniform};
use rand::Rng;

fn main() {
    genarate_random_types();
    generate_random_range_numbers();
    // This could take some time
    generate_random_with_uniform();
}

fn genarate_random_types() {
    println!("*** Starts - Generation random by type ***");
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u16 = rng.gen();

    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random u32: {}", rng.gen::<u32>());
    println!("Random i32: {}", rng.gen::<i32>());
    println!("Random float: {}", rng.gen::<f64>());
}

fn generate_random_range_numbers() {
    println!("\n*** Starts - Generation random by range ***");
    let mut rng = rand::thread_rng();

    println!("Integer: {}", rng.gen_range(0..10));
    println!("Float: {}", rng.gen_range(0.0..10.0));
}

fn generate_random_with_uniform() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..7);

    loop {
        let throw = die.sample(&mut rng);
        println!("Roll the die: {}", throw);
        if throw == 6 {
            break;
        }
    }
}
