use bevy::prelude::*;
use crate::config::*;
use crate::game::game_data::GameData;

pub fn setup_score(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    game_data: ResMut<GameData>
) {
    let font = asset_server.load("fonts/textFont.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: SCORE_FONT_SIZE,
        color: SCORE_COLOR,
    };
    let text_justification = JustifyText::Center;

    let score_text = "Score: ".to_owned() + &game_data.score.to_string();
    let lines_text = "Lines: ".to_owned() + &game_data.lines.to_string();

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(score_text, text_style.clone())
                .with_justify(text_justification),
            transform: Transform::from_xyz(-DISPLAY_SCORE_POSITION_X, DISPLAY_SCORE_POSITION_Y, 0.),
            ..default()
        },
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(lines_text, text_style.clone())
                .with_justify(text_justification),
            transform: Transform::from_xyz(-DISPLAY_SCORE_POSITION_X, DISPLAY_LINES_POSITION_Y, 0.),
            ..default()
        },
    ));
}
