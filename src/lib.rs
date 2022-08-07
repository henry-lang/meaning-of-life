pub const MEANING_OF_LIFE: u8 = 42;

mod tests {
    #[test]
    pub fn is_correct() {
        assert_eq!(crate::MEANING_OF_LIFE, 42);
    }
}