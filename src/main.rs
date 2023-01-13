#![warn(clippy::unwrap_used)]
#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const FIELD_COLOR: Color = Color::rgb(0.960784, 0.643137, 0.258823);

/// Marker [Component](bevy::prelude::Component) for the UI root node of all [Cells](Cell).
#[derive(Component)]
struct FieldUiRoot;

/// Marker [Component](bevy::prelude::Component) for the [Cells](Cell).
/// Cell data is the cells state to be one [Player](PlayerState) or None.
#[derive(Component)]
struct Cell(Option<PlayerState>);

/// Player state of the game.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum PlayerState {
    X,
    O,
}

impl Into<String> for PlayerState {
    fn into(self) -> String {
        match self {
            PlayerState::X => "X".to_string(),
            PlayerState::O => "O".to_string(),
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 420.0,
                height: 420.0,
                title: "TicTacToe in BEVY".to_string(),
                ..default()
            },
            ..default()
        }))
        .add_state(PlayerState::X)
        .add_plugin(WorldInspectorPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_fields)
        .add_system(cell_click_system)
        .add_system(cell_state_change_system)
        .run();
}

/// Handles clicks on any [Cell](Cell).
fn cell_click_system(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Cell>)>,
    mut player: ResMut<State<PlayerState>>,
) {
    for interaction in &mut interaction_query {
        if interaction == &Interaction::Clicked {
            let current_player = player.current().to_owned();
            
            // Switch Player
            player.set(match current_player {
                PlayerState::X => PlayerState::O,
                PlayerState::O => PlayerState::X,
            }).expect("player is not a PlayerState anymore?!");
        }
    }
}

/// Changes the text of a [Cell](Cell) when its state changes.
fn cell_state_change_system(
    cell_query: Query<(&Cell, &Children), Changed<Cell>>, 
    mut text_query: Query<&mut Text>,
) {
    println!("cell state change");
    for (cell, children) in cell_query.iter() {
        let mut text = text_query.get_mut(children[0]).expect("This is probably not a Cell!");
        println!("changing state");
        text.sections[0].value = match cell {
            Cell(Some(PlayerState::X)) => "X".to_string(),
            Cell(Some(PlayerState::O)) => "O".to_string(),
            Cell(None) => "".to_string(),
        };
    }
}

/// Sets up a basic 2d camera.
fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(Name::new("Main Camera"));
}

/// Sets up the playing field by spawning a [FieldUiRoot](FieldUiRoot) Node and 9 [Cells](Cell).
fn setup_fields(mut commands: Commands, asset_server: Res<AssetServer>){
    commands
        // FieldUiRoot Node
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                flex_wrap: FlexWrap::Wrap,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .insert(FieldUiRoot)
        .insert(Name::new("FieldUiRoot"))
        
        // Cells
        .with_children(|parent| {
            for _idx in 0..9 {
                parent
                    // Cell Button
                    .spawn(ButtonBundle {
                        style: Style {
                            align_self: AlignSelf::Center,
                            size: Size::new(Val::Percent(15.0), Val::Percent(15.0)),
                            margin: UiRect::all(Val::Percent(2.0)),
                            flex_basis: Val::Percent(25.0),
                            ..default()
                        },
                        background_color: FIELD_COLOR.into(),
                        ..default()
                    })
                    .insert(Cell(None))
                    .insert(Name::new("Cell"))
                    .with_children(|parent| {
                        parent
                            // Cell Text
                            .spawn(TextBundle {
                                style: Style {
                                    align_self: AlignSelf::Center,
                                    position: UiRect {
                                        left: Val::Percent(33.3),
                                        ..default()
                                    },
                                    ..default()
                                },
                                text: Text::from_section("", TextStyle {
                                    font: asset_server.load("ComicSansMS3.ttf"),
                                    font_size: 69.0,
                                    color: Color::BLACK.into(),
                                }).with_alignment(TextAlignment::CENTER),
                                ..default()
                            })
                            .insert(Name::new("Cell Text"));
                    });
            }
        });
}
