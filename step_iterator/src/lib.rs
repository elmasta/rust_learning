pub struct StepIterator<T> {
    beg: T,
    end: T,
    step: T,
    current: Option<T>,
}

impl<T> StepIterator<T>
where
    T: Copy + PartialOrd + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Default,
{
    pub fn new(beg: T, end: T, step: T) -> Self {
        Self {
            beg,
            end,
            step,
            current: None,
        }
    }
}

impl<T> Iterator for StepIterator<T>
where
    T: Copy + PartialOrd + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + Default,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_none() {
            self.current = Some(self.beg);
        } else {
            let next_value = self.current.unwrap() + self.step;
            if (self.step > Default::default() && next_value > self.end)
                || (self.step < Default::default() && next_value < self.end)
            {
                return None;
            }
            self.current = Some(next_value);
        }
        self.current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let mut step_iterator = StepIterator::new(0, 100, 10);
        assert_eq!(step_iterator.next(), Some(0));
        assert_eq!(step_iterator.next(), Some(10));
    }

    #[test]
    fn until_the_end() {
        for (i, v) in StepIterator::new(0, 100, 10).enumerate() {
            println!("position: {}, value: {}, ", i, v);
            assert_eq!(i * 10, v);
        }
    }

    #[test]
    fn test_with_floats() {
        for (i, v) in StepIterator::new(0.0, 10.0, 0.5).enumerate() {
            println!("position: {}, value: {}, ", i, v);
            assert_eq!(i as f64 * 0.5, v);
        }
    }

    #[test]
    fn test_with_floats_with_imperfect_range() {
        for (i, v) in StepIterator::new(0.3, 10.0, 0.5).enumerate() {
            println!("position: {}, value: {}, ", i, v);
            assert_eq!(i as f64 * 0.5 + 0.3, v);
        }
    }
}
