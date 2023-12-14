use bevy::app::{App, Plugin, Update};
use bevy::hierarchy::Children;
use bevy::math::{Quat, Vec3};
use bevy::prelude::{Commands, Component, Entity, Query, Transform, With};

pub struct GeneralPlugin;

impl Plugin for GeneralPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                Update, (
                    fix_model_transforms,
                ),
            )
        ;
    }
}

#[derive(Component)]
pub struct FixChildTransform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

#[derive(Component)]
pub struct NeedsTransformFix;

impl FixChildTransform {
    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }
}

#[derive(Component)]
pub struct ParentEntity(pub Entity);

pub fn fix_model_transforms(
    mut commands: Commands,
    mut scene_instance_query: Query<(Entity, &FixChildTransform, &Children)>,
    mut child_query: Query<&mut Transform, With<NeedsTransformFix>>,
) {
    for (parent, fix_scene_transform, children) in scene_instance_query.iter_mut() {
        for child in children.iter() {
            if let Ok(mut transform) = child_query.get_mut(*child) {
                transform.translation = fix_scene_transform.translation;
                transform.rotation = fix_scene_transform.rotation;
                transform.scale = fix_scene_transform.scale;
                commands.entity(parent).remove::<FixChildTransform>();
                commands.entity(*child).remove::<NeedsTransformFix>();
            }
        }
    }
}

