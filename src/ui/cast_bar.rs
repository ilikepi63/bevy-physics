use bevy::prelude::*;

use crate::spells::Casting;

static CAST_BAR_SIZE_IN_PX: f32 = 100.0;

#[derive(Component)]
pub struct CastBar;

#[derive(Component)]
pub struct CastBarInner;

pub fn setup_cast_bar(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    window: Query<&Window>,
) {
    let window = window.single();

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Px(20.0),
                    width: Val::Px(CAST_BAR_SIZE_IN_PX),
                    position_type: PositionType::Absolute,
                    left: Val::Px(window.resolution.width() / 2.0 - CAST_BAR_SIZE_IN_PX / 2.0),
                    top: Val::Percent(75.0),
                    // padding: UiRect::all(Val::Px(10.0)),
                    flex_direction: FlexDirection::Column,
                    // overflow: Overflow::
                    ..Default::default()
                },
                background_color: Color::BLACK.into(),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            CastBar,
        ))
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(0.0),
                        height: Val::Percent(100.0),
                        flex_direction: FlexDirection::Column,
                        // padding: UiRect::all(Val::Px(10.0)),
                        ..Default::default()
                    },
                    background_color: Color::BLUE.into(),
                    ..Default::default()
                },
                CastBarInner,
            ));
        });
}

pub fn update_cast_bar(
    caster: Query<&Casting>,
    mut bar_inner: Query<&mut Style, With<CastBarInner>>,
) {
    for casting in caster.iter() {
        let casting_percent: f32 = (casting.current_duration.as_millis() * 100
            / casting.total_duration.as_millis()) as f32; // should always be between 0 and 100

        for mut bar_inner in &mut bar_inner {
            bar_inner.width = Val::Percent(casting_percent);
        }
    }
}

pub fn update_cast_bar_visible(
    casting: Query<Added<Casting>>,
    mut cast_bar: Query<&mut Visibility, With<CastBar>>,
) {
    for _casting in casting.iter() {
        for mut cast_bar in &mut cast_bar {
            *cast_bar = Visibility::Visible
        }
    }
}

pub fn update_cast_bar_invisible(
    mut removed: RemovedComponents<Casting>,
    mut cast_bar: Query<&mut Visibility, With<CastBar>>,
) {
    for _ in removed.read() {
        for mut cast_bar in &mut cast_bar {
            *cast_bar = Visibility::Hidden
        }
    }
}
