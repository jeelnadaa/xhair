//! Command-line interface for offline crosshair rendering, format conversion, and MCP stdio proxy.

use anyhow::{bail, Context, Result};
use std::env;
use std::fs;
use std::path::PathBuf;
use xhair_model::crosshair::Crosshair;
use xhair_render::{render_png, RenderContext};

struct RenderArgs {
    input_path: Option<PathBuf>,
    preset: Option<String>,
    output_path: PathBuf,
    size: u32,
    scale: f32,
}

fn parse_render_args(args: &[String]) -> Result<RenderArgs> {
    let mut input_path = None;
    let mut preset = None;
    let mut output_path = None;
    let mut size = 64u32;
    let mut scale = 1.0f32;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--input" | "-i" => {
                if i + 1 < args.len() {
                    input_path = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            }
            "--preset" | "-p" => {
                if i + 1 < args.len() {
                    preset = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--output" | "-o" => {
                if i + 1 < args.len() {
                    output_path = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            }
            "--size" | "-s" => {
                if i + 1 < args.len() {
                    size = args[i + 1]
                        .parse()
                        .context("Invalid numeric value for --size")?;
                    i += 1;
                }
            }
            "--scale" => {
                if i + 1 < args.len() {
                    scale = args[i + 1]
                        .parse()
                        .context("Invalid numeric value for --scale")?;
                    i += 1;
                }
            }
            other if !other.starts_with('-') && input_path.is_none() => {
                input_path = Some(PathBuf::from(other));
            }
            _ => {}
        }
        i += 1;
    }

    let default_output = if let Some(ref input) = input_path {
        let mut stem = input.clone();
        stem.set_extension("png");
        stem
    } else {
        PathBuf::from("crosshair.png")
    };

    Ok(RenderArgs {
        input_path,
        preset,
        output_path: output_path.unwrap_or(default_output),
        size,
        scale,
    })
}

fn handle_render(args: &[String]) -> Result<()> {
    let config = parse_render_args(args)?;

    let crosshair = if let Some(ref path) = config.input_path {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read crosshair file at {:?}", path))?;
        serde_json::from_str::<Crosshair>(&content)
            .with_context(|| format!("Failed to parse crosshair JSON from {:?}", path))?
    } else if let Some(ref preset) = config.preset {
        match preset.to_lowercase().as_str() {
            "classic" => Crosshair::new_classic("Classic Reticle"),
            "dot" => {
                let mut xhair = Crosshair::new_blank("Dot Reticle");
                xhair.layers.push(xhair_model::layer::Layer::new(
                    "Center Dot",
                    xhair_model::shapes::LayerKind::Dot(xhair_model::shapes::DotProps {
                        diameter: 3.0,
                        shape: xhair_model::shapes::DotShape::Round,
                    }),
                ));
                xhair
            }
            "t_shape" | "t-shape" => {
                let mut xhair = Crosshair::new_blank("T-Shape Reticle");
                xhair.layers.push(xhair_model::layer::Layer::new(
                    "T-Shape",
                    xhair_model::shapes::LayerKind::TShape(xhair_model::shapes::TShapeProps {
                        bar_length: 10.0,
                        stem_length: 6.0,
                        thickness: 2.0,
                        gap: 2.0,
                    }),
                ));
                xhair
            }
            other => bail!(
                "Unknown preset '{}'. Available presets: classic, dot, t_shape",
                other
            ),
        }
    } else {
        // Default to classic crosshair
        Crosshair::new_classic("Classic Reticle")
    };

    let ctx = RenderContext::standard().with_scale(config.scale, config.scale);
    let png_bytes = render_png(&crosshair, &ctx, config.size, config.size)
        .context("Failed to rasterize crosshair into PNG")?;

    if let Some(parent) = config.output_path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }

    fs::write(&config.output_path, &png_bytes)
        .with_context(|| format!("Failed to write output image to {:?}", config.output_path))?;

    println!(
        "Rendered '{}' -> {:?} (size: {}x{}, scale: {:.2}, {} bytes)",
        crosshair.name,
        config.output_path,
        config.size,
        config.size,
        config.scale,
        png_bytes.len()
    );

    Ok(())
}

fn print_help() {
    println!("xhair-cli v{}", env!("CARGO_PKG_VERSION"));
    println!("High-performance crosshair overlay command-line utility\n");
    println!("USAGE:");
    println!("  xhair-cli render [input.xhair] [OPTIONS]");
    println!("  xhair-cli render --preset <classic|dot|t_shape> [OPTIONS]\n");
    println!("OPTIONS:");
    println!("  -i, --input <FILE>    Input .xhair or JSON file path");
    println!("  -p, --preset <NAME>   Built-in template preset (classic, dot, t_shape)");
    println!("  -o, --output <FILE>   Destination PNG file path [default: crosshair.png]");
    println!("  -s, --size <PIXELS>   Output square dimension in pixels [default: 64]");
    println!("      --scale <FACTOR>  Display scale multiplier [default: 1.0]");
    println!("  -h, --help            Print help information");
    println!("  -V, --version         Print version information");
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() || args.iter().any(|a| a == "-h" || a == "--help") {
        print_help();
        return Ok(());
    }

    if args.iter().any(|a| a == "-V" || a == "--version") {
        println!("xhair-cli v{}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    match args[0].as_str() {
        "render" => handle_render(&args[1..]),
        other => {
            eprintln!("Unknown command: '{}'", other);
            print_help();
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_version() {
        assert!(!env!("CARGO_PKG_VERSION").is_empty());
    }

    #[test]
    fn test_parse_render_args() {
        let args = vec![
            "--preset".into(),
            "classic".into(),
            "-o".into(),
            "test_out.png".into(),
            "--size".into(),
            "128".into(),
            "--scale".into(),
            "2.0".into(),
        ];
        let cfg = parse_render_args(&args).unwrap();
        assert_eq!(cfg.preset.as_deref(), Some("classic"));
        assert_eq!(cfg.output_path, PathBuf::from("test_out.png"));
        assert_eq!(cfg.size, 128);
        assert_eq!(cfg.scale, 2.0);
    }

    #[test]
    fn test_cli_render_preset_execution() {
        let temp_dir = tempfile::tempdir().unwrap();
        let out_file = temp_dir.path().join("out.png");

        let args = vec![
            "-p".into(),
            "classic".into(),
            "-o".into(),
            out_file.to_str().unwrap().into(),
            "-s".into(),
            "64".into(),
        ];

        handle_render(&args).expect("handle_render failed");
        assert!(out_file.exists());
        let meta = fs::metadata(&out_file).unwrap();
        assert!(meta.len() > 0);
    }
}
