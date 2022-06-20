use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// given a vector of data, returns another vector with the hash of each element.
pub fn create_leaf(data_vec: Vec<i64>) -> Vec<u64> {
    let mut leaf = Vec::new();
    for data in data_vec {
        leaf.push(calculate_hash(&data))
    }
    println!("{:?}", leaf);
    leaf
}

pub fn create_internal_nodes(previous_hash_list: Vec<u64>) -> Vec<u64> {
    let mut parent_nodes = Vec::new();
    let mut i = 0;

    while i < previous_hash_list.len() {
        parent_nodes.push(calculate_hash(
            &(previous_hash_list[i] + previous_hash_list[i + 1]),
        ));
        i += 2;
    }
    parent_nodes
}

pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn leaf_have_same_length_than_data() {
        let leaf = create_leaf(vec![1, 2, 3]);

        assert_eq!(vec![1, 2, 3].len(), leaf.len());
    }

    #[test]
    fn hashes_of_same_numbers_are_the_same() {
        let leaf = create_leaf(vec![1, 2, 3]);
        let leaf2 = create_leaf(vec![1, 2, 3]);

        assert_eq!(leaf, leaf2);
    }

    #[test]
    fn parent_node_list_have_half_of_child_list_len() {
        let parent_nodes = create_internal_nodes(vec![1, 2, 3, 4]);

        assert_eq!(parent_nodes.len(), vec![1, 2, 3, 4].len() / 2)
    }
}
