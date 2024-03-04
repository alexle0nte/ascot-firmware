#[derive(Clone)]
pub(crate) struct DummyLight {
    pub(crate) brightness: f64,
    pub(crate) save_energy: bool,
}

impl Default for DummyLight {
    fn default() -> Self {
        Self::init(4.0, true)
    }
}

impl DummyLight {
    pub(crate) const fn init(brightness: f64, save_energy: bool) -> Self {
        Self {
            brightness,
            save_energy,
        }
    }

    pub(crate) fn turn_light_on(&mut self, brightness: f64, save_energy: bool) {
        self.brightness = brightness;
        self.save_energy = save_energy;
        println!(
                "Dummy light turn light on action with brightness={brightness} and save energy={save_energy}"
            );
    }

    pub(crate) fn turn_light_off(&self) {
        println!("Run dummy light turn light off action");
    }

    pub(crate) fn toggle(&self) {
        println!("Run dummy light toggle action");
    }
}
