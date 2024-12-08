use bitvec::vec::BitVec;

pub trait BitSetMember {
    fn from_index(index: usize) -> Self;
    fn to_index(self) -> usize;
    fn max() -> Self;
}

pub struct BitSet<T> {
    vec: BitVec,
    _marker: std::marker::PhantomData<T>,
}

impl<T: BitSetMember> BitSet<T> {
    pub fn add(&mut self, value: T) -> bool {
        self.vec.replace(value.to_index(), true)
    }

    pub fn remove(&mut self, value: T) -> bool {
        self.vec.replace(value.to_index(), false)
    }

    pub fn has(&self, value: T) -> bool {
        self.vec[value.to_index()]
    }

    pub fn count(&self) -> usize {
        self.vec.count_ones()
    }

    pub fn empty(&mut self) {
        self.vec.fill(false);
    }

    pub fn iter(&self) -> impl Iterator<Item = T> + use<'_, T> {
        self.vec.iter_ones().map(|index| T::from_index(index))
    }
}

impl<T: BitSetMember> Default for BitSet<T> {
    fn default() -> Self {
        Self {
            vec: BitVec::repeat(false, T::max().to_index() + 1),
            _marker: std::marker::PhantomData,
        }
    }
}
