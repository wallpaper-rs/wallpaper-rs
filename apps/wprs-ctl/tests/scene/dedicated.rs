use super::fields;
use clap::CommandFactory;
use serde_json::json;
use wprs_ctl::Cli;
use wprs_editor::ACTIONS;

#[test]
fn a_dedicated_scene_command_for_an_action_takes_the_same_arguments() {
    assert_eq!(fields(json!({"": {}})), ACTIONS);
    let scene = Cli::command().find_subcommand("scene").cloned().unwrap();
    let mut lacking = Vec::new();
    for name in ACTIONS {
        let Some(command) = scene.find_subcommand(name.replace('_', "-")) else { continue };
        let mut ids: Vec<&str> = command.get_arguments().map(|a| a.get_id().as_str()).collect();
        for sub in command.get_subcommands() {
            ids.push(sub.get_name());
            ids.extend(sub.get_arguments().map(|a| a.get_id().as_str()));
        }
        let by_act = [
            ("effects", "grip"),
            ("effects", "mask_grip"),
            ("layer", "depth"),
            ("canvas", "probe"),
            ("canvas", "camera"),
            ("canvas", "camera_grip"),
            ("canvas", "camera_view"),
            ("manifest", "requires"),
            ("manifest", "preset"),
            ("manifest", "media"),
            ("manifest", "credit"),
            ("manifest", "uncredit"),
            ("manifest", "text"),
            ("manifest", "locale"),
            ("layer", "figure"),
            ("layer", "start"),
        ];
        for field in fields(json!({name: {"": null}})) {
            if by_act.contains(&(name, field.as_str())) {
                continue;
            }
            let named = ids.contains(&field.as_str()) || ids.contains(&field.replace('_', "-").as_str());
            if !named {
                lacking.push(format!("{name}.{field}"));
            }
        }
        assert!(ids.contains(&"dir"), "scene {name} takes the folder");
    }
    assert!(lacking.is_empty(), "scene commands lack {lacking:?}");
}
