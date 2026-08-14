pub enum WindowEventKind {
    CloseRequested,
    Resized { width: u32, height: u32 },
    RedrawRequested,
}
