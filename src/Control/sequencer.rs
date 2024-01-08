pub struct Sequencer {
    pub counter: u16,
    pub period: u16,
    steps: usize,
    pub current_step: usize,
}

impl Sequencer {
    pub fn new(steps: usize) -> Self {
        Sequencer {
            counter: 0,
            period: 0,
            current_step: 0,
            steps,
        }
    }

    pub fn tick(&mut self, step_enabled: bool) -> bool {
        if self.counter == 0 {
            self.counter = self.period;
            if step_enabled {
                self.current_step = (self.current_step + 1) % self.steps;
            }
            true
        } else {
            self.counter -= 1;
            false
        }
    }

}
