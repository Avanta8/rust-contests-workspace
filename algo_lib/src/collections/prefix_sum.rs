use std::ops::{
    Add,
    Bound::{Excluded, Included, Unbounded},
    RangeBounds, Sub,
};

#[derive(Debug, Clone)]
pub struct PrefixSum<T> {
    vec: Vec<T>,
}

impl<T> PrefixSum<T>
where
    T: Add + Sub + Default,
    for<'a> &'a T: Add<Output = T> + Sub<Output = T>,
{
    pub fn new(data: &[T]) -> Self {
        let mut vec = vec![T::default()];
        for elem in data.iter() {
            vec.push(unsafe { vec.last().unwrap_unchecked() } + elem);
        }

        Self { vec }
    }

    pub fn get<R: RangeBounds<usize>>(&self, range: R) -> T {
        let start = match range.start_bound() {
            Included(&t) => t,
            Excluded(&t) => t + 1,
            Unbounded => 0,
        };
        let end = match range.end_bound() {
            Included(&t) => t + 1,
            Excluded(&t) => t,
            Unbounded => self.vec.len() - 1,
        };

        &self.vec[end] - &self.vec[start]
    }
}
