pub struct PulseChannel {
    sweep: Sweep,
    envelope: Envelope,
    sequencer: Sequencer,
    length_counter: LengthCounter,
    duty_cycle: usize,
}


impl PulseChannel {
    pub fn new(sweep_negation_mode: SweepNegationMode) -> Self {
        PulseChannel {
            envelope: Envelope::new(),
            length_counter: LengthCounter::new(),
            sweep: Sweep::new(sweep_negation_mode),
            duty_cycle: 0,
        }
    }

    pub fn write_register(&mut self, address: u16, value: u8) {
        match address % 4 {
            0 => {
                self.duty_cycle = value as usize >> 6;
                self.envelope.write_register(value);
                self.length_counter.set_halted(value & 0b0010_0000 != 0)
            }
            1 => self.sweep.write_register(value),
            2 => self.sequencer.set_period_low(value),
            3 => {
                self.length_counter.write_register(value);
                self.sequencer.set_period_high(value & 0b111);
                self.envelope.start();

            }

            _ => panic!(),
        }
    }

    pub fn sample(&self) -> u8 {
        if self.length_counter.active() && self.sequencer.period >= 8
            && self.sweep.target_period(&self.sequencer) < 0x800
        {
            PULSE_WAVEFORMS[self.duty_cycle][self.sequencer.current_step] * self.envelope.volume()
    }

    pub fn tick_sequencer(&mut self) {
        self.sequencer.tick(true);
    }

    pub fn playing(&mut self) -> bool {
        self.length_counter.playing()
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.length_counter.set_enabled(value);
    }
}
