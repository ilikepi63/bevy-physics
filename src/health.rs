use bevy_health_bar3d::prelude::{HealthBarPlugin, Percentage};

#[derive(Component)]
struct Health {
    max: f32,
    current: f32,
}

impl Percentage for Health {
    fn value(&self) -> f32 {
        self.current / self.max
    }
}
