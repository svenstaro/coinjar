use macroquad::prelude::*;
use rapier2d::prelude::*;

struct Jar;
impl Jar {
    fn new() -> Self {
        Self {}
    }
}

#[macroquad::main("coinjar")]
async fn main() {
    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    // Create the ground.
    let collider = ColliderBuilder::cuboid(100.0, 0.1).build();
    collider_set.insert(collider);

    // Create the bouncing ball.
    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.0, 10.0])
        .build();
    let collider = ColliderBuilder::ball(0.5).restitution(0.7).build();
    let ball_body_handle = rigid_body_set.insert(rigid_body);
    collider_set.insert_with_parent(collider, ball_body_handle, &mut rigid_body_set);

    // Create other structures necessary for the simulation.
    let gravity = vector![0.0, -9.81];
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = BroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    let camera = Camera2D {
        zoom: vec2(1.0, 1.0),
        target: vec2(0., 0.),
        ..Default::default()
    };
    set_camera(&camera);

    loop {
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            None,
            &physics_hooks,
            &event_handler,
        );

        let ball_body = &rigid_body_set[ball_body_handle];
        println!("Ball altitude: {}", ball_body.translation().y);

        clear_background(BLACK);

        draw_rectangle(-0.3, -0.2, 0.01, 1., BLUE);
        draw_rectangle(0.3, -0.2, 0.01, 1., YELLOW);
        draw_rectangle(-0.3, 0.8, 0.6, 0.02, GREEN);

        next_frame().await
    }
}
