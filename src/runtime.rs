//! What the decals plugin registers for a game (and the editor): every
//! runtime contribution, grouped by the engine seam it plugs into.

use decals_component::DecalComponent;
use engine_backend::scene::helio_bridge::SceneRowsRegistration;
use engine_backend::scene::{ClassSpec, ComponentsBuilder};
use engine_backend::subsystems::render::RenderFeatureRegistration;
use helio_component::texture_assets::TextureBindingRegistration;

/// Everything the plugin registers for a game (and the editor).
pub(crate) fn build(p: &mut pulsar_plugins::PluginBuilder) {
    // ── Component classes ───────────────────────────────────────────────────

    p.components().class::<DecalComponent>(ClassSpec::default());

    // ── Rendering ───────────────────────────────────────────────────────────

    // The decal drawer, after the G-buffer's.
    p.add(RenderFeatureRegistration {
        origin: p.origin(),
        features: decals_component::render_features,
        configure: |config| config,
    });

    // The decals' source rows and the join that places them in their owners'
    // boxes as the decal pass's rows.
    p.add(SceneRowsRegistration {
        origin: p.origin(),
        name: "decals.rows",
        register_columns: decals_component::register_columns,
        derivation: Some(decals_component::derivation),
    });

    // ── Assets ──────────────────────────────────────────────────────────────

    // A decal takes in its loaded albedo image.
    p.add(TextureBindingRegistration {
        origin: p.origin(),
        name: "decals.textures",
        register: decals_component::register_texture_binding,
    });
}

#[cfg(test)]
mod tests {
    use engine_backend::subsystems::render::helio_renderer::render_features;

    #[test]
    fn registers_its_feature() {
        let features = render_features::plugin_features();
        assert!(
            features.ids().contains(&"helio.decals"),
            "features: {:?}",
            features.ids()
        );
    }

    /// Every registration of the plugin names this root, so it resolves to
    /// the plugin's id.
    #[test]
    fn every_registration_resolves_to_the_plugin() {
        let ours: Vec<_> = pulsar_plugins::contributions()
            .into_iter()
            .filter(|summary| summary.origin == pulsar_plugins::origin!())
            .collect();
        assert!(
            ours.iter()
                .any(|summary| summary.category == "component.class")
        );
        for summary in &ours {
            assert_eq!(
                pulsar_plugins::plugin_of(summary.origin).map(|plugin| plugin.id.as_str()),
                Some("pulsar.decals"),
                "{summary:?}"
            );
        }
    }
}
