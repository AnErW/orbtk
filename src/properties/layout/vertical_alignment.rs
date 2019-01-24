use super::Margin;
use crate::structs::Spacer;

/// Used to vertical align a widget.
#[derive(Copy, Clone, PartialEq)]
pub enum VerticalAlignment {
    /// Align left.
    Top,

    /// Align center.
    Center,

    /// Align bottom.
    Bottom,

    /// Stretch to available height.
    Stretch,
}

impl Default for VerticalAlignment {
    fn default() -> Self {
        VerticalAlignment::Stretch
    }
}

impl VerticalAlignment {
    /// Calculates the y position of the widget depending on the available height, the goal height
    /// margin and vertical alignment.
    pub fn align_y(&self, available_height: f64, height: f64, margin: Margin) -> f64 {
        match self {
            VerticalAlignment::Bottom => available_height - height - margin.bottom(),
            VerticalAlignment::Center => (available_height - height) / 2.0,
            _ => margin.top(),
        }
    }

    /// Calculates the height of the widget depending on the available height, the goal height
    /// margin and vertical alignment.
    pub fn align_height(&self, available_height: f64, height: f64, margin: Margin) -> f64 {
        match self {
            VerticalAlignment::Stretch => available_height - margin.top() - margin.bottom(),
            _ => height,
        }
    }
}

impl<T: Into<String>> From<T> for VerticalAlignment {
    fn from(t: T) -> Self {
        match &t.into()[..] {
            "Top" | "top" => {
                VerticalAlignment::Top
            },
            "Center" | "center" => {
                VerticalAlignment::Center
            },
            "Bottom" | "bottom" => {
                VerticalAlignment::Bottom
            },
            _ => VerticalAlignment::Stretch
        }
    }
}