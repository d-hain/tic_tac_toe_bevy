use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

//region Bevy Book - Getting Started
// #[derive(Component)]
// struct Person;
// 
// #[derive(Component)]
// struct Name(String);
// 
// #[derive(Resource)]
// struct GreetTimer(Timer);
// 
// pub struct HelloPlugin;
// 
// impl Plugin for HelloPlugin {
//     fn build(&self, app: &mut App) {
//         app.insert_resource(GreetTimer(Timer::from_seconds(2., TimerMode::Repeating)))
//             // ^^^ Add a GreetTimer that repeats every 2 seconds
//             .add_startup_system(add_people) // Startup systems run once on startup (obviously)
//             // .add_system(hello_world) // does not have to be executed before greet_people system
//             // Systems are executed in parallel when possible
//             .add_system(greet_people);
//     }
// }
// 
// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins) // Adds the DefaultPlugins of Bevy
//         // Adds an event loop so systems are called every frame
//         .add_plugin(HelloPlugin)
//         .run();
// }
// 
// // /// System that prints "hello world!"
// // fn hello_world() {
// //     println!("hello world!");
// // }
// 
// /// System that spawns 3 Persons with a Name
// fn add_people(mut commands: Commands) {
//     commands.spawn((Person, Name("David Hain".to_string())));
//     commands.spawn((Person, Name("Eren Jäger".to_string())));
//     commands.spawn((Person, Name("The Primeagen".to_string())));
// }
// 
// /// System that searches all Names with the Person component
// /// (all Persons with a Name)
// /// Prints "hello NAME!" for every Person with a Name
// fn greet_people(
//     time: Res<Time>,
//     mut timer: ResMut<GreetTimer>,
//     query: Query<&Name, With<Person>>
// ) {
//     // only greets people when the timer has run out
//     // just_finished() only sends true on the tick the timer resets
//     if timer.0.tick(time.delta()).just_finished() {
//         for name in query.iter() {
//             println!("hello {}!", name.0);
//         }
//     }
// }
//endregion

const FIELD_COLOR: Color = Color::rgb(0.960784, 0.643137, 0.258823);

#[derive(Component)]
struct FieldUiRoot;

#[derive(Component)]
struct Field;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "TicTacToe in BEVY".to_string(),
                ..default()
            },
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin)
        .add_startup_system(setup_camera)
        // .add_startup_system(setup_scene)
        .add_startup_system(test_ui_system)
        .add_system(field_click_system)
        .run();
}

fn test_ui_system(mut commands: Commands){
   commands
       .spawn(NodeBundle {
           style: Style {
               size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
               justify_content: JustifyContent::Center,
               flex_wrap: FlexWrap::Wrap,
               ..default()
           },
           background_color: Color::NONE.into(),
           ..default()
       })
       .insert(FieldUiRoot)
       .insert(Name::new("FieldUIRoot"))
       .with_children(|parent| {
           for _idx in 0..9 {
               parent
                   .spawn(ButtonBundle {
                       style: Style {
                           size: Size::new(Val::Percent(15.0), Val::Percent(15.0)),
                           align_self: AlignSelf::Center,
                           margin: UiRect::all(Val::Percent(2.0)),
                           flex_basis: Val::Percent(25.0),
                           ..default()
                       },
                       background_color: FIELD_COLOR.into(),
                       ..default()
                   })
                   .insert(Field)
                   .insert(Name::new("Field"));
           }
       });
}

fn field_click_system(
    mut interaction_query: Query<(&Interaction, &Children), (Changed<Interaction>, With<Button>)>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, children) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        if interaction == &Interaction::Clicked {
            text.sections[0].value = "Test".to_string();
        }
    }
}

fn spawn_field(
    commands: &mut Commands,
    size: Size,
    background_color: BackgroundColor,
    absolute_position: UiRect,
    text: impl Into<String>,
    text_font: Handle<Font>,
    text_font_size: f32,
    text_font_color: Color,
) {
    commands
        .spawn(ButtonBundle {
            style: Style {
                size,
                // center child text vertically and horizontally
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                position: absolute_position,
                ..default()
            },
            background_color,
            ..default()
        })
        .insert(Name::new("Field"))
        // child text
        .with_children(|parent| {
            parent
                .spawn(TextBundle::from_section(
                    text,
                    TextStyle {
                        font: text_font,
                        font_size: text_font_size,
                        color: text_font_color,
                    },
                ))
                .insert(Name::new("Field Text"));
        });
}

fn setup_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(Name::new("Main Camera"));
}

// fn setup_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let field_color = Color::rgb(0.960784, 0.643137, 0.258823);
//     let field_size = Size::new(Val::Px(FIELD_SIZE), Val::Px(FIELD_SIZE));
//     let field_font = "ComicSansMS3.ttf";
//     let field_font_size = 40.0;
//     let field_font_color = Color::BLACK;
//     
//     for x in 0..3 {
//         for y in 0..3 {
//             spawn_field(
//                 &mut commands,
//                 field_size,
//                 field_color.into(),
//                 UiRect {
//                     left: Val::Px((FIELD_SIZE + FIELD_SIZE / 10.0) * y as f32),
//                     top: Val::Px((FIELD_SIZE + FIELD_SIZE / 10.0) * x as f32),
//                     ..default()
//                 },
//                 "X",
//                 asset_server.load(field_font),
//                 field_font_size,
//                 field_font_color,
//             );
//         }
//     }
// }
