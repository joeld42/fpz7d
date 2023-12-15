//use std::fmt::Debug;

use rand::Rng;
use bevy::{
    prelude::*, 
    window::CursorGrabMode,
    app::{AppExit, ScheduleRunnerPlugin},
    input::mouse::MouseMotion,
    gltf::{Gltf, GltfMesh}, 
    transform::commands, pbr::NotShadowCaster
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
            .insert_resource(ClearColor(Color::rgb(0.3764, 0.47451, 0.6314 )))
            .init_resource::<DebugTools>()
            .init_resource::<GameState>()            
            .add_systems( Startup, (fpz7_setup, map_setup) )
            //.add_systems( Update, test_blarg )          
            .add_systems( Update, toggle_debug_camera )            
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
    ent_lamp: Option<Entity>,
    ent_fps_camera: Option<Entity>,
    //ent: Option<Entity>,
}


fn fpz7_setup (
    asset_server: Res<AssetServer>,
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
    // commands.spawn( PbrBundle {
    //     mesh: meshes.add(shape::Circle::new(4.0).into() ),
    //     material: materials.add( Color::WHITE.into() ),
    //     transform: Transform::from_rotation(Quat::from_rotation_x( -std::f32::consts::FRAC_PI_2)),
    //     ..default()
    // });

    // Kick off the gltf asset load
    // let gltf = asset_server.load("fpz7d.glb");
    // commands.insert_resource(ZSceneAssets(gltf) );

    // gamestate and player
    game.ent_player = Some(
        // commands.spawn(
        //         Transform::from_xyz( 0.0, 0.0, 0.0)
        //         ).id() );
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb_u8(250, 60, 60).into()),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),                
                ..default()
            }, NotShadowCaster ) ).id() );
                

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb_u8(124, 144, 255).into()),
        transform: Transform::from_xyz(1.0, 0.5, 2.0),
        ..default()
    });

    // light
    game.ent_lamp = Some( commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    }).id() );

    // Camera
    game.ent_fps_camera = Some( commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz( 3.0, 1.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        projection: Projection::Perspective ( PerspectiveProjection { fov: std::f32::consts::PI / 2.0,  ..default() } ),        
        ..default()
    }).id() );


    // Debug Camera
    commands.spawn ( (Camera3dBundle {
        camera: Camera {
            is_active: false,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 30.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),        
        ..default()
        },
        DebugCamera,
    ) );

}


// notes:
//  zelda room size 16x11 tiles
// world map is 258x88 tiles (16x8 rooms)

fn map_setup (    
    asset_server: Res<AssetServer>,
    mut commands : Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    ) {

        // commands.spawn( SceneBundle {
        //     scene: asset_server.load("fpz7d.glb#Scene0"),
        //     ..default()
        // });

        let canyon_scene = asset_server.load("CanyonChunk.glb#Scene0");

        let ground_scene = asset_server.load("Ground.glb#Scene0");

        // commands.spawn( SceneBundle {
        //     scene: canyon_scene,
        //     transform: Transform::from_xyz( 10.0, 0.0, 10.0 ),
        //     ..default()
        // });

        for room_j in 0..8 {
            for room_i in 0..16 {

                //let secret_number = rand::thread_rng().gen_range(1..=100);
                let mut rng = rand::thread_rng();
                let room_color = Color::rgb_linear(
                    rng.gen(), rng.gen(), rng.gen() );                

                // ground plane for room                
                // commands.spawn(PbrBundle {
                //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                //     material: materials.add(room_color.into()),
                //     transform: Transform::from_xyz(((room_i * 16) - (8*16)) as f32, hite, (room_j * 11) as f32 + 5.5 )
                //             .with_scale( Vec3 { x:16.0, y:0.1, z:11.0 }),
                    
                //     ..default()
                // });

                let room_origin = Vec3 { x:((room_i * 16) - (8*16)) as f32, 
                    y: 0.0, 
                    z: (room_j * 11) as f32 + 5.0 };

                commands.spawn( SceneBundle {
                    scene: ground_scene.clone(),
                    transform: Transform { translation: room_origin, 
                        ..default() },
                    ..default()
                } );

                // test tiles
                //if (room_j==0) && (room_i==8) {                                                        

                  //  println!("Room Origin is {}", room_origin );

                    for tile_j in 0..11 {
                        for tile_i in 0..16 {

                            // chance to spawn a tile
                            if rng.gen::<f32>() < 0.1f32 {

                                let rot : f32 = rng.gen::<f32>() * std::f32::consts::PI * 2.0;

                                let tile_origin = Vec3 { x: ((tile_i as f32) - (16.0/2.0)) + room_origin.x + 0.5, 
                                    y: 0.0, 
                                    z: tile_j as f32 + room_origin.z - 5.0 };

                                let tx = tile_origin.x + (tile_i as f32 - 8.0);
                                //println!("Tile {} {} tx is {}", tile_i, tile_j, tile_origin );

                                    commands.spawn( SceneBundle {
                                        scene: canyon_scene.clone(),
                                        transform: Transform { 
                                            translation: tile_origin,
                                            rotation: Quat::from_rotation_y( rot ),
                                            ..default()
                                          },
                                        ..default()
                                    });

                            }

                        }
                    }
                //}

            }
        }
}


