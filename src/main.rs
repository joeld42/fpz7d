use bevy::prelude::*;


#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build( &self, app: &mut App ) {
        
        app.insert_resource( GreetTimer( Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems( Startup, add_people )
            .add_systems( Startup, fpz7_setup )
            .add_systems( Update, greet_people )
            ;
    }
}

fn fpz7_setup (
    mut commands : Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // circular base
    commands.spawn( PbrBundle {
        mesh: meshes.add(shape::Circle::new(4.0).into() ),
        material: materials.add( Color::WHITE.into() ),
        transform: Transform::from_rotation(Quat::from_rotation_x( -std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb_u8(124, 144, 255).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

}


fn add_people( mut commands: Commands ) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn greet_people( 
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>, 
    query: Query<&Name, With<Person>> ) {

    if timer.0.tick( time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0 );
        }
    }
}

fn hello_world() {
    println!("hello world...");
}

fn main() {
    App::new()
        .add_plugins( (DefaultPlugins, HelloPlugin) )        
        .run();

}
