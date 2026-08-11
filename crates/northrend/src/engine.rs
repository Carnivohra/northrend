use northrend_backend::Backend;

pub struct Engine<B: Backend> {
    _backend: B,
}

impl<B: Backend> Engine<B> {
    pub fn new(_backend: B) -> Self {
        Self { _backend }
    }

    pub fn run(&self) {
        //let window = self.backend.create_window();

        loop {
            //let events = self.backend.poll_events();
        }
    }
}
