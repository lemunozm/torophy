pub trait BorrowMutTwo<T> {
    fn get_two_mut<'a>(&'a mut self, index1: usize, index2: usize) -> (&'a mut T, &'a mut T);
}

impl<T> BorrowMutTwo<T> for Vec<T> {
    fn get_two_mut<'a>(&'a mut self, index1: usize, index2: usize) -> (&'a mut T, &'a mut T) {
        if index1 > index2 {
            let (a, b) = self.split_at_mut(index1);
            (&mut b[0], &mut a[index2])
        }
        else {
            let (a, b) = self.split_at_mut(index2);
            (&mut a[index1], &mut b[0])
        }
    }
}
