use crate::Accessible;

struct DesktopIter {
    index: i32,
    count: i32,
}

impl Default for DesktopIter {
    fn default() -> Self {
        Self {
            index: 0,
            count: crate::desktop_count(),
        }
    }
}

impl Iterator for DesktopIter {
    type Item = Accessible;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.count {
            let res = crate::desktop(self.index);
            self.index += 1;
            res
        } else {
            None
        }
    }
}

impl ExactSizeIterator for DesktopIter {
    fn len(&self) -> usize {
        self.count as usize
    }
}

pub fn desktops() -> impl ExactSizeIterator<Item = Accessible> {
    DesktopIter::default()
}
