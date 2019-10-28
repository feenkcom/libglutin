use super::*;

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
pub enum GlutinCursorIcon {
    /// The platform-dependent default cursor.
    Default,
    /// A simple crosshair.
    Crosshair,
    /// A hand (often used to indicate links in web browsers).
    Hand,
    /// Self explanatory.
    Arrow,
    /// Indicates something is to be moved.
    Move,
    /// Indicates text that may be selected or edited.
    Text,
    /// Program busy indicator.
    Wait,
    /// Help indicator (often rendered as a "?")
    Help,
    /// Progress indicator. Shows that processing is being done. But in contrast
    /// with "Wait" the user may still interact with the program. Often rendered
    /// as a spinning beach ball, or an arrow with a watch or hourglass.
    Progress,

    /// Cursor showing that something cannot be done.
    NotAllowed,
    ContextMenu,
    Cell,
    VerticalText,
    Alias,
    Copy,
    NoDrop,
    Grab,
    Grabbing,
    AllScroll,
    ZoomIn,
    ZoomOut,

    /// Indicate that some edge is to be moved. For example, the 'SeResize' cursor
    /// is used when the movement starts from the south-east corner of the box.
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
}

impl From<GlutinCursorIcon> for CursorIcon {
    fn from(cursor_icon: GlutinCursorIcon) -> CursorIcon {
        match cursor_icon {
            GlutinCursorIcon::Default => CursorIcon::Default,
            GlutinCursorIcon::Crosshair => CursorIcon::Crosshair,
            GlutinCursorIcon::Hand => CursorIcon::Hand,
            GlutinCursorIcon::Arrow => CursorIcon::Arrow,
            GlutinCursorIcon::Move => CursorIcon::Move,
            GlutinCursorIcon::Text => CursorIcon::Text,
            GlutinCursorIcon::Wait => CursorIcon::Wait,
            GlutinCursorIcon::Help => CursorIcon::Help,
            GlutinCursorIcon::Progress => CursorIcon::Progress,
            GlutinCursorIcon::NotAllowed => CursorIcon::NotAllowed,
            GlutinCursorIcon::ContextMenu => CursorIcon::ContextMenu,
            GlutinCursorIcon::Cell => CursorIcon::Cell,
            GlutinCursorIcon::VerticalText => CursorIcon::VerticalText,
            GlutinCursorIcon::Alias => CursorIcon::Alias,
            GlutinCursorIcon::Copy => CursorIcon::Copy,
            GlutinCursorIcon::NoDrop => CursorIcon::NoDrop,
            GlutinCursorIcon::Grab => CursorIcon::Grab,
            GlutinCursorIcon::Grabbing => CursorIcon::Grabbing,
            GlutinCursorIcon::AllScroll => CursorIcon::AllScroll,
            GlutinCursorIcon::ZoomIn => CursorIcon::ZoomIn,
            GlutinCursorIcon::ZoomOut => CursorIcon::ZoomOut,
            GlutinCursorIcon::EResize => CursorIcon::EResize,
            GlutinCursorIcon::NResize => CursorIcon::NResize,
            GlutinCursorIcon::NeResize => CursorIcon::NeResize,
            GlutinCursorIcon::NwResize => CursorIcon::NwResize,
            GlutinCursorIcon::SResize => CursorIcon::SResize,
            GlutinCursorIcon::SeResize => CursorIcon::SeResize,
            GlutinCursorIcon::SwResize => CursorIcon::SwResize,
            GlutinCursorIcon::WResize => CursorIcon::WResize,
            GlutinCursorIcon::EwResize => CursorIcon::EwResize,
            GlutinCursorIcon::NsResize => CursorIcon::NsResize,
            GlutinCursorIcon::NeswResize => CursorIcon::NeswResize,
            GlutinCursorIcon::NwseResize => CursorIcon::NwseResize,
            GlutinCursorIcon::ColResize => CursorIcon::ColResize,
            GlutinCursorIcon::RowResize => CursorIcon::RowResize,
        }
    }
}