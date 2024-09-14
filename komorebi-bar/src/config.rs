use crate::widget::WidgetConfig;
use eframe::egui::Pos2;
use eframe::egui::TextBuffer;
use eframe::egui::Vec2;
use komorebi_client::Rect;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct KomobarConfig {
    /// Viewport options (see: https://docs.rs/egui/latest/egui/viewport/struct.ViewportBuilder.html)
    pub viewport: Option<ViewportConfig>,
    /// Frame options (see: https://docs.rs/egui/latest/egui/containers/struct.Frame.html)
    pub frame: Option<FrameConfig>,
    /// Monitor options
    pub monitor: MonitorConfig,
    /// Font family
    pub font_family: Option<String>,
    /// Theme
    pub theme: Option<KomobarTheme>,
    /// Left side widgets (ordered left-to-right)
    pub left_widgets: Vec<WidgetConfig>,
    /// Right side widgets (ordered left-to-right)
    pub right_widgets: Vec<WidgetConfig>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct ViewportConfig {
    /// The desired starting position of the bar (0,0 = top left of the screen)
    pub position: Option<Position>,
    /// The desired size of the bar from the starting position (usually monitor width x desired height)
    pub inner_size: Option<Position>,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct FrameConfig {
    /// Margin inside the painted frame
    pub inner_margin: Position,
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct MonitorConfig {
    /// Komorebi monitor index of the monitor on which to render the bar
    pub index: usize,
    /// Automatically apply a work area offset for this monitor to accommodate the bar
    pub work_area_offset: Option<Rect>,
}

impl KomobarConfig {
    pub fn read(path: &PathBuf) -> color_eyre::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let mut value: Self = match path.extension().unwrap().to_string_lossy().as_str() {
            "yaml" => serde_yaml::from_str(&content)?,
            "json" => serde_json::from_str(&content)?,
            _ => panic!("unsupported format"),
        };

        if value.frame.is_none() {
            value.frame = Some(FrameConfig {
                inner_margin: Position { x: 10.0, y: 10.0 },
            });
        }

        Ok(value)
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Position {
    /// X coordinate
    pub x: f32,
    /// Y coordinate
    pub y: f32,
}

impl From<Position> for Vec2 {
    fn from(value: Position) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<Position> for Pos2 {
    fn from(value: Position) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type")]
pub enum KomobarTheme {
    /// A theme from catppuccin-egui
    Catppuccin {
        name: komorebi_themes::Catppuccin,
        accent: Option<komorebi_themes::CatppuccinValue>,
    },
    /// A theme from base16-egui-themes
    Base16 {
        name: komorebi_themes::Base16,
        accent: Option<komorebi_themes::Base16Value>,
    },
}