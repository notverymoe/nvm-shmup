// Copyright 2024 Natalie Baker // AGPLv3 //

use bevy::{
    app::PluginGroupBuilder, 
    core_pipeline::{
        core_3d::graph::Core3d, 
        tonemapping::{DebandDither, Tonemapping}
    }, 
    math::AspectRatio, 
    prelude::*, 
    render::{
        camera::{camera_system, CameraMainTextureUsages, CameraProjection, CameraProjectionPlugin, CameraRenderGraph, CameraUpdateSystem, Exposure, ScalingMode}, 
        primitives::Frustum, view::{check_visibility, update_frusta, ColorGrading, VisibilitySystems, VisibleEntities}
    }, 
    transform::TransformSystem
};

struct PluginGameCamera;

impl Plugin for PluginGameCamera {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate, 
            (
                update_game_camera
                    .before(TransformSystem::TransformPropagate)
                    .after(CameraUpdateSystem),
                update_frusta::<ProjectionGame>
                    .in_set(VisibilitySystems::UpdateProjectionFrusta)
                    .after(camera_system::<ProjectionGame>)
                    .after(TransformSystem::TransformPropagate)
                    .before(check_visibility),
            )
        );
    }
}

pub struct PluginsGameCamera;

impl PluginGroup for PluginsGameCamera {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PluginGameCamera)
            .add(CameraProjectionPlugin::<ProjectionGame>::default())
    }
}

pub fn update_game_camera(mut q_cameras: Query<(&mut Transform, &ProjectionGame)>) {
    q_cameras.iter_mut().for_each(|(mut transform, projection)| {
        transform.translation.z = -projection.calculate().0;
    });
}

#[derive(Bundle)]
pub struct GameCameraBundle {
    pub camera:              Camera,
    pub camera_render_graph: CameraRenderGraph,
    pub projection:          ProjectionGame,
    pub visible_entities:    VisibleEntities,
    pub frustum:             Frustum,
    pub transform:           Transform,
    pub global_transform:    GlobalTransform,
    pub camera_3d:           Camera3d,
    pub tonemapping:         Tonemapping,
    pub deband_dither:       DebandDither,
    pub color_grading:       ColorGrading,
    pub exposure:            Exposure,
    pub main_texture_usages: CameraMainTextureUsages,
}

impl Default for GameCameraBundle {
    fn default() -> Self {
        Self{
            camera_render_graph: CameraRenderGraph::new(Core3d),
            camera:              Camera::default(),
            projection:          ProjectionGame::default(),
            visible_entities:    VisibleEntities::default(),
            frustum:             Frustum::default(),
            transform:           Transform::default(),
            global_transform:    GlobalTransform::default(),
            camera_3d:           Camera3d::default(),
            tonemapping:         Tonemapping::default(),
            color_grading:       ColorGrading::default(),
            exposure:            Exposure::default(),
            main_texture_usages: CameraMainTextureUsages::default(),
            deband_dither:       DebandDither::Enabled,
        }
    }

}

#[derive(Debug, Clone, Copy, Reflect)]
pub struct Plane {
    pub distance: f32,
    pub size:     f32,
}

impl Plane {
    
    #[must_use]
    pub const fn new(distance: f32, size: f32) -> Self {
        Self{distance, size}
    }

}

#[derive(Debug, Clone, Copy, Component, Reflect)]
#[reflect(Component, Default)]
pub struct ProjectionGame {
    pub aspect_ratio: f32,
    pub planes_a: Plane,
    pub planes_b: Plane,
    pub near: f32,
    pub far:  f32,
}

impl Default for ProjectionGame {
    fn default() -> Self {
        Self { 
            aspect_ratio: 1.0,
            planes_a: Plane::new(1.0, 100.0), 
            planes_b: Plane::new(2.0, 200.0), 
            near: 0.1,
            far: 1000.0,
        }
    }
}

impl ProjectionGame {
    #[must_use]
    pub fn calculate(&self) -> (f32, Projection) {
        calculate_frustrum_from_planes(self.aspect_ratio, self.planes_a, self.planes_b, self.near, self.far)
    }
}

impl CameraProjection for ProjectionGame {
    fn get_projection_matrix(&self) -> Mat4 {
        self.calculate().1.get_projection_matrix()
    }

    fn update(&mut self, width: f32, height: f32) {
        self.aspect_ratio = AspectRatio::new(width, height).into();
    }

    fn far(&self) -> f32 {
        self.far
    }

    fn get_frustum_corners(&self, z_near: f32, z_far: f32) -> [bevy::math::Vec3A; 8] {
        self.calculate().1.get_frustum_corners(z_near, z_far)
    }
}

#[must_use]
pub fn calculate_frustrum_from_planes(
    aspect_ratio: f32,
    plane_a: Plane,
    plane_b: Plane,
    near: f32,
    far:  f32,
) -> (f32, Projection) {

    let dir = Vec3::new(
        0.5*plane_b.size*aspect_ratio - 0.5*plane_a.size*aspect_ratio,
        0.5*plane_b.size              - 0.5*plane_a.size,
        plane_b.distance              - plane_a.distance,
    ).normalize();
    assert!(dir.x >= 0.0 && dir.y >= 0.0);

    if dir.x == 0.0 && dir.y == 0.0 {
        let right = 0.5*plane_a.size*aspect_ratio;
        let top   = 0.5*plane_a.size;

        (0.0, Projection::Orthographic(OrthographicProjection{
            near,
            far,
            viewport_origin: Vec2::new(0.5, 0.5),
            scaling_mode: ScalingMode::WindowSize(1.0),
            area: Rect::from_center_half_size(Vec2::ZERO, Vec2::new(right, top)),
            scale: 1.0
        }))
    } else {

        let fov = Vec3::new(dir.x, 0.0, dir.z).normalize().angle_between(dir);
        let projection = PerspectiveProjection{fov, aspect_ratio, near, far};

        // Calculate how far forward/backward a camera would need to be 
        // for the projection to line up plane_a and plane_b at their 
        // correct distances. ie. the frustrum defined by the planes may
        // have an origin behind or infront or behind the zero-origin.
        let n_x  = plane_a.size/dir.x;
        let dist = n_x*dir.z;
        let diff = plane_a.distance - dist;

        (diff, Projection::Perspective(projection))
    
        /*
        // TODO should I just calculate FOV?
        //let n_z   = near/dir.z;
        //let right = n_z*dir.x;
        //let top   = n_z*dir.y;

        // Terms A/B cancel out to 0 because view is symmetric
        let c   =      (far+near)/(far-near);
        let d   = -(2.0*far*near)/(far-near);
        let m00 = near/right; // Terms simplify to (2.0*near)/(2.0*right), since view is symmetric
        let m11 = near/top;   // Terms simplify to (2.0*near)/(2.0*top  ), since view is symmetric
    
        let projection = Mat4::from_cols(
            Vec4::new(m00, 0.0, 0.0,  0.0),
            Vec4::new(0.0, m11, 0.0,  0.0),
            Vec4::new(0.0, 0.0,   c, -1.0),
            Vec4::new(0.0, 0.0,   d,  0.0),
        );

        // Calculate how far forward/backward a camera would need to be 
        // for the projection to line up plane_a and plane_b at their 
        // correct distances. ie. the frustrum defined by the planes may
        // have an origin behind or infront or behind the zero-origin.
        let n_x  = plane_a.size/dir.x;
        let dist = n_x*dir.z;
        let diff = plane_a.distance - dist;

        (diff, projection)
        */
    }

}
