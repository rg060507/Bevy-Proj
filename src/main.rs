use bevy::prelude::*;

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_name, greet_people).chain());
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, HelloPlugin))
        .run();
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("David Tenant".to_string())));
    commands.spawn((Person, Name("Matt Smith".to_string())));
    commands.spawn((Person, Name("Peter Capaldi".to_string())));
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished(){
        for name in &query{
            println!("Hello, {}!", name.0);
        }
    }
}

fn update_name(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query{
        if name.0 == "David Tenant"{
            name.0 = "Dave Tents".to_string();
            break;
        }
    }
}
