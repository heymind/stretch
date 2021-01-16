use std::{
    marker::PhantomData,
    ops::{Add, Sub},
};

use euclid::{point2, size2};

impl<T, U> Into<euclid::Rect<T, U>> for crate::geometry::Rect<T>
where
    T: Copy + Sub<Output = T>,
{
    fn into(self) -> euclid::Rect<T, U> {
        euclid::Rect {
            origin: point2(self.start, self.top),
            size: size2(self.end - self.start, self.bottom - self.top),
        }
    }
}

impl<T, U> From<euclid::Rect<T, U>> for crate::geometry::Rect<T>
where
    T: Copy + Add<Output = T>,
{
    fn from(r: euclid::Rect<T, U>) -> Self {
        Self { start: r.origin.x, end: r.origin.x + r.size.width, top: r.origin.y, bottom: r.origin.y + r.size.height }
    }
}
impl<U> From<euclid::Rect<f32, U>> for crate::geometry::Rect<crate::style::Dimension> {
    fn from(r: euclid::Rect<f32, U>) -> Self {
        let r: crate::geometry::Rect<f32> = r.into();
        Self {
            start: crate::style::Dimension::Points(r.start),
            end: crate::style::Dimension::Points(r.end),
            top: crate::style::Dimension::Points(r.top),
            bottom: crate::style::Dimension::Points(r.bottom),
        }
    }
}

impl<T, U> Into<euclid::Point2D<T, U>> for crate::geometry::Point<T> {
    fn into(self) -> euclid::Point2D<T, U> {
        euclid::Point2D { x: self.x, y: self.y, _unit: PhantomData }
    }
}
impl<T, U> From<euclid::Point2D<T, U>> for crate::geometry::Point<T> {
    fn from(p: euclid::Point2D<T, U>) -> Self {
        Self { x: p.x, y: p.y }
    }
}
impl<T, U> Into<euclid::Size2D<T, U>> for crate::geometry::Size<T> {
    fn into(self) -> euclid::Size2D<T, U> {
        euclid::Size2D { width: self.width, height: self.height, _unit: PhantomData }
    }
}
impl<T, U> From<euclid::Size2D<T, U>> for crate::geometry::Size<T> {
    fn from(s: euclid::Size2D<T, U>) -> Self {
        Self { width: s.width, height: s.height }
    }
}
