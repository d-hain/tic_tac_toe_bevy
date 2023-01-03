use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2., TimerMode::Repeating)))
            // ^^^ Add a GreetTimer that repeats every 2 seconds
            .add_startup_system(add_people) // Startup systems run once on startup (obviously)
            // .add_system(hello_world) // does not have to be executed before greet_people system
            // Systems are executed in parallel when possible
            .add_system(greet_people);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins) // Adds the DefaultPlugins of Bevy
        // Adds an event loop so systems are called every frame
        .add_plugin(HelloPlugin)
        .run();
}

// /// System that prints "hello world!"
// fn hello_world() {
//     println!("hello world!");
// }

/// System that spawns 3 Persons with a Name
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("David Hain".to_string())));
    commands.spawn((Person, Name("Eren Jäger".to_string())));
    commands.spawn((Person, Name("The Primeagen".to_string())));
}

/// System that searches all Names with the Person component
/// (all Persons with a Name)
/// Prints "hello NAME!" for every Person with a Name
fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>
) {
    // only greets people when the timer has run out
    // just_finished() only sends true on the tick the timer resets
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }
    }
}