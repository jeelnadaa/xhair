//! MCP Prompt templates (§13.6).

use crate::protocol::{PromptArgument, PromptDefinition, PromptMessage, ToolContent};
use serde_json::Value;

/// Lists all prompt templates provided by the MCP server.
pub fn list_prompts() -> Vec<PromptDefinition> {
    vec![
        PromptDefinition {
            name: "design_crosshair_from_description".to_string(),
            description: "Designs a new crosshair based on natural language description."
                .to_string(),
            arguments: vec![
                PromptArgument {
                    name: "description".to_string(),
                    description: "Natural language description of the desired crosshair style."
                        .to_string(),
                    required: true,
                },
                PromptArgument {
                    name: "primary_color".to_string(),
                    description: "Optional preferred hex color (e.g. #36F2CB).".to_string(),
                    required: false,
                },
            ],
        },
        PromptDefinition {
            name: "tune_for_visibility".to_string(),
            description: "Analyzes contrast and recommends colors/glow for high game visibility."
                .to_string(),
            arguments: vec![PromptArgument {
                name: "crosshair_id".to_string(),
                description: "Crosshair UUID to analyze.".to_string(),
                required: true,
            }],
        },
        PromptDefinition {
            name: "convert_game_code".to_string(),
            description: "Converts in-game crosshair code (Valorant) into an xhair document."
                .to_string(),
            arguments: vec![PromptArgument {
                name: "code".to_string(),
                description: "Raw in-game crosshair code string.".to_string(),
                required: true,
            }],
        },
        PromptDefinition {
            name: "build_calibration_plan".to_string(),
            description: "Generates step-by-step instructions to verify sub-pixel centering."
                .to_string(),
            arguments: vec![PromptArgument {
                name: "resolution".to_string(),
                description: "Screen resolution (e.g. 1920x1080 or 1280x960).".to_string(),
                required: true,
            }],
        },
        PromptDefinition {
            name: "make_symmetric_variants".to_string(),
            description:
                "Takes an existing crosshair and generates 3-way, 4-way, and radial variants."
                    .to_string(),
            arguments: vec![PromptArgument {
                name: "crosshair_id".to_string(),
                description: "Source crosshair UUID.".to_string(),
                required: true,
            }],
        },
    ]
}

/// Retrieves a rendered prompt template with arguments filled in.
pub fn get_prompt(name: &str, args: Value) -> Result<(String, Vec<PromptMessage>), String> {
    match name {
        "design_crosshair_from_description" => {
            let desc = args
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let color = args
                .get("primary_color")
                .and_then(|v| v.as_str())
                .unwrap_or("#36F2CB");

            let prompt_text = format!(
                "Please design a new crosshair with the following requirements:\n\
                - Description: {}\n\
                - Preferred color: {}\n\n\
                Step 1: Call `crosshair.create` with a name.\n\
                Step 2: Call `layer.add` or `layer.update` to define arms/dots with exact game-pixel dimensions.\n\
                Step 3: Call `render.preview` to verify visual aesthetics.\n\
                Step 4: Call `analyze.alignment` to ensure pixel-edge or pixel-center alignment is accurate.",
                desc, color
            );

            Ok((
                "Design Crosshair".to_string(),
                vec![PromptMessage {
                    role: "user".to_string(),
                    content: ToolContent::Text { text: prompt_text },
                }],
            ))
        }
        "tune_for_visibility" => {
            let cid = args
                .get("crosshair_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let prompt_text = format!(
                "Analyze visibility for crosshair `{}`:\n\
                1. Call `analyze.visibility` with `crosshair_id: \"{}\"`.\n\
                2. If the contrast rating is below AAA, recommend adding an outer black outline or enabling glow.\n\
                3. Call `effects.set` to apply recommendations and verify with `render.preview`.",
                cid, cid
            );
            Ok((
                "Tune Visibility".to_string(),
                vec![PromptMessage {
                    role: "user".to_string(),
                    content: ToolContent::Text { text: prompt_text },
                }],
            ))
        }
        "convert_game_code" => {
            let code = args.get("code").and_then(|v| v.as_str()).unwrap_or("");
            let prompt_text = format!(
                "Convert this in-game crosshair code:\n`{}`\n\n\
                1. Call `import.valorant_code` with `code: \"{}\"`.\n\
                2. Inspect the imported layers with `crosshair.get`.\n\
                3. Call `render.preview` to confirm visual parity with the in-game appearance.",
                code, code
            );
            Ok((
                "Convert Game Code".to_string(),
                vec![PromptMessage {
                    role: "user".to_string(),
                    content: ToolContent::Text { text: prompt_text },
                }],
            ))
        }
        "build_calibration_plan" => {
            let res = args
                .get("resolution")
                .and_then(|v| v.as_str())
                .unwrap_or("1920x1080");
            let prompt_text = format!(
                "Calibration plan for resolution {}:\n\
                1. Call `analyze.alignment` to check whether this resolution has an even or odd width/height.\n\
                2. If even, true screen center lies on a pixel edge; if odd, center lies on pixel middle.\n\
                3. Call `calibration.show` to project the test pattern.\n\
                4. Adjust with `calibration.set_offset` if sub-pixel nudging is needed.",
                res
            );
            Ok((
                "Build Calibration Plan".to_string(),
                vec![PromptMessage {
                    role: "user".to_string(),
                    content: ToolContent::Text { text: prompt_text },
                }],
            ))
        }
        "make_symmetric_variants" => {
            let cid = args
                .get("crosshair_id")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let prompt_text = format!(
                "Generate symmetry variants for crosshair `{}`:\n\
                1. Call `crosshair.get` with `id: \"{}\"`.\n\
                2. Call `crosshair.duplicate` twice.\n\
                3. Update one variant with 3-arm radial symmetry (`arm_count: 3`) and one with T-shape.\n\
                4. Render both previews using `render.preview`.",
                cid, cid
            );
            Ok((
                "Make Symmetric Variants".to_string(),
                vec![PromptMessage {
                    role: "user".to_string(),
                    content: ToolContent::Text { text: prompt_text },
                }],
            ))
        }
        _ => Err(format!("Unknown prompt: {}", name)),
    }
}
