pub trait App {
    type EventError;

    fn ticks(&self) -> u64;

    fn processed_events(&mut self) -> Result<bool, Self::EventError>;

    fn update(&mut self, delta: f64);

    fn draw(&mut self, interp: f64, total_time: u64);
}

pub fn gameloop<T: App>(app: &mut T, fps: i32) -> Result<(), T::EventError> {
    //We do not do 1/fps for delta because of the difference between 16 and 16.6...
    //Probably not a major difference
    let step = 1000 / fps as u64;
    let delta = step as f64 / 1000.0;

    let mut previous = app.ticks();
    let mut lag = 0;

    'mainloop: loop {
        let total = app.ticks();
        let current = total;
        let elapsed = current - previous;

        previous = current;
        lag += elapsed;

        if !app.processed_events()? {
            break 'mainloop;
        }

        while lag >= step {
            app.update(delta);

            lag -= step;
        }

        app.draw(lag as f64 / step as f64, total);
    }

    Ok(())
}
