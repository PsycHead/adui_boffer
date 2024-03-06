use std::slice::{Iter, IterMut};

pub struct InplaceIter<'a, T: Clone> {
    inout_iter: IterMut<'a, T>,
}

impl<'a, T: Clone> InplaceIter<'a, T> {
    pub fn new<'r: 'a>(inout: &'r mut [T]) -> Self {
        Self {
            inout_iter: inout.iter_mut(),
        }
    }
}

impl<'a, T: Clone> Iterator for InplaceIter<'a, T> {
    type Item = (T, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.inout_iter.next()?;
        Some((item.clone(), item))
    }
}

pub struct OutOfPlaceIter<'a, 'b, T: Clone> {
    src_iter: Iter<'a, T>,
    dst_iter: IterMut<'b, T>,
}

impl<'a, 'b, T: Clone> OutOfPlaceIter<'a, 'b, T> {
    pub fn new(src: &'a [T], dst: &'b mut [T]) -> Self {
        Self {
            src_iter: src.iter(),
            dst_iter: dst.iter_mut(),
        }
    }
}

impl<'a, 'b, T: Clone> Iterator for OutOfPlaceIter<'a, 'b, T> {
    type Item = (T, &'b mut T);

    fn next(&mut self) -> Option<Self::Item> {
        let src = self.src_iter.next()?;
        let dst = self.dst_iter.next()?;
        Some((src.clone(), dst))
    }
}

#[cfg(test)]
mod tests {
    use crate::iter::{InplaceIter, OutOfPlaceIter};

    #[test]
    fn inplace() {
        let mut data = vec![2f32; 8];

        let inplace_iter = InplaceIter::new(&mut data);
        for (src, dst) in inplace_iter {
            *dst += src;
        }

        assert_eq!(&data.as_slice(), &vec![4f32; 8].as_slice());
    }

    #[test]
    fn outofplace() {
        let mut dst_data = vec![2f32; 8];
        let src_data = dst_data.clone();

        let inplace_iter = OutOfPlaceIter::new(&src_data, &mut dst_data);
        for (src, dst) in inplace_iter {
            *dst += src;
        }

        assert_eq!(&dst_data.as_slice(), &vec![4f32; 8].as_slice());
    }
}
