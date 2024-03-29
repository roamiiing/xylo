use nanoid::nanoid;

const ALPHABET: [char; 16] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
];

pub fn hash() -> String {
    nanoid!(10, &ALPHABET)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash() {
        let hash = hash();
        assert_eq!(hash.len(), 10);
    }
}
