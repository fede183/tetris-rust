use bevy::prelude::*;
use crate::config::*;

#[derive(Component)]
pub struct ScoreAndLines {
    pub score: i32,
    pub lines: i32,
}

pub fn init_score(mut commands: Commands) {
    commands.spawn(ScoreAndLines {score:0, lines:0});
}

pub fn setup_score(mut commands: Commands, asset_server: Res<AssetServer>, query: Query<&ScoreAndLines>) {
    let font = asset_server.load("fonts/textFont.ttf");
    let text_style = TextStyle {
        font: font.clone(),
        font_size: SCORE_FONT_SIZE,
        color: SCORE_COLOR,
    };
    let text_justification = JustifyText::Center;

    let score = query.single();

    let score_text = "Score: ".to_owned() + &score.score.to_string();
    let lines_text = "Lines: ".to_owned() + &score.lines.to_string();

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
