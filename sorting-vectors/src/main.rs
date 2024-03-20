fn main() {
    sort_integers_vector();
}


fn sort_integers_vector() {
    let mut vec = vec![1, 5, 10, 2, 15];

    vec.sort();

    assert_eq!(vec, vec![1, 2, 5, 10, 15]);
    println!("Sorted vector: {:?}", vec);
}
