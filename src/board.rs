use bevy::prelude::*;
use crate::game::game_state::GameData;
use crate::{config::*, generate_rectangle};
use crate::game::piece::Piece;
use crate::game::point::Point;
use crate::game::point::PointColor;
use crate::common::RectangleWithBorder;

struct PieceComponentSprites {
    parent: SpatialBundle,
    children: Vec<SpriteBundle>,
}

impl PieceComponentSprites {
    fn new(children: Vec<SpriteBundle>) -> PieceComponentSprites {
        let parent = SpatialBundle::default();

        PieceComponentSprites {
            parent,
            children
        }
    }
}

enum PointMode {
    Board,
    Next
}

impl PointMode {
    fn get_position(&self, x: i32, y: i32, z: i32) -> Vec3 {
        let x_position = SQUARE_SIZE* (x as f32);
        let y_position = SQUARE_SIZE* (y as f32);

        let (x_position, y_position) = match self {
            PointMode::Board => (-DISPLAY_FIRST_BOARD_POSITION_X + x_position, DISPLAY_FIRST_BOARD_POSITION_Y - y_position),
            PointMode::Next => (DISPLAY_NEXT_PIECE_POSITION_X + x_position, DISPLAY_NEXT_PIECE_POSITION_Y + y_position),
        };

        Vec3 { x: x_position, y: y_position, z: z as f32}
    }
}

struct PieceToSpriteProvider {
    mode: PointMode
}

impl PieceToSpriteProvider {
    fn generate_point(&self, point: &Point) -> SpriteBundle {
        let color = get_color(point.color);
        let mode = &self.mode;
        let position = mode.get_position(point.x, point.y, 3);
        let sprite = generate_rectangle(position, SQUARE_SIZE, SQUARE_SIZE, color);

        sprite
    }

    pub fn generate_piece(&self, piece: &Piece) -> PieceComponentSprites {
        let sprites = piece.points.iter().map(|point| self.generate_point(point)).collect();

        PieceComponentSprites::new(sprites)
    }
}

const PROVIDER_BOARD_PIECE: PieceToSpriteProvider = PieceToSpriteProvider { mode: PointMode::Board };
const PROVIDER_NEXT_PIECE: PieceToSpriteProvider = PieceToSpriteProvider { mode: PointMode::Next };

#[derive(Component)]
pub struct PointComponent;

#[derive(Component)]
pub struct PieceComponent;

#[derive(Component)]
pub struct BoardPieceComponent;

#[derive(Component)]
pub struct NextPieceComponent;

#[derive(Component)]
pub struct RemainingPointsComponent;


fn get_color(color: PointColor) -> Color {
    match color {
        PointColor::RED => Color::RED,
        PointColor::BLUE => Color::BLUE,
        PointColor::YELLOW => Color::YELLOW,
        PointColor::GREEN => Color::GREEN,
    }
}

fn generate_cell_on_board(commands: &mut Commands) {
    let square_size = SQUARE_SIZE - 5.;
    let cell = RectangleWithBorder::new(square_size, square_size, 5., BOARD_COLOR, BORDER_SQUARE_COLOR);

    for x in 0..10 {
        for y in 0..20 {
            cell.spawn(commands, PointMode::Board.get_position(x, y, 1));
        }
    }
}

fn spawn_piece_as_child(commands: &mut ChildBuilder, sprites: Vec<SpriteBundle>) {
    for sprite in sprites {
        commands.spawn(sprite);
    }
}

fn spawn_piece(commands: &mut Commands, game_data: &ResMut<GameData>) {
    let piece = &game_data.piece;
    let sprite_piece = PROVIDER_BOARD_PIECE.generate_piece(piece);

    commands.spawn(BoardPieceComponent)
        .insert(PieceComponent)
        .insert(sprite_piece.parent)
        .with_children(|parent| { 
            for sprite in sprite_piece.children {
                parent.spawn(sprite);
            }
        });
}

fn spawn_next_piece(commands: &mut Commands, game_data: &ResMut<GameData>) {
    let next_piece = &game_data.next_piece;
    let sprite_next_piece = PROVIDER_NEXT_PIECE.generate_piece(next_piece);

    commands.spawn(NextPieceComponent)
        .insert(PieceComponent)
        .insert(sprite_next_piece.parent)
        .with_children(|parent| { 
            spawn_piece_as_child(parent, sprite_next_piece.children);
        });
}

pub fn init_board(mut commands: Commands) {
    RectangleWithBorder::new(DISPLAY_BOARD_HEIGHT, DISPLAY_BOARD_WIGTH, BORDER_SIZE, BOARD_COLOR, BORDER_COLOR).spawn(&mut commands, Vec3{ x: 0., y: 0., z: 0.});
    
    generate_cell_on_board(&mut commands);

    RectangleWithBorder::new(DISPLAY_NEXT_PIECE_HEIGHT, DISPLAY_NEXT_PIECE_WIGTH, BORDER_SIZE, BOARD_COLOR, BORDER_COLOR).spawn(&mut commands, Vec3{ x: DISPLAY_NEXT_PIECE_POSITION_X, y: DISPLAY_NEXT_PIECE_POSITION_Y, z: 0.});
}

pub fn init_board_pieces(mut commands: Commands, game_data: ResMut<GameData>) {
    spawn_piece(&mut commands, &game_data);
    spawn_next_piece(&mut commands, &game_data);
}