/*
fn add_people( mut commands: Commands ) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}
*/

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
    if input.just_pressed( KeyCode::Escape ) {
        app_exit_events.send(AppExit);
    }

}

fn player_controller(
    time: Res<Time>,
    game: Res<GameState>,
    dbg: Res<DebugTools>,
    keyboard_input: Res<Input<KeyCode>>,
    mut transforms: Query<&mut Transform>,
    ) 
    {
        let move_speed = 2f32;
        let mut move_x : f32 = 0.0;
        let mut move_y : f32 = 0.0;
        
        if keyboard_input.pressed( KeyCode::W ) ||
            keyboard_input.pressed( KeyCode::Up ) {
                move_y = move_y - move_speed;
            }

        if keyboard_input.pressed( KeyCode::S ) ||
            keyboard_input.pressed( KeyCode::Down ) {
                move_y = move_y + move_speed;
            }

        if keyboard_input.pressed( KeyCode::A ) ||
            keyboard_input.pressed( KeyCode::Left ) {
                move_x = move_x - move_speed;
            }

        if keyboard_input.pressed( KeyCode::D ) ||
            keyboard_input.pressed( KeyCode::Right ) {
                move_x = move_x + move_speed;
            }
        

        let curr = transforms.get( game.ent_player.unwrap() ).unwrap().clone();
        let curr_cam = transforms.get( game.ent_fps_camera.unwrap() ).unwrap().clone();

        if keyboard_input.just_pressed( KeyCode::E ) {
            println!("Player Pos is {}", curr_cam.translation );
        }
        
        let cam_fwd = curr_cam.rotation * Vec3::Z;

        // Snap to ground for now (don't normalize this so we move slower when looking down)
        let cam_fwd = Vec3 { x: cam_fwd.x, y: 0.0,  z:cam_fwd.z };

        let move_dir_fwd = cam_fwd;
        let move_dir_right = Vec3::cross( Vec3::Y, cam_fwd ).normalize();

            

        let upd_pos = curr.translation + 
            (move_dir_fwd * move_y * time.delta_seconds() ) +
            (move_dir_right * move_x * time.delta_seconds());
        
        // TODO: check for collision        
        
        // let mut player_transform = transforms.get_mut(game.ent_player.unwrap()).unwrap();
        // player_transform.translation = upd_pos;
        // player_transform.rotation = curr.rotation;
        

        let mut player_transform = transforms.get_mut(game.ent_player.unwrap()).unwrap();
        //let player_offs = if dbg.dbg_cam { Vec3::ZERO } else { Vec3 { x : 0.0, y : -3.0, z : 0.0 } };
        player_transform.translation = upd_pos;
        player_transform.rotation = curr.rotation;

        let mut lamp_transform = transforms.get_mut(game.ent_lamp.unwrap()).unwrap();
        let lamp_offset = Vec3 { x : -2.0, y : 3.5, z : 0.0 };
        lamp_transform.translation = upd_pos + lamp_offset;        
        
        let mut fps_camera_transform = transforms.get_mut(game.ent_fps_camera.unwrap()).unwrap();
        let fps_cam_offset = Vec3 { x : 0.0, y : 0.4, z : 0.0 };
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
    let move_ang = 16.0 * (std::f32::consts::PI / 180.0 );

    for ev in motion_evr.iter() {
        //println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);

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

/*
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
 */

fn main() {
    App::new()
        .add_plugins( (DefaultPlugins, HelloPlugin) )        
        .run();

}
