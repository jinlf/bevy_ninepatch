use bevy::prelude::*;

use bevy_ninepatch::{
    NinePatchBuilder, NinePatchBundle, NinePatchContent, NinePatchData, NinePatchPlugin,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::default()
        .add_plugins(DefaultPlugins)
        // Add the `NinePatchPlugin` plugin
        .add_plugins(NinePatchPlugin::<()>::default())
        .add_systems(Startup, setup)
        .add_systems(Update, set_content)
        .run();

    Ok(())
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
) {
    let panel_texture_handle = asset_server.load("glassPanel_corners.png");

    // load the 9-Patch as an assets and keep an `Handle<NinePatchBuilder<()>>`
    let nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(20, 20, 20, 20));

    commands.spawn(
        // this component bundle will be detected by the plugin, and the 9-Patch UI element will be added as a child
        // of this entity
        NinePatchBundle {
            style: Style {
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                // size: Size::new(Val::Px(500.), Val::Px(300.)),
                width: Val::Px(500.),
                height: Val::Px(300.),
                ..Default::default()
            },
            nine_patch_data: NinePatchData {
                nine_patch: nine_patch_handle,
                texture: panel_texture_handle,
                ..Default::default()
            },
            ..Default::default()
        },
    );

    commands.spawn(Camera2dBundle::default());
}

fn set_content(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(Entity, &mut NinePatchContent<()>)>,
) {
    for (entity, mut nine_patch_content) in query.iter_mut() {
        if !nine_patch_content.loaded {
            // load font
            let font = asset_server.load("Kenney Future Narrow.ttf");

            let content_entity = commands
                .spawn(TextBundle::from_section(
                    "Hello!",
                    TextStyle {
                        font,
                        font_size: 50.0,
                        color: Color::GREEN,
                    },
                ))
                .id();
            commands.entity(entity).push_children(&[content_entity]);
            nine_patch_content.loaded = true;
        }
    }
}
