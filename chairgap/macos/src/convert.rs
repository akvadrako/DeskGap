use chairgap_common::geo::{Point, Rect, Size};
use chairgap_common::window::VibrancyConfig;
use chairgap_macos_src::DGVisualEffectViewConfig;
use cocoa::appkit::CGFloat;
use cocoa::foundation::{NSPoint, NSRect, NSSize};
// Orphan rule doesn't allow us to impl `From` here, so we define our own alternative
pub(crate) trait ConvertTo<T> {
    fn convert(self) -> T;
}

impl ConvertTo<DGVisualEffectViewConfig> for Option<VibrancyConfig> {
    fn convert(self) -> DGVisualEffectViewConfig {
        match self {
            None => DGVisualEffectViewConfig { size: -1 },
            Some(config) => DGVisualEffectViewConfig {
                size: config.size as i32,
            },
        }
    }
}

impl ConvertTo<Size> for NSSize {
    fn convert(self) -> Size {
        Size {
            width: self.width as u32,
            height: self.height as u32,
        }
    }
}

impl ConvertTo<NSSize> for Size {
    fn convert(self) -> NSSize {
        NSSize {
            width: self.width as CGFloat,
            height: self.height as CGFloat,
        }
    }
}

impl ConvertTo<Point> for NSPoint {
    fn convert(self) -> Point {
        Point {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

impl ConvertTo<NSPoint> for Point {
    fn convert(self) -> NSPoint {
        NSPoint {
            x: self.x as CGFloat,
            y: self.y as CGFloat,
        }
    }
}

impl ConvertTo<Rect> for NSRect {
    fn convert(self) -> Rect {
        Rect {
            position: self.origin.convert(),
            size: self.size.convert(),
        }
    }
}

impl ConvertTo<NSRect> for Rect {
    fn convert(self) -> NSRect {
        NSRect {
            origin: self.position.convert(),
            size: self.size.convert(),
        }
    }
}
