// idea here: https://github.com/1-rafael-1/moving_median/blob/main/src/lib.rs
struct MovingMedianFilter {
    index: usize,
    values: [i16; 100],
}

impl MovingMedianFilter {
    // We inject the new value at self.index
    pub fn add_value(new_value: i16) -> i16 {
        todo!()
    }

    // Get the median of the current buffer
    pub fn get_median() -> i16 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_new_median() {
        assert_eq!(2, 3);
    }
}
