//use std::fmt::Debug;

use bevy::{
    prelude::*, 
    window::CursorGrabMode,
    app::{AppExit, ScheduleRunnerPlugin},
    input::mouse::MouseMotion,
};




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
            .init_resource::<GameState>()
            .add_systems( Startup, add_people )
            .add_systems( Startup, fpz7_setup )
            .add_systems( Update, toggle_debug_camera )
            .add_systems( Update, greet_people )
            .add_systems( Update, player_controller )
            .add_systems( Update, mouse_look )
            ;
    }
}

#[derive(Component)]
struct DebugCamera;

#[derive(Resource,Default)]
struct DebugTools {
    dbg_cam : bool,
}

#[derive(Resource,Default)]
struct GameState {
    ent_player: Option<Entity>,
    ent_fps_camera: Option<Entity>,
}


fn fpz7_setup (
    mut commands : Commands,
    mut windows: Query<&mut Window>,
    mut dbg: ResMut<DebugTools>,
    mut game: ResMut<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // set up debug tools    
    dbg.dbg_cam = false;

    // grab mouse
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;

    // circular base
    commands.spawn( PbrBundle {
        mesh: meshes.add(shape::Circle::new(4.0).into() ),
        material: materials.add( Color::WHITE.into() ),
        transform: Transform::from_rotation(Quat::from_rotation_x( -std::f32::consts::FRAC_PI_2)),
        ..default()
    });

    // gamestate and player
    game.ent_player = Some(
        // commands.spawn(
        //         Transform::from_xyz( 0.0, 0.0, 0.0)
        //         ).id() );
        commands.spawn(
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb_u8(250, 60, 60).into()),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            }).id() );
                

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb_u8(124, 144, 255).into()),
        transform: Transform::from_xyz(1.0, 0.5, 2.0),
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
    game.ent_fps_camera = Some( commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),

        ..default()
    }).id() );


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
    mut windows: Query<&mut Window>,
    mut dbg: ResMut<DebugTools>,
    //query: Query<&Camera, Option<&DebugCamera>>,
    mut q_dbg_cam: Query<&mut Camera, With<DebugCamera> >,
    mut q_regular_cam: Query<&mut Camera, Without<DebugCamera> >,
    input: Res<Input<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
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

        // mouse grab
        let mut window = windows.single_mut();
        if use_dbg_cam {
            //mouse grab off in debug mode
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;
        } else {
            window.cursor.visible = false;
            window.cursor.grab_mode = CursorGrabMode::Locked;
        }
        
    }

    // Check for app exit
    if (input.just_pressed( KeyCode::Escape )) {        
        app_exit_events.send(AppExit);
    }

}

fn player_controller(
    time: Res<Time>,
    game: Res<GameState>,
    keyboard_input: Res<Input<KeyCode>>,
    mut transforms: Query<&mut Transform>,
    ) 
    {
        let move_speed = 10f32;
        let mut move_x : f32 = 0.0;
        let mut move_y : f32 = 0.0;
        
        if keyboard_input.pressed( KeyCode::W ) ||
            keyboard_input.pressed( KeyCode::Up ) {
                move_y = move_y + move_speed;
            }

        if keyboard_input.pressed( KeyCode::S ) ||
            keyboard_input.pressed( KeyCode::Down ) {
                move_y = move_y - move_speed;
            }

        if keyboard_input.pressed( KeyCode::A ) ||
            keyboard_input.pressed( KeyCode::Left ) {
                move_x = move_x - move_speed;
            }

        if keyboard_input.pressed( KeyCode::D ) ||
            keyboard_input.pressed( KeyCode::Right ) {
                move_x = move_x + move_speed;
            }

        let move_dir_fwd = Vec3::Z;
        let move_dir_right = Vec3::X;

        let curr = transforms.get( game.ent_player.unwrap() ).unwrap().clone();
        

        let upd_pos = curr.translation + 
            (move_dir_fwd * move_y * time.delta_seconds() ) +
            (move_dir_right * move_x * time.delta_seconds());
        
        // TODO: check for collision        
        
        // let mut player_transform = transforms.get_mut(game.ent_player.unwrap()).unwrap();
        // player_transform.translation = upd_pos;
        // player_transform.rotation = curr.rotation;
        

        let mut player_transform = transforms.get_mut(game.ent_player.unwrap()).unwrap();
        player_transform.translation = upd_pos;
        player_transform.rotation = curr.rotation;
        
        let mut fps_camera_transform = transforms.get_mut(game.ent_fps_camera.unwrap()).unwrap();
        let fps_cam_offset = Vec3 { x : 0.0, y : 1.5, z : 0.0 };
        fps_camera_transform.translation = upd_pos + fps_cam_offset;
        //fps_camera_transform.rotation = curr.rotation;



    }

fn mouse_look (
    time: Res<Time>,
    game: Res<GameState>,
    mut motion_evr: EventReader<MouseMotion>,
    mut transforms: Query<&mut Transform>,
) {
    let lookstr = time.delta_seconds();
    let move_ang = 4.0 * (3.1416 / 180.0 );

    for ev in motion_evr.iter() {
        println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);

        let rot_x = -ev.delta.x * lookstr;
        let rot_y = -ev.delta.y * lookstr;

        //let curr = transforms.get( game.ent_fps_camera.unwrap() ).unwrap().clone();

        let mut fps_camera_transform = transforms.get_mut(game.ent_fps_camera.unwrap()).unwrap();

        let xrot = Quat::from_axis_angle( Vec3::Y,  rot_x * move_ang );
        let yrot = Quat::from_axis_angle( Vec3::X,  rot_y * move_ang );

        fps_camera_transform.rotation =  xrot * fps_camera_transform.rotation;
        fps_camera_transform.rotation =  fps_camera_transform.rotation * yrot;
        

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
