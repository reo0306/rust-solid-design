use std::sync::Arc;

trait LightState {
    fn switch(&self) -> Arc<dyn LightState>;
}

struct OffState;

impl OffState {
    fn new() -> Self { Self { } }
}

impl LightState for OffState {
    fn switch(&self) -> Arc<dyn LightState> {
        println!("ライトを点灯します");
        Arc::new(OnState::new())
    }
}

struct OnState;

impl OnState {
    fn new() -> Self { Self { } }
}

impl LightState for OnState {
    fn switch(&self) -> Arc<dyn LightState> {
        println!("ライトを消灯します");
        Arc::new(OffState::new())
    }
}

struct LightSwitch {
    state: Arc<dyn LightState>,
}

impl LightSwitch {
    fn new(state: Arc<dyn LightState>) -> Self {
        Self { state }
    }

    fn switch(&mut self) {
        self.state = self.state.switch();
    }
}

pub struct StateMain;

impl StateMain {
    pub fn index() {
        let mut light_state = LightSwitch::new(Arc::new(OffState::new()));
        light_state.switch();
        light_state.switch();
        light_state.switch();
    }
}
