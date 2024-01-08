// use bevy::prelude::*;
// use bevy_hanabi::prelude::*;

// pub fn make_particle(commands: &mut Commands, effects: &mut ResMut<Assets<EffectAsset>>) -> Handle<EffectAsset> {
//     let mut color_gradient1 = Gradient::new();
//     color_gradient1.add_key(0.0, Vec4::new(4.0, 4.0, 4.0, 1.0));
//     color_gradient1.add_key(0.1, Vec4::new(4.0, 4.0, 0.0, 1.0));
//     color_gradient1.add_key(0.9, Vec4::new(4.0, 0.0, 0.0, 1.0));
//     color_gradient1.add_key(1.0, Vec4::new(4.0, 0.0, 0.0, 0.0));

//     let mut size_gradient1 = Gradient::new();
//     size_gradient1.add_key(0.3, Vec2::new(0.2, 0.02));
//     size_gradient1.add_key(1.0, Vec2::splat(0.0));

//     effects.add(
//         EffectAsset {
//             name: "portal".to_string(),
//             capacity: 32768,
//             spawner: Spawner::rate(5000.0.into()),
//             ..Default::default()
//         }
//         .init(InitPositionCircleModifier {
//             center: Vec3::ZERO,
//             axis: Vec3::Z,
//             radius: 4.,
//             dimension: ShapeDimension::Surface,
//         })
//         .init(InitLifetimeModifier {
//             // Give a bit of variation by randomizing the lifetime per particle
//             lifetime: Value::Uniform((0.6, 1.3)),
//         })
//         .update(LinearDragModifier { drag: 2. })
//         .update(RadialAccelModifier::constant(Vec3::ZERO, -6.0))
//         .update(TangentAccelModifier::constant(Vec3::ZERO, Vec3::Z, 30.))
//         .render(ColorOverLifetimeModifier {
//             gradient: color_gradient1,
//         })
//         .render(SizeOverLifetimeModifier {
//             gradient: size_gradient1,
//         })
//         .render(OrientAlongVelocityModifier),
//     )

// }
