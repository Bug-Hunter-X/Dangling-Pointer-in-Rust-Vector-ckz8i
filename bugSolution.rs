fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of getting the raw pointer, use indexing or iterators
    v[0] = 10;
    println!("Modified vector: {:?}", v);
}