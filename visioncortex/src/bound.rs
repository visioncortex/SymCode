use std::cmp::min;
use crate::{PointI32, disjoint_sets};

/// Any object that has a bounding rect
pub trait Bound {
    fn bound(&self) -> BoundingRect;
    fn overlaps<B: Bound>(&self, other: &B) -> bool {
        self.bound().hit(other.bound())
    }
}

/// The rectangle that bounds an object
#[derive(Copy, Clone, PartialEq, Default, Eq, Debug)]
pub struct BoundingRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

/// Statistics over a collection of objects with `Bound` trait
#[derive(Debug)]
pub struct BoundStat {
    pub average_area: i32,
    pub average_width: i32,
    pub average_height: i32,
    pub min_width: i32,
    pub min_height: i32,
}

impl BoundStat {
    pub fn calculate<B: Bound>(bs: &[B]) -> Self {
        let mut sum_area   = 0;
        let mut sum_width  = 0;
        let mut sum_height = 0;
        let mut min_width  = i32::MAX;
        let mut min_height = i32::MAX;

        for b in bs.iter() {
            let b      = b.bound();
            let width  = b.width();
            let height = b.height();

            sum_area   += width * height;
            sum_width  += width;
            sum_height += height;
            min_width   = min(min_width, width);
            min_height  = min(min_height, height);
        }

        let n = bs.len() as i32;

        Self {
            average_area:   sum_area / n,
            average_width:  sum_width / n,
            average_height: sum_height / n,
            min_width,
            min_height,
        }
    }
}

impl BoundingRect {
    // assume top-left origin
    pub fn new_x_y_w_h(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            left: x,
            top: y,
            right: x + w,
            bottom: y + h,
        }
    }

    pub fn width(self) -> i32 {
        self.right - self.left
    }

    pub fn height(self) -> i32 {
        self.bottom - self.top
    }

    pub fn is_empty(self) -> bool {
        self.width() == 0 && self.height() == 0
    }

    pub fn center(self) -> PointI32 {
        PointI32 {
            x: (self.left + self.right) >> 1,
            y: (self.top + self.bottom) >> 1,
        }
    }

    /// Calculates the squared distance betweeen the center of two `BoundingRect`s.
    pub fn sq_dist(self, other: Self) -> i32 {
        let diff = self.center() - other.center();
        diff.dot(diff)
    }

    pub fn aspect_ratio(self) -> f64 {
        std::cmp::max(self.width(), self.height()) as f64
            / std::cmp::min(self.width(), self.height()) as f64
    }

    pub fn aspect_ratio_doubled(self) -> i32 {
        2 * std::cmp::max(self.width(), self.height()) / std::cmp::min(self.width(), self.height())
    }

    pub fn add_x_y(&mut self, x: i32, y: i32) {
        if self.is_empty() {
            self.left = x;
            self.right = x + 1;
            self.top = y;
            self.bottom = y + 1;
            return;
        }
        if x < self.left {
            self.left = x;
        } else if x + 1 > self.right {
            self.right = x + 1;
        }
        if y < self.top {
            self.top = y;
        } else if y + 1 > self.bottom {
            self.bottom = y + 1;
        }
    }

    pub fn merge(&mut self, other: Self) {
        if other.is_empty() {
            return;
        }
        if self.is_empty() {
            self.left = other.left;
            self.right = other.right;
            self.top = other.top;
            self.bottom = other.bottom;
            return;
        }
        self.left = std::cmp::min(self.left, other.left);
        self.right = std::cmp::max(self.right, other.right);
        self.top = std::cmp::min(self.top, other.top);
        self.bottom = std::cmp::max(self.bottom, other.bottom);
    }

    pub fn clear(&mut self) {
        self.left = 0;
        self.right = 0;
        self.top = 0;
        self.bottom = 0;
    }

    pub fn hit(self, other: Self) -> bool {
        let r1 = self;
        let r2 = other;
        !(r2.left > r1.right ||
          r2.right < r1.left ||
          r2.top > r1.bottom ||
          r2.bottom < r1.top )
    }

    pub fn clip(&mut self, other: Self) {
        if self.left < other.left {
            self.left = other.left;
        }
        if self.top < other.top {
            self.top = other.top;
        }
        if self.right > other.right {
            self.right = other.right;
        }
        if self.bottom > other.bottom {
            self.bottom = other.bottom;
        }
    }

    pub fn squared(self) -> Self {
        let size = std::cmp::max(self.width(), self.height());
        Self::new_x_y_w_h(
            self.left - ((size - self.width()) >> 1),
            self.top - ((size - self.height()) >> 1),
            size,
            size,
        )
    }

    pub fn translate(&mut self, p: PointI32) {
        self.left += p.x;
        self.top += p.y;
        self.right += p.x;
        self.bottom += p.y;
    }
}

impl Bound for BoundingRect {
    fn bound(&self) -> BoundingRect {
        *self
    }
}

