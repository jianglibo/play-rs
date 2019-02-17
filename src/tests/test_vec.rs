#[cfg(test)]
mod tests {
    use crate::tests::tutil::init_log;

    // scan stop on None.
    #[test]
    fn test_move_element() {
        init_log();
        let mut vec = vec![1, 2, 3];
        let mut vec2 = vec![4, 5, 6];
        vec.append(&mut vec2);
        assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
        assert_eq!(vec2, []);

        let mut vec = Vec::new();
        vec.push(0);
        vec.extend_from_slice(&[1, 2, 3]);
        assert_eq!(vec, [0,1,2,3]);
    }
}
