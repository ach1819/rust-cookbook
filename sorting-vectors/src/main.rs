fn main() {
    sort_integers_vector();
    sort_float_vertor();
}

fn sort_integers_vector() {
    let mut vec = vec![1, 5, 10, 2, 15];

    vec.sort();

    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
    println!("Sorted vector: {:?}", vec);
}

fn sort_float_vertor() {
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];

    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);

    println!("Sorted vector(float): {:?}", vec);
}
