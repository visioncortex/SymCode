use bit_vec::BitVec;

pub trait Trace {
    fn bits(&self) -> &BitVec;

    /// the default implementation is to XOR the two bit strings and count the number of 1s
    fn diff(&self, other: &Self) -> usize {
        let (mut self_clone, mut other_clone) = (self.bits().clone(), other.bits().clone());
        self_clone.difference(&other.bits());
        other_clone.difference(&self.bits());
        self_clone.or(&other_clone);
        self_clone.into_iter().filter(|bit| *bit).count()
    }
}
