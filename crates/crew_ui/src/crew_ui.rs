use command_palette_hooks::CommandPaletteFilter;
use feature_flags::{CrewFeatureFlag, FeatureFlagAppExt};
use gpui::{App, PromptLevel, actions};
use std::any::TypeId;
use util::ResultExt;

actions!(crew, [NewSession]);

pub fn init(cx: &mut App) {
    update_action_visibility(cx);
    cx.observe_flag::<CrewFeatureFlag, _>(|_, cx| update_action_visibility(cx))
        .detach();
    cx.on_action(|_: &NewSession, cx| {
        if !cx.has_flag::<CrewFeatureFlag>() {
            return;
        }
        if let Some(window) = cx.active_window() {
            // Action dispatch already borrows the window until the current update finishes.
            cx.defer(move |cx| {
                if !cx.has_flag::<CrewFeatureFlag>() {
                    return;
                }
                window
                    .update(cx, |_, window, cx| {
                        drop(window.prompt(
                            PromptLevel::Info,
                            "Crew sessions are not supported yet",
                            None,
                            &["OK"],
                            cx,
                        ));
                    })
                    .log_err();
            });
        }
    });
}

fn update_action_visibility(cx: &mut App) {
    let enabled = cx.has_flag::<CrewFeatureFlag>();
    CommandPaletteFilter::update_global(cx, |filter, _| {
        let action_types = [TypeId::of::<NewSession>()];
        if enabled {
            filter.show_action_types(&action_types);
        } else {
            filter.hide_action_types(&action_types);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use command_palette_hooks::CommandPaletteFilter;
    use feature_flags::{
        CrewFeatureFlag, FeatureFlagAppExt, FeatureFlagStore, FeatureFlagsSettings,
    };
    use gpui::{TestAppContext, UpdateGlobal};
    use settings::{Settings, SettingsStore};

    #[gpui::test]
    fn crew_action_tracks_flag_changes(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            FeatureFlagsSettings::register(cx);
            FeatureFlagStore::init(cx);
            command_palette_hooks::init(cx);
            init(cx);
            assert!(CommandPaletteFilter::global_mut(cx).is_hidden(&NewSession));
        });
        let window = cx.add_window(|_, _| gpui::Empty);
        let activation_result = window.update(cx, |_, window, _| window.activate_window());
        assert!(
            activation_result.is_ok(),
            "could not activate test window: {activation_result:?}"
        );
        cx.run_until_parked();
        cx.update(|cx| {
            cx.set_staff(true);
            assert!(!cx.has_flag::<CrewFeatureFlag>());
            assert!(CommandPaletteFilter::global_mut(cx).is_hidden(&NewSession));
            assert!(cx.active_window().is_some());
            cx.dispatch_action(&NewSession);
        });
        cx.run_until_parked();
        assert!(!cx.has_pending_prompt());
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .feature_flags
                        .get_or_insert_default()
                        .insert("crew".into(), "on".into());
                });
            });
        });
        cx.run_until_parked();
        cx.update(|cx| {
            assert!(!CommandPaletteFilter::global_mut(cx).is_hidden(&NewSession));
            cx.dispatch_action(&NewSession);
        });
        cx.run_until_parked();
        assert!(
            cx.pending_prompt()
                .is_some_and(|(message, _)| message.contains("not supported"))
        );
        cx.simulate_prompt_answer("OK");
        cx.update(|cx| {
            SettingsStore::update_global(cx, |store, cx| {
                store.update_user_settings(cx, |settings| {
                    settings
                        .feature_flags
                        .get_or_insert_default()
                        .insert("crew".into(), "off".into());
                });
            });
        });
        cx.run_until_parked();
        cx.update(|cx| {
            assert!(CommandPaletteFilter::global_mut(cx).is_hidden(&NewSession));
        });
    }
}
