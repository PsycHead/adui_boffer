use crate::iter::{InplaceIter, OutOfPlaceIter};

pub struct InplaceBuffer<'a, T: Copy> {
    inout: &'a mut [T],
}

impl<'a, T: Copy> InplaceBuffer<'a, T> {
    pub fn new(inout: &'a mut [T]) -> Self {
        Self { inout }
    }
}

impl<'a, T: Copy> IoBuffer<T> for InplaceBuffer<'a, T> {
    fn iter<'r, 's>(&'s mut self) -> InplaceIter<'r, T>
    where
        T: 'r,
        's: 'r,
    {
        InplaceIter::new(self.inout)
    }
}

pub struct OutOfPlaceBuffer<'a, 'b, T: Copy> {
    src: &'a [T],
    dst: &'b mut [T],
}

impl<'a, 'b, T: Copy> OutOfPlaceBuffer<'a, 'b, T> {
    pub fn new(src: &'a [T], dst: &'b mut [T]) -> Self {
        Self { src, dst }
    }
}

impl<'a, 'b, T: Copy> IoBuffer<T> for OutOfPlaceBuffer<'a, 'b, T> {
    fn iter<'r, 's>(&'s mut self) -> OutOfPlaceIter<'r, 'r, T>
    where
        T: 'r,
        's: 'r,
    {
        OutOfPlaceIter::new(self.src, self.dst)
    }
}

trait IoBuffer<T> {
    fn iter<'a, 's>(&'s mut self) -> impl Iterator<Item = (T, &'a mut T)>
    where
        T: 'a,
        's: 'a;
}

#[cfg(test)]
mod tests {
    use super::{InplaceBuffer, IoBuffer, OutOfPlaceBuffer};

    #[test]
    fn inplace() {
        let mut data = [2f32; 8];
        let mut buff = InplaceBuffer::new(&mut data);
        let iter = buff.iter();

        for (src, dst) in iter {
            *dst *= src;
        }
    }

    #[test]
    fn outofplace() {
        let mut dst_data = [2f32; 8];
        let src_data = [1f32; 8];
        let mut buff = OutOfPlaceBuffer::new(&src_data, &mut dst_data);
        let iter = buff.iter();

        for (src, dst) in iter {
            *dst *= src;
        }
    }
}
