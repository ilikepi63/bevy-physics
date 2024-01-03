use bevy::prelude::*;

use crate::health_bars::PrimaryCamera;

use self::{
    action_bar::setup_action_bar,
    cast_bar::{setup_cast_bar, update_cast_bar, update_cast_bar_visible, update_cast_bar_invisible},
    tooltip::{mouseover_system, setup_tooltip, tooltip_events, ShowsTooltip, TooltipState},
};

mod action_bar;
mod cast_bar;
mod tooltip;

static BUTTON_SIZE: f32 = 30.0;

#[derive(Component)]
pub struct RootNode;

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {

    // root node
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::End,
                    ..default()
                },
                ..default()
            },
            RootNode,
        ))
        .with_children(|parent| setup_action_bar(parent, &asset_server));

    // setup_cast_bar(&mut commands, &asset_server);

    setup_tooltip(&mut commands, &asset_server);
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        // app.init_resource::<TooltipState>();
        app.add_event::<TooltipState>();
        app.add_startup_system(setup_ui);
        app.add_startup_system(setup_cast_bar);
        app.add_system(tooltip_events);
        app.add_system(mouseover_system);
        app.add_system(update_cast_bar);
        app.add_system(update_cast_bar_visible);
        app.add_system(update_cast_bar_invisible);
    }
}
