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
    let text_color = TextColor(Color::from(SCORE_COLOR));
    let text_font = TextFont {
        font: font.clone(),
        font_size: SCORE_FONT_SIZE,
        ..default()
    };
    let text_justification = JustifyText::Center;

    let score_text = "Score: 0".to_owned();
    let lines_text = "Lines: 0".to_owned();
    
    let text_score = Text2d::new(score_text);
    let text_lines = Text2d::new(lines_text);
    
    let transform_score = Transform::from_xyz(DISPLAY_SCORE_POSITION_X, DISPLAY_SCORE_POSITION_Y, 3.);
    let transform_lines = Transform::from_xyz(DISPLAY_SCORE_POSITION_X, DISPLAY_LINES_POSITION_Y, 3.);

    commands.spawn((ScoreComponent,
            text_score,
            text_color,
            text_font.clone(),
            TextLayout::new_with_justify(text_justification),
            transform_score
    ));
    commands.spawn((LinesComponent,
            text_lines,
            text_color,
            text_font.clone(),
            TextLayout::new_with_justify(text_justification),
            transform_lines
    ));
}

pub fn update_score(
    game_data: ResMut<GameData>,
    mut query_score: Query<&mut Text2d, With<ScoreComponent>>,
) {

    let score_text = "Score: ".to_owned() + &game_data.score.to_string();

    let mut text_score_component = query_score.single_mut();

    text_score_component.0 = score_text;
}

pub fn update_lines(
    game_data: ResMut<GameData>,
    mut query_lines: Query<&mut Text2d, With<LinesComponent>>,
) {

    let lines_text = "Lines: ".to_owned() + &game_data.lines.to_string();

    let mut text_lines_component = query_lines.single_mut();

    text_lines_component.0 = lines_text;
}
