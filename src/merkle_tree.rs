use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn create_leaf(data_vec: Vec<i64>) -> Vec<u64> {
	let	mut leaf = Vec::new();
	for data in data_vec {
		leaf.push(calculate_hash(&data))
	};
	leaf
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
	let mut s = DefaultHasher::new();
	t.hash(&mut s);
	s.finish()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn leaf_have_same_length_than_data() {
        let leaf = create_leaf(vec![1,2,3]);

        assert_eq!(vec![1,2,3].len(), leaf.len());
    }
}
