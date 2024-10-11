use bevy::prelude::*;
use crate::config::*;
use crate::game::game_data::GameData;

#[derive(Component)]
pub struct ScoreComponent;

#[derive(Component)]
pub struct LinesComponent;

pub fn init_score(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let font = asset_server.load("fonts/textFont.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: SCORE_FONT_SIZE,
        color: SCORE_COLOR,
    };
    let text_justification = JustifyText::Center;

    let score_text = "Score: 0".to_owned();
    let lines_text = "Lines: 0".to_owned();

    commands.spawn((ScoreComponent,
        Text2dBundle {
            text: Text::from_section(score_text, text_style.clone())
                .with_justify(text_justification),
            transform: Transform::from_xyz(-DISPLAY_SCORE_POSITION_X, DISPLAY_SCORE_POSITION_Y, 0.),
            ..default()
        },
    ));
    commands.spawn((LinesComponent,
        Text2dBundle {
            text: Text::from_section(lines_text, text_style.clone())
                .with_justify(text_justification),
            transform: Transform::from_xyz(-DISPLAY_SCORE_POSITION_X, DISPLAY_LINES_POSITION_Y, 0.),
            ..default()
        },
    ));
}

pub fn update_score(
    game_data: ResMut<GameData>,
    mut query_score: Query<&mut Text, With<ScoreComponent>>,
) {

    let score_text = "Score: ".to_owned() + &game_data.score.to_string();

    let mut text_score_component = query_score.single_mut();

    text_score_component.sections[0].value = score_text;
}

pub fn update_lines(
    game_data: ResMut<GameData>,
    mut query_lines: Query<&mut Text, With<LinesComponent>>,
) {

    let lines_text = "Lines: ".to_owned() + &game_data.lines.to_string();

    let mut text_lines_component = query_lines.single_mut();

    text_lines_component.sections[0].value = lines_text;
}
