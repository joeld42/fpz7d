//use std::fmt::Debug;

//use std::cmp;
use std::fs::File;
use std::io::{ self, BufRead, BufReader };

use rand::Rng;
use bevy::{
    prelude::*, 
    window::CursorGrabMode,
    app::{AppExit, ScheduleRunnerPlugin},
    input::mouse::MouseMotion,
    gltf::{Gltf, GltfMesh},     
    transform::commands, pbr::NotShadowCaster
};
use bevy_easings::Lerp;

const STAB_TIME: f32 = 0.5;

const STAB_EXTENT_A : f32 = 0.5; // local z when not stab
const STAB_EXTENT_B : f32 = -0.2; // local z at max stab extent

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Resource)]
struct RawTilemapData(Vec<RawTileData>);

// What object the tile represents.
// It might only be part of the object like
// for the big tree or canyon walls
#[repr(u8)]
#[derive(Clone,Copy)]
enum TileObject {
    EmptyGround,
    RoughGround,  // Sandy tiles
    Canyon,
    Rock, // or is it a bush?
    Tree,
    BigTree,
    Planks,
    Stairs,
    Statue,
    Tombstone,

    Unknown, // Something we don't support yet
}

#[repr(u8)]
#[derive(Clone,Copy)]
enum TileBiome  {
    Desert,
    Grasslands,
}

#[derive(Clone,Copy)]
struct RawTileData 
{
    code : u8,
    blocked : bool,
    tile : TileObject,
    biome : TileBiome
}

pub struct FPZ7DGamePlugin
{
    hello : u32,
    raw_tiles : Vec<RawTileData>,    
}

impl Plugin for FPZ7DGamePlugin {    

    fn build( &self, app: &mut App ) {

        println!( "From Build, hello is {}, raw tiles count is {}", self.hello, self.raw_tiles.len() );
        
        app.insert_resource( GreetTimer( Timer::from_seconds(2.0, TimerMode::Repeating)))
            .insert_resource(ClearColor(Color::rgb(0.3764, 0.47451, 0.6314 )))
            .insert_resource( RawTilemapData( self.raw_tiles.clone() ) )
            .init_resource::<DebugTools>()
            .init_resource::<GameState>()
            .add_systems( Startup, (fpz7_setup, map_setup) )            
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
    ent_sword: Option<Entity>,
    ent_lamp: Option<Entity>,
    ent_fps_camera: Option<Entity>,    
}

#[derive(Component, Default)]
struct AttackState {
    stabby_amt : f32, // set to 1.0 to play stab animation, will cool down to 0
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

    let sword_scene = asset_server.load("MasterSword.glb#Scene0");

    // gamestate and player
    game.ent_player = Some(        
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb_u8(250, 60, 60).into()),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),                
                ..default()
            }, 
            NotShadowCaster,
            AttackState { ..default() },
        ) ).id() );
           
    // Player's sword
    game.ent_sword = Some(
        commands.spawn( SceneBundle {
            scene: sword_scene.clone(),
            transform: Transform { 
                translation: Vec3 { x: 0.0, y : 0.0, z : STAB_EXTENT_A }, 
                rotation: Quat::from_rotation_x( -std::f32::consts::PI / 2.0 ),
                 ..default() },
            ..default()
        }  ).id() );

    // parent sword to player
    commands.entity(game.ent_player.unwrap()).push_children(&[game.ent_sword.unwrap()]);
    
    
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
        transform: Transform::from_xyz(0.0, 180.0, 100.0).looking_at(Vec3 { x: 0.0, y: 0.0, z: 40.0}, Vec3::Y),        
        ..default()
        },
        DebugCamera,
    ) );

}


// notes:
//  zelda room size 16x11 tiles
// world map is 256x88 tiles (16x8 rooms)

