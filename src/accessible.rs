pub use crate::auto::{traits::AccessibleExt, Accessible};

pub struct ChildIter<'a> {
    accessible: &'a Accessible,
    index: i32,
    count: i32,
}

impl Iterator for ChildIter<'_> {
    type Item = Result<Accessible, glib::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.count {
            let res = Some(self.accessible.child_at_index(self.index));
            self.index += 1;
            res
        } else {
            None
        }
    }
}

impl ExactSizeIterator for ChildIter<'_> {
    fn len(&self) -> usize {
        self.count as usize
    }
}

impl Accessible {
    pub fn children(&self) -> Result<ChildIter<'_>, glib::Error> {
        let count = self.child_count()?;
        Ok(ChildIter {
            accessible: self,
            index: 0,
            count,
        })
    }
}
