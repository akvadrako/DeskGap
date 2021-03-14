use cocoa::foundation::{NSSize, NSPoint, NSRect};
use crate::geo::{Size, Point, Rect};
use cocoa::appkit::CGFloat;

impl From<Size> for NSSize {
    fn from(s: Size) -> Self {
        Self { width: s.width as CGFloat, height: s.height as CGFloat }
    }
}

impl From<Point> for NSPoint {
    fn from(p: Point) -> Self {
        Self { x: p.x as CGFloat, y: p.y as CGFloat}
    }
}

impl From<Rect> for NSRect {
    fn from(r: Rect) -> Self {
        Self {
            origin: r.position.into(),
            size: r.size.into()
        }
    }
}
impl From<NSSize> for Size {
    fn from(s: NSSize) -> Self {
        Self { width: s.width as u32, height: s.height as u32 }
    }
}

impl From<NSPoint> for Point {
    fn from(p: NSPoint) -> Self {
        Self { x: p.x as i32, y: p.y as i32 }
    }
}

impl From<NSRect> for Rect {
    fn from(r: NSRect) -> Self {
        Self {
            position: r.origin.into(),
            size: r.size.into()
        }
    }
}