fn map_setup (    
    asset_server: Res<AssetServer>,
    raw_map : Res<RawTilemapData>,
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
        
        let raw_map = &raw_map.0;
        // let t : RawTileData = raw_map[10];
        // println!("Tile t is {}", t.blocked );

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

                let room_origin = Vec3 { x:(((room_i * 16) as f32) - ((8*16) as f32)), 
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
                  let tile_i = 0;
                  let tile_j = 0;
                  let raw_ndx = (room_j + tile_j) * (16*16) + (room_i * 16) + tile_i;
                  println!("Room {} {} raw_ndx {}", room_i, room_j, raw_ndx  );

                    for tile_j in 0..11 {
                        for tile_i in 0..16 {

                            let raw_ndx = ((room_j * 11) + tile_j) * (16*16) + (room_i * 16) + tile_i;
                            if (raw_ndx >= raw_map.len()) {
                                 println!("Bad index, room {} {} tile {} {} ndx {}",
                                         room_i, room_j, tile_i, tile_j, raw_ndx );
                                         continue;
                             }

                            let tile : RawTileData = raw_map[ raw_ndx ];                            

                            // let t : RawTileData = raw_map[10];

                            // chance to spawn a tile
                            //if rng.gen::<f32>() < 0.1f32 {
                            if tile.blocked {

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
    mouse_buttons: Res<Input<MouseButton>>,
    mut transforms: Query<&mut Transform>,
    mut attack: Query<&mut AttackState>,
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
        
        // Apply attack
        let mut attack = attack.get_mut( game.ent_player.unwrap() ).unwrap();                    
        if mouse_buttons.just_pressed( MouseButton::Left ) && (!dbg.dbg_cam) {

            // stab
            attack.stabby_amt = STAB_TIME;
        } else {
            attack.stabby_amt = f32::max( 0.0, attack.stabby_amt - time.delta_seconds() );
        }
        
        let stab_t = attack.stabby_amt / STAB_TIME;        


        let curr = transforms.get( game.ent_player.unwrap() ).unwrap().clone();
        let curr_cam = transforms.get( game.ent_fps_camera.unwrap() ).unwrap().clone();

        if keyboard_input.just_pressed( KeyCode::E ) {
            println!("Player Pos is {}", curr_cam.translation );
        }
        
        let cam_fwd = curr_cam.rotation * Vec3::Z;

        // Snap to ground for now (don't normalize this so we move slower when looking down)
        let cam_fwd = Vec3 { x: cam_fwd.x, y: 0.0,  z:cam_fwd.z };

        let move_dir_fwd = cam_fwd.normalize();
        let move_dir_right = Vec3::cross( Vec3::Y, move_dir_fwd ).normalize();

        // orthornormalize (this should be UP for now, but might need to change if we support slopes)
        let move_dir_up = Vec3::cross( move_dir_fwd, move_dir_right ).normalize();
        let M_facing_rot = Mat3 { x_axis : move_dir_right, y_axis: move_dir_up, z_axis: move_dir_fwd };
        let facing_rot : Quat = Quat::from_mat3( &M_facing_rot );

            
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
        //player_transform.rotation = curr.rotation;
        player_transform.rotation = facing_rot;

        let mut sword_transform = transforms.get_mut(game.ent_sword.unwrap()).unwrap();

        // FIXME: figure out how to call Lerp()
        let stab_lerp = ((1.0 -stab_t) * STAB_EXTENT_A) + (stab_t * STAB_EXTENT_B);
        sword_transform.translation = Vec3 { x : 0.0, y : 0.0, z : stab_lerp };

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

fn load_overworld( ) -> io::Result<Vec<RawTileData>>
{
    let mut raw_tiles = Vec::<RawTileData>::with_capacity( 256*88 );    

    // Read the overworld tiles
    let fp = File::open( "assets/nes_zelda_overworld_tile_map.txt")?;
    let reader = BufReader::new(fp);    

    for line in reader.lines() {
        for tile_index_str in line.expect("Failed to read line")
                .split_whitespace()
                .map( |s| s.to_string() ) {
                    let tile_index = i64::from_str_radix( tile_index_str.as_str(), 16 ).unwrap();

                    let tile = match tile_index {
                        0x02 | 0x0e => TileObject::EmptyGround,
                        0x01 | 0x28 | 0x29 | 0x2a => TileObject::Canyon,
                        _ => TileObject::Unknown
                    };

                    raw_tiles.push( RawTileData { 
                            code: tile_index as u8, 
                            blocked : false, 
                            tile: tile, 
                            biome: TileBiome::Desert })
                }            
    }

    // Now read the blocking tiles
    let fp = File::open( "assets/nes_zelda_overworld_blocking_map.txt")?;
    //let fp = File::open( "assets/nes_zelda_overworld_blockTEST_map.txt")?;
    let reader = BufReader::new(fp);    

    let mut ndx = 0;
    for line in reader.lines() {
        for tile_blocking_str in line.expect("Failed to read line").chars() {            
            
            raw_tiles[ndx].blocked = match tile_blocking_str {
                'X' => true,
                '.' => false,
                _ => { println!("Unexpected '{}'", tile_blocking_str); false }
            };
            ndx += 1;            
        }
        
    }

    println!("Map read {} tiles", raw_tiles.len() );

    Ok(raw_tiles)
}

fn main() {

    let game_plugin = FPZ7DGamePlugin { 
        hello : 42,
        raw_tiles : load_overworld().unwrap(),
     };

    App::new()
        .add_plugins( (DefaultPlugins, game_plugin ) )        
        .run();

}
