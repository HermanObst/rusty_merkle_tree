mod merkle_tree;

fn main() {
    let merkle_tree = merkle_tree::create_merkle_tree(vec![1, 2, 3, 4]);
    println!("{:?}", merkle_tree);
}
