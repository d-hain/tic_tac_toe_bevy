use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const FIELD_COLOR: Color = Color::rgb(0.960784, 0.643137, 0.258823);

/// Marker [Component](bevy::prelude::Component) for the UI root node of all [Cells](Cell)
#[derive(Component)]
struct FieldUiRoot;

/// Marker [Component](bevy::prelude::Component) for the [Cells](Cell)
#[derive(Component)]
struct Cell;

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
        .add_plugin(WorldInspectorPlugin)
        .add_startup_system(setup_camera)
        .add_startup_system(setup_fields)
        .add_system(field_click_system)
        .run();
}

/// Handles clicks on any [Field](Cell)
fn field_click_system(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Cell>)>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        if interaction == &Interaction::Clicked {
            text.sections[0].value = "Test".to_string();
        }
    }
}

/// Sets up a basic 2d camera
fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(Name::new("Main Camera"));
}

/// Sets up the playing field by spawning a [FieldUiRoot](FieldUiRoot) Node and 9 [Cells](Cell)
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
                    .insert(Cell)
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
                                text: Text::from_section("O", TextStyle {
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
