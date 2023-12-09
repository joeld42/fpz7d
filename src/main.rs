//use std::fmt::Debug;

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
            .init_resource::<DebugTools>()
            .add_systems( Startup, add_people )
            .add_systems( Startup, fpz7_setup )
            .add_systems( Update, toggle_debug_camera )
            .add_systems( Update, greet_people )
            ;
    }
}

#[derive(Component)]
struct DebugCamera;

#[derive(Resource,Default)]
struct DebugTools {
    dbg_cam : bool,
}


fn fpz7_setup (
    mut commands : Commands,
    mut dbg: ResMut<DebugTools>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // set up debug tools    
    println!("will setup debug tools");
    dbg.dbg_cam = false;
    println!("Setup debug tools done");

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


    // Debug Camera
    commands.spawn ( (Camera3dBundle {
        camera: Camera {
            is_active: false,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 1.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),        
        ..default()
        },
        DebugCamera,
    ) );

}


fn add_people( mut commands: Commands ) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

fn toggle_debug_camera( 
    mut dbg: ResMut<DebugTools>,
    //query: Query<&Camera, Option<&DebugCamera>>,
    mut q_dbg_cam: Query<&mut Camera, With<DebugCamera> >,
    mut q_regular_cam: Query<&mut Camera, Without<DebugCamera> >,
    input: Res<Input<KeyCode>>,
    ) {
    let toggle_debug = input.just_pressed(KeyCode::Tab);

    if toggle_debug {
        
        let use_dbg_cam: bool = !dbg.dbg_cam;
        dbg.dbg_cam = use_dbg_cam;

        for mut cam in &mut q_dbg_cam {        
            cam.is_active = use_dbg_cam;
        }
        for mut cam in &mut q_regular_cam {        
            cam.is_active = !use_dbg_cam;
        }
        
    }

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

fn main() {
    App::new()
        .add_plugins( (DefaultPlugins, HelloPlugin) )        
        .run();

}
