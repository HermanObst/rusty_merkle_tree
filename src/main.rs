mod merkle_tree;

fn main() {
    merkle_tree::create_leaf(vec![1,2,3]);
    merkle_tree::create_leaf(vec![2,1,5]);
}