pub fn average_width<B: Bound>(bs: &[B]) -> i32 {
    let sum: i32 = bs
        .iter()
        .map(|b| b.bound().width())
        .sum();

    sum / (bs.len() as i32)
}

pub fn average_height<B: Bound>(bs: &[B]) -> i32 {
    let sum: i32 = bs
        .iter()
        .map(|b| b.bound().height())
        .sum();

    sum / (bs.len() as i32)
}

pub fn enclosing_bound<B: Bound>(bs: &[B]) -> BoundingRect {
    let mut enclosing = BoundingRect::default();

    for b in bs.iter() {
        enclosing.merge(b.bound());
    }

    enclosing
}

pub fn merge_expand<B: Bound>(items: Vec<B>, expand_x: i32, expand_y: i32) -> Vec<Vec<B>> {
    disjoint_sets::group_by_cached_key(
        items,
        |item| {
            expand(item.bound(), expand_x, expand_y)
        },
        |a, b| a.overlaps(b),
    )
}

pub fn expand(b: BoundingRect, expand_x: i32, expand_y: i32) -> BoundingRect {
    BoundingRect::new_x_y_w_h(
        b.left - expand_x,
        b.top - expand_y,
        b.width() + 2 * expand_x,
        b.height() + 2 * expand_y
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bounding_rect_1x1() {
        let mut rect = BoundingRect::default();
        rect.add_x_y(0, 0);
        assert_eq!(rect.left, 0);
        assert_eq!(rect.top, 0);
        assert_eq!(rect.right, 1);
        assert_eq!(rect.bottom, 1);
        assert_eq!(rect.width(), 1);
        assert_eq!(rect.height(), 1);
    }

    #[test]
    fn bounding_rect_2x2() {
        let mut rect = BoundingRect::default();
        rect.add_x_y(1, 1);
        rect.add_x_y(2, 2);
        assert_eq!(rect.left, 1);
        assert_eq!(rect.top, 1);
        assert_eq!(rect.right, 3);
        assert_eq!(rect.bottom, 3);
        assert_eq!(rect.width(), 2);
        assert_eq!(rect.height(), 2);
    }

    #[test]
    fn bounding_rect_aspect_ratio_doubled() {
        let mut rect = BoundingRect::default();
        rect.add_x_y(0, 0);
        rect.add_x_y(1, 0);
        assert_eq!(rect.aspect_ratio_doubled(), 4);
    }

    #[test]
    fn bounding_rect_clip() {
        let mut rect = BoundingRect::default();
        rect.add_x_y(1, 1);
        rect.add_x_y(4, 4);
        rect.clip(BoundingRect::new_x_y_w_h(0, 0, 3, 3));
        assert_eq!(rect, BoundingRect::new_x_y_w_h(1, 1, 2, 2));
    }

    #[test]
    fn enclosing_bound_test() {
        let mut a = BoundingRect::default();
        a.add_x_y(1, 1);
        let mut b = BoundingRect::default();
        b.add_x_y(2, 2);
        assert_eq!(
            enclosing_bound(&[a, b]),
            BoundingRect { left: 1, top: 1, right: 3, bottom: 3 }
        );
    }

    #[test]
    fn merge_expand_noop() {
        let mut a = BoundingRect::default();
        a.add_x_y(1, 1);
        let mut b = BoundingRect::default();
        b.add_x_y(3, 3);
        assert_eq!(
            merge_expand(vec![a, b], 0, 0),
            [[b],[a]]
        );
    }

    #[test]
    fn merge_expand_merged() {
        let mut a = BoundingRect::default();
        a.add_x_y(1, 1);
        let mut b = BoundingRect::default();
        b.add_x_y(3, 3);
        assert_eq!(
            merge_expand(vec![a, b], 1, 1),
            [[b,a]]
        );
    }

    #[test]
    fn merge_horizontal() {
        let mut a = BoundingRect::default();
        a.add_x_y(1, 1);
        let mut b = BoundingRect::default();
        b.add_x_y(3, 1);
        assert_eq!(
            merge_expand(vec![a, b], 1, 0),
            [[b,a]]
        );
    }

    #[test]
    fn merge_horizontal_noop() {
        let mut a = BoundingRect::default();
        a.add_x_y(1, 1);
        let mut b = BoundingRect::default();
        b.add_x_y(1, 3);
        assert_eq!(
            merge_expand(vec![a, b], 1, 0),
            [[b],[a]]
        );
    }

    #[test]
    fn merge_vertical() {
        let mut a = BoundingRect::default();
        a.add_x_y(1, 1);
        let mut b = BoundingRect::default();
        b.add_x_y(1, 3);
        assert_eq!(
            merge_expand(vec![a, b], 0, 1),
            [[b,a]]
        );
    }

    #[test]
    fn merge_vertical_noop() {
        let mut a = BoundingRect::default();
        a.add_x_y(1, 1);
        let mut b = BoundingRect::default();
        b.add_x_y(3, 1);
        assert_eq!(
            merge_expand(vec![a, b], 0, 1),
            [[b],[a]]
        );
    }
}
