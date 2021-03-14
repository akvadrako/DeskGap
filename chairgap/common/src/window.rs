use crate::geo::Point;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub enum Location {
    Center,
    Exact(Point),
}

#[derive(Debug, Eq, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub enum Event {
    Blur,
    Focus,
    Move,
    Resize,
    Close,
}

#[cfg(target_os = "macos")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TitleStyle {
    Visible,
    Hidden {
        traffic_light_position: Option<Point>,
    },
}
#[cfg(target_os = "macos")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VibrancyConfig {
    pub size: u32,
}

#[cfg(target_os = "macos")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct VibrancyUpdate {
    pub top: Option<VibrancyConfig>,
    pub left: Option<VibrancyConfig>,
    pub bottom: Option<VibrancyConfig>,
    pub right: Option<VibrancyConfig>,
}
