#[derive(Resource)]
pub struct MatchTime(Timer)

impl MatchTime {
    pub fn new() -> Self {
        Self(Timer::from_seconds(60.0, TimerMode::Once)
    }
}

// We need to implement Default so that our timer can be initialized as
// a resource when we call `init_resource`
impl Default for MatchTime {
    fn default() -> Self {
        Self::new()
    }
}

fn countdown(
    time: Res<Time>,
    mut match_time: ResMut<MatchTime>
) {
    match_time.0.tick(time.delta());
}

fn end_match(match_time: Res<MatchTime>) {
    if match_time.0.finished() {
        // Here we would rest our game
    }
}
