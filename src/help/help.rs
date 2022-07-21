/*
pub fn help_input (
    mut commands: Commands,

    keys: Res<Input<KeyCode>>,

    mut ev_key: EventReader<KeyboardInput>,
    mut ev_exit: EventWriter<AppExit>,
    mut ev_window_change: EventWriter<WindowChangeEvent>,
    //mut ev_restart: EventWriter<RestartEvent>,
) {
    for ev in ev_key.iter() {
        if ev.state == ElementState::Pressed {
            match ev.key_code {
                Some(KeyCode::Escape) => {
                    ev_exit.send(AppExit);
                }
                Some(KeyCode::NumpadAdd) | Some(KeyCode::Equals) => {
                    ev_window_change.send(WindowChangeEvent(1));
                }
                Some(KeyCode::NumpadSubtract) | Some(KeyCode::Minus) => {
                    ev_window_change.send(WindowChangeEvent(-1));
                }
                Some(KeyCode::R) => {
                    if keys.pressed(KeyCode::LShift) || keys.pressed(KeyCode::RShift) {
                        commands.insert_resource(NextState(GameState::Restart));
                    }
                }

                _ => {}
            }
        }
    }
}
 */