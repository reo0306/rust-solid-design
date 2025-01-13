trait LightState<T> {
    fn switch(&self) -> T;
}

struct OffState;

impl OffState {
    fn new() -> Self { Self { } }
}

impl LightState<OnState> for OffState {
    fn switch(&self) -> OnState {
        println!("ライトを点灯します");
        OnState::new()
    }
}

struct OnState;

impl OnState {
    fn new() -> Self { Self { } }
}

impl LightState<OffState> for OnState {
    fn switch(&self) -> OffState {
        println!("ライトを消灯します");
        OffState::new()
    }
}

struct LightSwitch<T> {
    state: T,
}

impl<T: LightState<T>> LightSwitch<T> {
    fn new() -> Self {
        Self {
            state: OffState::new(),
        }
    }

    fn switch(&self) {
        self.state = self.state.switch();
    }
}

pub struct StateMain<T>;

impl<T> StateMain {
    pub fn index() {
        let light_state: LightSwitch<T> = LightSwitch::new();
        light_state.switch();
        light_state.switch();
        light_state.switch();
    }
}
