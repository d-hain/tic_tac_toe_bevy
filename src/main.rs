#![warn(clippy::unwrap_used)]
#![allow(clippy::type_complexity)]

use belly::prelude::*;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const FIELD_COLOR: Color = Color::rgb(0.960784, 0.643137, 0.258823);

/// Marker [`Component`] for the UI root node of everything.
#[derive(Component)]
struct UiRoot;

/// A [`Cell`] stores its current state as a [`Option`]<[`PlayerState`]>.
#[derive(Component)]
struct Cell(Option<PlayerState>);

impl From<Cell> for String {
    fn from(value: Cell) -> Self {
        match value {
            Cell(Some(PlayerState::X)) => "X".to_string(),
            Cell(Some(PlayerState::O)) => "O".to_string(),
            Cell(None) => "".to_string(),
        }
    }
}

impl From<&Cell> for String {
    fn from(value: &Cell) -> Self {
        match value {
            Cell(Some(PlayerState::X)) => "X".to_string(),
            Cell(Some(PlayerState::O)) => "O".to_string(),
            Cell(None) => "".to_string(),
        }
    }
}

/// Player state of the game.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum PlayerState {
    X,
    O,
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
        .add_plugin(BellyPlugin)
        .add_plugin(WorldInspectorPlugin)
        .add_state(PlayerState::X)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_fields)
        // .add_startup_system(setup_text.after(setup_fields))
        .add_system(cell_click_system)
        .add_system(cell_state_change_system)
        .run();
}

/// Handles clicks on any [`Cell`].
/// Changes the [`PlayerState`] of the game
/// Changes the state of the clicked [`Cell`]
fn cell_click_system(
    mut interaction_query: Query<(&Interaction, &mut Cell), (Changed<Interaction>, With<Cell>)>,
    mut player: ResMut<State<PlayerState>>,
) {
    for (interaction, mut cell) in &mut interaction_query {
        if interaction == &Interaction::Clicked {
            let current_player = player.current().to_owned();

            // Switch Player
            player.set(match current_player {
                PlayerState::X => PlayerState::O,
                PlayerState::O => PlayerState::X,
            }).expect("player is not a PlayerState anymore?!");

            let _ = cell.0.insert(current_player);
        }
    }
}

/// Changes the text of a [`Cell`] when its state changes.
fn cell_state_change_system(
    cell_query: Query<(&Cell, &Children), Changed<Cell>>, //TODO: Changed<Cell> does not really work
    mut text_query: Query<&mut Text>,
) {
    for (cell, children) in cell_query.iter() {
        let mut text = text_query.get_mut(children[0]).expect("This is probably not a Cell!");
        // println!("changing state");
        text.sections[0].value = cell.into();
    }
}

/// Sets up a basic 2d camera.
fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(Name::new("Main Camera"));
}

/// Sets up the playing field by spawning a [`UiRoot`] Node and 9 [`Cell`]s.
fn setup_fields(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.add(eml! {
       <body s:padding = "50px">
            <button>"Belly hohoho"</button><br/>
            <button>"Belly hahaha"</button>
        </body>
    });
}

fn setup_text(mut commands: Commands, asset_server: Res<AssetServer>, ui_root_query: Query<Entity, With<UiRoot>>) {
    println!("setup");
    for ui_root_entity in ui_root_query.iter() {
        println!("for");
        // setup_top_text(&mut commands, &asset_server, ui_root_entity);
        setup_player_state_text(&mut commands, &asset_server);
    }
}

fn setup_top_text(commands: &mut ChildBuilder) {
    println!("toptext");
    commands
        .spawn(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexStart,
                justify_content: JustifyContent::FlexStart,
                size: Size::new(Val::Auto, Val::Auto),
                ..default()
            },
            text: Text::from_section("TicTacToe", TextStyle {
                // font: asset_server.load("ComicSansMS3.ttf"),
                font_size: 42.0,
                color: Color::BLACK,
                ..default()
            }),
            ..default()
        })
        // .set_parent(field_ui_root_entity)
        .insert(Name::new("TicTacToe Text"));
}

fn setup_player_state_text(mut commands: &mut Commands, asset_server: &AssetServer) {}