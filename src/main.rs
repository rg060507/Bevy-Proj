use bevy::prelude::*;

const RECT1_COLOR: Color = Color::srgb(1., 0., 0.);
const RECT2_COLOR: Color = Color::srgb(0., 0., 1.);

const RECT_SPEED: f32 = 500.;

const RECT_SIZE: f32 = 128.;

#[derive(Component)]
struct Rect1;

#[derive(Component)]
struct Rect2;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (check_for_collisions, move_rect1, move_rect2))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(RECT1_COLOR)),
        Transform::from_translation(Vec3::from((0., 0., 1.))).with_scale(Vec3::splat(RECT_SIZE)),
        Rect1,
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(RECT2_COLOR)),
        Transform::from_translation(Vec3::from((200., 0., 0.))).with_scale(Vec3::splat(RECT_SIZE)),
        Rect2,
    ));
}

fn move_rect1(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut transform: Single<&mut Transform, With<Rect1>>,
    time: Res<Time>,
){
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::KeyA){
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::KeyD){
        direction += 1.0;
    }

    transform.translation.x += direction * RECT_SPEED * time.delta_secs();
}

fn move_rect2(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut transform: Single<&mut Transform, With<Rect2>>,
    time: Res<Time>,
){
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft){
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight){
        direction += 1.0;
    }

    transform.translation.x += direction * RECT_SPEED * time.delta_secs();
}
fn check_for_collisions(
    rect1: Single<&mut Transform, With<Rect1>>,
    rect2: Single<&mut Transform, (With<Rect2>, Without<Rect1>)>,) {

    let rect1x = rect1.translation.x;
    let rect2x = rect2.translation.x;
    let rectSize = RECT_SIZE / 2.;

    if rect1x + rectSize > rect2x - rectSize && rect1x - rectSize < rect2x + rectSize {
        println!("collisions detected!");
    }
    else if !(rect1x + rectSize > rect2x - rectSize && rect1x - rectSize < rect2x + rectSize) {
        println!("no collisions detected!");
    }
}