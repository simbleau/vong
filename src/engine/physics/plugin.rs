use crate::engine::lyon_utils::{self, usvg_draw, Convert};
use bevy::{prelude::*, render::mesh::VertexAttributeValues};
use bevy_rapier2d::prelude::*;
use bevy_vello::{
    vello_svg::usvg::{self, TreeParsing},
    VelloVector,
};
use lyon_tessellation::{FillTessellator, StrokeTessellator};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(
            100.0,
        ));
        app.add_plugins(RapierDebugRenderPlugin::default().disabled());
        app.add_systems(
            Update,
            (create_vello_colliders, show_debug_visualizations),
        );
    }
}

/// A tag which marks the object to receive a physical collider.
#[derive(Component)]
pub struct PhysicsTag;

/// Create colliders for vello vectors without them.
fn create_vello_colliders(
    mut commands: Commands,
    // A query for Vello vectors tagged with the [`PhysicsTag`] but do not have
    // a [`Collider`]
    query: Query<
        (Entity, &Handle<VelloVector>),
        (With<PhysicsTag>, Without<Collider>),
    >,
    vectors: Res<Assets<VelloVector>>,
) {
    for (e, vec_handle) in query.iter() {
        let Some(vector) = vectors.get(vec_handle) else {
            return;
        };
        const EGG: &str = include_str!("../../../assets/egg.svg");
        const BACON: &str = include_str!("../../../assets/bacon.svg");
        let tree = match vec_handle
            .path()
            .map(|ap| ap.path().to_str().unwrap())
            .unwrap()
        {
            "egg.svg" => {
                usvg::Tree::from_str(EGG, &usvg::Options::default()).unwrap()
            }
            "bacon.svg" => {
                usvg::Tree::from_str(BACON, &usvg::Options::default()).unwrap()
            }
            e => panic!("{e:?}"),
        };
        let lyon_svg = usvg_draw::Svg::from_tree(&tree);
        let tessellation_mesh_buffer = lyon_utils::generate_buffer(
            &lyon_svg,
            &mut FillTessellator::new(),
            &mut StrokeTessellator::new(),
        );
        let tessellation_mesh: Mesh = tessellation_mesh_buffer.convert();
        if let Some(VertexAttributeValues::Float32x3(positions)) =
            tessellation_mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        {
            let points: Vec<Vec2> = positions
                .iter()
                .map(|[x, y, _]| {
                    // Adjust for center origin
                    Vec2::new(*x - vector.width / 2.0, *y + vector.height / 2.0)
                })
                .collect();

            let collider = Collider::convex_hull(points.as_slice()).unwrap();
            commands.entity(e).insert(collider);
        }
    }
}
pub fn show_debug_visualizations(
    keyboard_input: Res<Input<KeyCode>>,
    mut query_physics: ResMut<DebugRenderContext>,
    mut query: Query<&mut bevy_vello::debug::DebugVisualizations>,
) {
    if keyboard_input.just_pressed(KeyCode::Q) {
        query_physics.enabled = !query_physics.enabled;
        for mut flag in query.iter_mut() {
            *flag = match *flag {
                bevy_vello::debug::DebugVisualizations::Hidden => {
                    bevy_vello::debug::DebugVisualizations::Visible
                }
                bevy_vello::debug::DebugVisualizations::Visible => {
                    bevy_vello::debug::DebugVisualizations::Hidden
                }
            };
        }
    }
}
