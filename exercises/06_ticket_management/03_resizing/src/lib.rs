#[cfg(test)]
mod tests {
    #[test]
    fn resizing() {
        let mut v = Vec::with_capacity(3);
        v.push(1);
        v.push(2); // max capacity reached
        assert_eq!(v.capacity(), 3);

        v.push(3); // beyond capacity, needs to resize
        v.push(4);

        // Can you guess what the new capacity will be?
        // Beware that the standard library makes no guarantees about the
        // algorithm used to resize the vector, so this may change in the future.
        assert_eq!(v.capacity(), 6);
    }
}
