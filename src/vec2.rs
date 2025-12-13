use std::{
    num::NonZeroU32,
    ops::{Index, IndexMut},
};

/// A 2D vector-like data structure that allocates memory in chunks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vec2<T> {
    data: Vec<Vec<T>>,
    len: usize,        // current number of elements
    cap: usize,        // allocated capacity
    chunk_size: usize, // number of elements per chunk
}

pub struct Iter<'a, T> {
    iter_row: Option<std::slice::Iter<'a, T>>,
    iter_rows: std::slice::Iter<'a, Vec<T>>,
}

pub struct IterMut<'a, T> {
    iter_row: Option<std::slice::IterMut<'a, T>>,
    iter_rows: std::slice::IterMut<'a, Vec<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut row_iter) = self.iter_row {
            let value = row_iter.next();
            if value.is_some() {
                return value;
            }
        }

        if let Some(arr) = self.iter_rows.next() {
            self.iter_row = Some(arr.iter());
        }

        if let Some(ref mut row_iter) = self.iter_row {
            return row_iter.next();
        }

        None
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut row_iter) = self.iter_row {
            let value = row_iter.next();
            if value.is_some() {
                return value;
            }
        }

        if let Some(arr) = self.iter_rows.next() {
            self.iter_row = Some(arr.iter_mut());
        }

        if let Some(ref mut row_iter) = self.iter_row {
            return row_iter.next();
        }

        None
    }
}

impl<T> Vec2<T> {
    /// Creates a new `Vec2` with the specified chunk size.
    #[inline]
    pub fn new(chunk_size: NonZeroU32) -> Self {
        Vec2 {
            data: Vec::new(),
            len: 0,
            cap: 0,
            chunk_size: chunk_size.get() as usize,
        }
    }

    /// Returns the number of elements in the `Vec2`.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the `Vec2` contains no elements.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the chunk size of the `Vec2`.
    #[inline]
    pub fn chunk_size(&self) -> usize {
        self.chunk_size
    }

    /// Returns the capacity of the `Vec2`.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.cap
    }

    /// Returns a reference to the element at the specified index, or `None` if out of bounds.
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            None
        } else {
            Some(&self[index])
        }
    }

    /// Returns a mutable reference to the element at the specified index, or `None` if out of bounds.
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            None
        } else {
            Some(&mut self[index])
        }
    }

    /// Clears the `Vec2`, removing all elements.
    #[inline]
    pub fn clear(&mut self) {
        for row in self.data.iter_mut() {
            row.clear();
        }
        self.len = 0;
    }

    /// Pushes a new element to the end of the `Vec2`.
    #[inline]
    pub fn push(&mut self, value: T) {
        if self.len == self.cap {
            self.data.push(Vec::with_capacity(self.chunk_size));
            self.cap += self.chunk_size;
        }
        self.data[self.len / self.chunk_size].push(value);
        self.len += 1;
    }

    /// Pops the last element from the `Vec2`.
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        self.data[self.len / self.chunk_size].pop()
    }

    /// Swaps the elements at the specified indices.
    #[inline]
    pub fn swap(&mut self, a: usize, b: usize) {
        let pa = &raw mut self[a];
        let pb = &raw mut self[b];
        unsafe {
            std::ptr::swap(pa, pb);
        }
    }

    /// Returns an iterator over the elements of the `Vec2`.
    #[inline]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            iter_row: None,
            iter_rows: self.data.iter(),
        }
    }

    /// Returns a mutable iterator over the elements of the `Vec2`.
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            iter_row: None,
            iter_rows: self.data.iter_mut(),
        }
    }
}

impl<T> Index<usize> for Vec2<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index / self.chunk_size][index % self.chunk_size]
    }
}

impl<T> IndexMut<usize> for Vec2<T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index / self.chunk_size][index % self.chunk_size]
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    #[test]
    fn test_vec2_push_pop() {
        use std::num::NonZeroU32;
        let mut vec2 = super::Vec2::new(NonZeroU32::new(3).unwrap());
        vec2.push(1);
        vec2.push(2);
        vec2.push(3);
        vec2.push(4);
        assert_eq!(vec2.len(), 4);
        assert_eq!(vec2.pop(), Some(4));
        assert_eq!(vec2.pop(), Some(3));
        assert_eq!(vec2.len(), 2);
    }

    #[test]
    fn test_vec2_capacity() {
        use std::num::NonZeroU32;
        let mut vec2 = super::Vec2::new(NonZeroU32::new(5).unwrap());
        assert_eq!(vec2.capacity(), 0);
        for i in 0..7 {
            vec2.push(i);
        }
        assert_eq!(vec2.capacity(), 10);
        assert_eq!(2, vec2.capacity() / vec2.chunk_size());
        assert_eq!(vec2.data[0].capacity(), 5);
        assert_eq!(vec2.data[1].capacity(), 5);
    }

    #[test]
    fn test_vec2_index() {
        use std::num::NonZeroU32;
        let mut vec2 = super::Vec2::new(NonZeroU32::new(2).unwrap());
        vec2.push(10);
        vec2.push(20);
        vec2.push(30);
        assert_eq!(vec2[0], 10);
        assert_eq!(vec2[1], 20);
        assert_eq!(vec2[2], 30);
        vec2[1] = 25;
        assert_eq!(vec2[1], 25);
    }

    #[test]
    fn test_vec2_iter() {
        use std::num::NonZeroU32;
        let mut vec2 = super::Vec2::new(NonZeroU32::new(3).unwrap());
        for i in 0..5 {
            vec2.push(i);
        }
        let collected: Vec<_> = vec2.iter().cloned().collect();
        assert_eq!(collected, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_vec2_iter_mut() {
        use std::num::NonZeroU32;
        let mut vec2 = super::Vec2::new(NonZeroU32::new(3).unwrap());
        for i in 0..5 {
            vec2.push(i);
        }
        for value in vec2.iter_mut() {
            *value *= 2;
        }
        let collected: Vec<_> = vec2.iter().cloned().collect();
        assert_eq!(collected, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_vec2_for_each() {
        use std::num::NonZeroU32;
        let mut vec2 = super::Vec2::new(NonZeroU32::new(4).unwrap());
        for i in 0..5 {
            vec2.push(i);
        }
        vec2.iter_mut().for_each(|x| *x += 1);
        let collected: Vec<_> = vec2.iter().cloned().collect();
        assert_eq!(collected, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_vec2_swap() {
        use std::num::NonZeroU32;
        let mut vec2 = super::Vec2::new(NonZeroU32::new(2).unwrap());
        vec2.push(1);
        vec2.push(2);
        vec2.push(3);
        vec2.push(4);
        vec2.swap(0, 3);
        assert_eq!(vec2[0], 4);
        assert_eq!(vec2[3], 1);
    }
}
