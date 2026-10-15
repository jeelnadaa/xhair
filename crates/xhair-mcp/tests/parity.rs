//! Parity CI test between UI actions, command registry, and MCP tools (§13.8).
//!
//! Principle P4: Anything a human can do in the UI exists as a command and therefore as an MCP tool.

use serde::Deserialize;
use std::collections::HashSet;
use std::path::PathBuf;
use xhair_mcp::list_tools;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct UiActionManifestItem {
    id: String,
    name: String,
    command: String,
    mcp_tool: String,
    description: String,
}

fn repo_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // crates/xhair-mcp -> root
    manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("Failed to locate repo root")
        .to_path_buf()
}

#[test]
fn test_ui_command_mcp_parity() {
    let root = repo_root();
    let actions_path = root.join("ui/src/generated/actions.json");

    assert!(
        actions_path.exists(),
        "UI actions manifest missing at {:?}. Run export before running parity test.",
        actions_path
    );

    let content = std::fs::read_to_string(&actions_path)
        .unwrap_or_else(|e| panic!("Failed to read {:?}: {}", actions_path, e));

    let ui_actions: Vec<UiActionManifestItem> = serde_json::from_str(&content)
        .unwrap_or_else(|e| panic!("Failed to parse UI actions JSON: {}", e));

    assert!(
        !ui_actions.is_empty(),
        "UI actions manifest must not be empty"
    );

    let tools = list_tools();
    assert!(!tools.is_empty(), "MCP tools list must not be empty");

    let tool_names: HashSet<&str> = tools.iter().map(|t| t.name.as_str()).collect();

    // 1. Every tool must have documentation (non-empty description) and input schema
    for tool in &tools {
        assert!(
            !tool.description.trim().is_empty(),
            "MCP Tool '{}' lacks a description",
            tool.name
        );
        assert!(
            tool.input_schema.is_object(),
            "MCP Tool '{}' lacks a valid JSON object input schema",
            tool.name
        );
    }

    // 2. Every UI action must map to a command and a valid MCP tool
    for action in &ui_actions {
        assert!(
            !action.command.trim().is_empty(),
            "UI action '{}' lacks a command mapping",
            action.id
        );
        assert!(
            !action.description.trim().is_empty(),
            "UI action '{}' lacks a description",
            action.id
        );
        assert!(
            tool_names.contains(action.mcp_tool.as_str()),
            "UI action '{}' points to MCP tool '{}' which is not registered in list_tools()",
            action.id,
            action.mcp_tool
        );
    }

    // 3. Every document mutation command in xhair-commands must have a corresponding MCP tool
    let core_commands = [
        "layer.add",
        "layer.update",
        "layer.remove",
        "layer.reorder",
        "layer.duplicate",
        "global.set",
        "symmetry.set",
        "effects.set",
        "nudge",
        "edit.batch",
    ];

    for cmd in core_commands {
        assert!(
            tool_names.contains(cmd),
            "Core command '{}' has no corresponding MCP tool (violates P4 Human/Agent Parity)",
            cmd
        );
        assert!(
            ui_actions.iter().any(|a| a.command == cmd),
            "Core command '{}' is not declared in UI actions manifest (violates P4)",
            cmd
        );
    }

    // 4. Verify specific key high-value tools exist
    let required_tools = [
        "library.list",
        "crosshair.create",
        "crosshair.get",
        "crosshair.delete",
        "render.preview",
        "analyze.alignment",
        "analyze.visibility",
        "validate",
        "overlay.get_state",
        "overlay.set_enabled",
        "overlay.activate",
        "export.code",
        "import.code",
        "import.valorant_code",
        "meta.capabilities",
        "meta.schema",
        "undo",
        "redo",
    ];

    for req in required_tools {
        assert!(
            tool_names.contains(req),
            "Required MCP tool '{}' is missing from server implementation",
            req
        );
    }

    println!(
        "Parity verified: {} UI actions, {} core commands, and {} MCP tools in complete alignment.",
        ui_actions.len(),
        core_commands.len(),
        tools.len()
    );
}
