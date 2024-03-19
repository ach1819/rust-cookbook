use rand::distributions::{Alphanumeric, Distribution, Standard, Uniform};
use rand::{thread_rng, Rng};

fn main() {
    genarate_random_types();
    generate_random_range_numbers();
    // This could take some time
    generate_random_with_uniform();
    generate_random_values_of_custom_type();
    generate_random_password();
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
    println!("\n*** Starts - Generation random using uniform ***");
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

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
        }
    }
}

fn generate_random_values_of_custom_type() {
    println!("\n*** Starts - Generation random custom types ***");
    let mut rng = rand::thread_rng();
    let rand_tuple = rng.gen::<(i32, bool, f64)>();
    let rand_point: Point = rng.gen();
    println!("Random tuple: {:?}", rand_tuple);
    println!("Random Point: {:?}", rand_point);
}

fn generate_random_password() {
    println!("\n*** Starts - Generation random passwords ***");
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    println!("Your secure password: {}", rand_string);
}
