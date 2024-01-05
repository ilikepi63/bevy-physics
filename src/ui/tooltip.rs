use bevy::prelude::*;

#[derive(Event)]
pub struct TooltipState {
    title: String, // Ideally we'd want to avoid allocations here, so we pre-allocate these to a fixed number
    description: String,
    shown: bool,
}

impl Default for TooltipState {
    fn default() -> Self {
        TooltipState {
            title: String::with_capacity(128),
            description: String::with_capacity(256),
            shown: false,
        }
    }
}

#[derive(Component)]
pub struct Tooltip;

// we spawn the tooltip
pub fn setup_tooltip(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    right: Val::Px(1.0),
                    bottom: Val::Px(1.0),
                    padding: UiRect::all(Val::Px(10.0)),
                    flex_direction: FlexDirection::Column,
                    // overflow: Overflow::
                    ..Default::default()
                },
                background_color: Color::BLACK.into(),
                visibility: Visibility::Hidden,
                ..Default::default()
            },
            Tooltip,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style { 
                    flex_wrap: FlexWrap::Wrap,
                    ..Default::default()
                },
                text: Text {
                    sections: vec![TextSection {
                        value: String::with_capacity(128),
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

            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: String::with_capacity(256),
                        style: TextStyle {
                            font: asset_server.load("Rosela.ttf"),
                            font_size: 12.0,
                            color: Color::WHITE,
                            
                        },
                    }],
                    ..Default::default()
                },
                style: Style{
                    max_width: Val::Px(200.0),
                    ..Default::default()
                },
                ..Default::default()
            });
        });
}

pub fn tooltip_events(
    mut tooltip: EventReader<TooltipState>,
    mut ui_tooltip: Query<(&mut Visibility, &mut Style, &mut Children), With<Tooltip>>,
    mut text_query: Query<&mut Text>,
) {
    for event in tooltip.iter() {
        for (mut visibility, style, mut children) in &mut ui_tooltip {
            if event.shown {
                *visibility = Visibility::Visible;

                text_query
                    .get_mut(children[0])
                    .unwrap()
                    .sections
                    .get_mut(0)
                    .map(|val| val.value = event.title.clone()); // TODO: remove this allocation
                text_query
                    .get_mut(children[1])
                    .unwrap()
                    .sections
                    .get_mut(0)
                    .map(|val| val.value = event.description.clone()); // TODO: remove this allocation
            } else {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

#[derive(Component)]
pub struct ShowsTooltip {
    pub title: String,
    pub description: String,
}

pub fn mouseover_system(
    mut interactions: Query<
        (&Interaction, &ShowsTooltip),
        (Changed<Interaction>, With<ShowsTooltip>),
    >,
    mut event_writer: EventWriter<TooltipState>,
) {
    for (interaction, shows_tooltip) in &mut interactions {
        match *interaction {
            Interaction::Hovered => event_writer.send(TooltipState {
                title: shows_tooltip.title.clone(),
                description: shows_tooltip.description.clone(),
                shown: true,
            }),
            Interaction::None => {
                event_writer.send(TooltipState {
                    title: "".to_string(),
                    description: "".to_string(),
                    shown: false,
                });
            }
            _ => {
                // info!("Doing something else! {:#?}", interaction);
            }
        }
    }
}
