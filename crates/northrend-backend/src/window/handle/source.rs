use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub(super) trait WindowHandleSource:
    HasDisplayHandle + HasWindowHandle + Send + Sync {}

impl<T> WindowHandleSource for T
    where T: HasDisplayHandle + HasWindowHandle + Send + Sync {}
