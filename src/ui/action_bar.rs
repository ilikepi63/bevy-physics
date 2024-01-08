use bevy::prelude::*;

use super::{tooltip::ShowsTooltip, BUTTON_SIZE};

fn spawn_action_bar_button(
    parent: &mut ChildBuilder,
    button: &str,
    tooltip: ShowsTooltip,
    asset_server: &Res<AssetServer>,
) {
    parent
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(BUTTON_SIZE),
                    height: Val::Px(BUTTON_SIZE),
                    margin: UiRect::all(Val::Px(1.0)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                background_color: Color::BLACK.into(),
                ..default()
            },
            tooltip,
            Interaction::Hovered,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: String::from(button),
                        style: TextStyle {
                            font: asset_server.load("Rosela.ttf"),
                            font_size: 18.0,
                            color: Color::WHITE,
                        },
                    }],
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

pub fn setup_action_bar(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>) {
    parent
        .spawn(NodeBundle {
            style: Style {
                // width: Val::Px(200.),
                border: UiRect::all(Val::Px(2.)),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {

            spawn_action_bar_button(parent, "R", ShowsTooltip { title: "Attack".to_string(), description: "Attack the target, causing 100% weapon damage".to_string() }, asset_server);

            spawn_action_bar_button(parent, "Q", ShowsTooltip { title: "Cast Spell".to_string(), description: "Cast a spell that moves outward from the caster, causing 100% spell damage to the first target it hits.".to_string() }, asset_server);


            // parent.spawn(ActionBarButton::default());
        });
}
