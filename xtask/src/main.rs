use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GraphNode {
    id: String,
    #[serde(rename = "type")]
    node_type: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    summary: String,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GraphEdge {
    from: String,
    to: String,
    #[serde(rename = "type")]
    edge_type: String,
}

struct MemoryGraph {
    nodes: HashMap<String, GraphNode>,
    edges: Vec<GraphEdge>,
}

impl MemoryGraph {
    fn load(repo_root: &Path) -> Result<Self> {
        let mut nodes = HashMap::new();
        let mut edges = Vec::new();

        let node_paths = [
            repo_root.join("docs/agent/graph/curated/nodes.jsonl"),
            repo_root.join("docs/agent/graph/generated/nodes.jsonl"),
        ];

        for path in &node_paths {
            if path.exists() {
                let content = fs::read_to_string(path)?;
                for line in content.lines() {
                    let line = line.trim();
                    if !line.is_empty() {
                        let node: GraphNode = serde_json::from_str(line)
                            .with_context(|| format!("Failed to parse line in {:?}", path))?;
                        nodes.insert(node.id.clone(), node);
                    }
                }
            }
        }

        let edge_paths = [
            repo_root.join("docs/agent/graph/curated/edges.jsonl"),
            repo_root.join("docs/agent/graph/generated/edges.jsonl"),
        ];

        for path in &edge_paths {
            if path.exists() {
                let content = fs::read_to_string(path)?;
                for line in content.lines() {
                    let line = line.trim();
                    if !line.is_empty() {
                        let edge: GraphEdge = serde_json::from_str(line)
                            .with_context(|| format!("Failed to parse edge line in {:?}", path))?;
                        edges.push(edge);
                    }
                }
            }
        }

        Ok(Self { nodes, edges })
    }

    fn print_summary(&self) {
        println!("=== XHAIR Project Memory Graph Summary ===");
        println!("Total Nodes: {}", self.nodes.len());
        println!("Total Edges: {}", self.edges.len());

        let mut type_counts: HashMap<String, usize> = HashMap::new();
        for node in self.nodes.values() {
            *type_counts.entry(node.node_type.clone()).or_insert(0) += 1;
        }

        println!("\nNodes by Type:");
        let mut types: Vec<_> = type_counts.into_iter().collect();
        types.sort_by(|a, b| a.0.cmp(&b.0));
        for (t, count) in types {
            println!("  - {:<15}: {}", t, count);
        }

        println!("\nInvariants & Requirements:");
        for node in self.nodes.values() {
            if node.node_type == "Requirement" {
                println!("  - [{}] {}: {}", node.id, node.name, node.summary);
            }
        }
    }

    fn print_node(&self, target_id: &str) {
        if let Some(node) = self.nodes.get(target_id) {
            println!("Node: {} ({})", node.id, node.node_type);
            println!("Name: {}", node.name);
            println!("Summary: {}", node.summary);
            if let Some(ref p) = node.path {
                println!("Path: {}", p);
            }
            if !node.tags.is_empty() {
                println!("Tags: {}", node.tags.join(", "));
            }

            println!("\nOutgoing Relationships:");
            for edge in &self.edges {
                if edge.from == target_id {
                    println!("  --[{}]--> {}", edge.edge_type, edge.to);
                }
            }

            println!("\nIncoming Relationships:");
            for edge in &self.edges {
                if edge.to == target_id {
                    println!("  <--[{}]-- {}", edge.edge_type, edge.from);
                }
            }
        } else {
            eprintln!("Node not found: {}", target_id);
        }
    }
}

fn get_repo_root() -> Result<PathBuf> {
    let mut dir = env::current_dir()?;
    loop {
        if dir.join("Cargo.toml").exists() && dir.join("docs").exists() {
            return Ok(dir);
        }
        if !dir.pop() {
            break;
        }
    }
    env::current_dir().context("Could not determine repository root")
}

fn handle_graph(args: &[String]) -> Result<()> {
    let root = get_repo_root()?;
    let graph = MemoryGraph::load(&root)?;

    if args.is_empty() || args.iter().any(|a| a == "--summary") {
        graph.print_summary();
        return Ok(());
    }

    if let Some(pos) = args.iter().position(|a| a == "--node") {
        if let Some(id) = args.get(pos + 1) {
            graph.print_node(id);
            return Ok(());
        }
    }

    // Default to summary
    graph.print_summary();
    Ok(())
}

fn handle_check_agent_state() -> Result<()> {
    let root = get_repo_root()?;
    println!("Checking agent continuity files in {:?}...", root);

    let required_files = [
        "AGENTS.md",
        "CLAUDE.md",
        ".github/copilot-instructions.md",
        ".cursor/rules/xhair.mdc",
        "docs/agent/PROJECT_STATE.md",
        "docs/agent/TASKS.md",
        "docs/agent/SESSION_LOG.md",
        "docs/agent/INVARIANTS.md",
        "docs/agent/OPEN_QUESTIONS.md",
        "docs/agent/GLOSSARY.md",
        "docs/spec/CROSSHAIR_OVERLAY_SPEC.md",
        "docs/spec/AGENT_CONTINUITY_AND_RELEASES.md",
    ];

    for file in &required_files {
        let p = root.join(file);
        if !p.exists() {
            anyhow::bail!("Missing required agent continuity file: {}", file);
        }
    }

    println!("All agent continuity files verified successfully.");
    Ok(())
}

fn handle_schema(args: &[String]) -> Result<()> {
    let root = get_repo_root()?;
    let registry = xhair_commands::PropertyRegistry::standard();
    let json_content = serde_json::to_string_pretty(&registry)? + "\n";
    let md_content = registry.to_markdown();

    let json_path = root.join("ui/src/generated/properties.json");
    let md_path = root.join("docs/PROPERTIES.md");

    let check_mode = args.iter().any(|a| a == "--check");

    if check_mode {
        let existing_json = if json_path.exists() {
            fs::read_to_string(&json_path)?
        } else {
            String::new()
        };
        let existing_md = if md_path.exists() {
            fs::read_to_string(&md_path)?
        } else {
            String::new()
        };

        let json_normalized = existing_json.replace("\r\n", "\n");
        let md_normalized = existing_md.replace("\r\n", "\n");
        let gen_json_normalized = json_content.replace("\r\n", "\n");
        let gen_md_normalized = md_content.replace("\r\n", "\n");

        if json_normalized != gen_json_normalized || md_normalized != gen_md_normalized {
            anyhow::bail!(
                "Property schema drift detected! Run 'cargo xtask schema' to regenerate."
            );
        }

        println!("Property schemas and documentation are up to date.");
        Ok(())
    } else {
        if let Some(parent) = json_path.parent() {
            fs::create_dir_all(parent)?;
        }
        if let Some(parent) = md_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&json_path, json_content)?;
        fs::write(&md_path, md_content)?;

        println!("Generated properties schema: {:?}", json_path);
        println!("Generated properties documentation: {:?}", md_path);
        Ok(())
    }
}

fn handle_package(args: &[String]) -> Result<()> {
    println!("Packaging xhair release artifacts (§21.3, M10)...");
    let root = get_repo_root()?;
    let mut version = "0.1.0".to_string();
    if let Some(pos) = args.iter().position(|a| a == "--version") {
        if let Some(v) = args.get(pos + 1) {
            version = v.clone();
        }
    }
    println!("Target version: {}", version);

    let dist_dir = root.join("dist");
    fs::create_dir_all(&dist_dir)?;

    // Locate binaries in target/release or debug
    let mut app_exe = root.join("target/release/xhair.exe");
    let mut cli_exe = root.join("target/release/xhair-cli.exe");

    if !app_exe.exists() || !cli_exe.exists() {
        println!("Release binaries not found, checking debug binaries...");
        if root.join("target/debug/xhair.exe").exists()
            && root.join("target/debug/xhair-cli.exe").exists()
        {
            app_exe = root.join("target/debug/xhair.exe");
            cli_exe = root.join("target/debug/xhair-cli.exe");
        } else {
            println!("Compiling binaries with cargo build --bin xhair --bin xhair-cli...");
            let status = std::process::Command::new("cargo")
                .args(["build", "--bin", "xhair", "--bin", "xhair-cli"])
                .current_dir(&root)
                .status()?;
            if !status.success() {
                anyhow::bail!("cargo build failed for xhair / xhair-cli binaries");
            }
            app_exe = root.join("target/debug/xhair.exe");
            cli_exe = root.join("target/debug/xhair-cli.exe");
        }
    }

    let readme_path = root.join("README.md");
    let license_path = root.join("LICENSE");

    // 1. Generate NSIS installer script: dist/installer.nsi
    let nsi_path = dist_dir.join("installer.nsi");
    let nsi_content = format!(
        r#"; XHAIR Windows NSIS Installer Script (§21.3)
!include "MUI2.nsh"

Name "XHAIR"
OutFile "xhair-{}-x64-setup.exe"
InstallDir "$PROGRAMFILES64\xhair"
InstallDirRegKey HKCU "Software\xhair" "Install_Dir"
RequestExecutionLevel admin

!define MUI_ABORTWARNING
!define MUI_ICON "..\design\favicon.ico"
!define MUI_UNICON "..\design\favicon.ico"

!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "{}"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

!insertmacro MUI_LANGUAGE "English"

Section "XHAIR (required)" SecCore
  SectionIn RO
  SetOutPath "$INSTDIR"
  File "{}"
  File "{}"
  File "{}"
  File "{}"

  WriteRegStr HKCU "Software\xhair" "Install_Dir" "$INSTDIR"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\xhair" "DisplayName" "XHAIR Crosshair Overlay"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\xhair" "UninstallString" '"$INSTDIR\uninstall.exe"'
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\xhair" "DisplayVersion" "{}"
  WriteRegStr HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\xhair" "Publisher" "jeelnadaa"
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\xhair" "NoModify" 1
  WriteRegDWORD HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\xhair" "NoRepair" 1
  WriteUninstaller "$INSTDIR\uninstall.exe"

  CreateDirectory "$SMPROGRAMS\XHAIR"
  CreateShortcut "$SMPROGRAMS\XHAIR\XHAIR.lnk" "$INSTDIR\xhair.exe" "" "$INSTDIR\xhair.exe" 0
  CreateShortcut "$SMPROGRAMS\XHAIR\Uninstall XHAIR.lnk" "$INSTDIR\uninstall.exe" "" "$INSTDIR\uninstall.exe" 0
  CreateShortcut "$DESKTOP\XHAIR.lnk" "$INSTDIR\xhair.exe" "" "$INSTDIR\xhair.exe" 0
SectionEnd

Section "Uninstall"
  DeleteRegKey HKLM "Software\Microsoft\Windows\CurrentVersion\Uninstall\xhair"
  DeleteRegKey HKCU "Software\xhair"
  DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "xhair"

  Delete "$INSTDIR\xhair.exe"
  Delete "$INSTDIR\xhair-cli.exe"
  Delete "$INSTDIR\README.md"
  Delete "$INSTDIR\LICENSE"
  Delete "$INSTDIR\uninstall.exe"

  Delete "$SMPROGRAMS\XHAIR\XHAIR.lnk"
  Delete "$SMPROGRAMS\XHAIR\Uninstall XHAIR.lnk"
  RMDir "$SMPROGRAMS\XHAIR"
  Delete "$DESKTOP\XHAIR.lnk"

  RMDir "$INSTDIR"
SectionEnd
"#,
        version,
        license_path.display(),
        app_exe.display(),
        cli_exe.display(),
        readme_path.display(),
        license_path.display(),
        version
    );
    fs::write(&nsi_path, nsi_content)?;
    println!("Generated NSIS script: {:?}", nsi_path);

    // If makensis is installed, compile installer
    if let Ok(status) = std::process::Command::new("makensis")
        .arg(&nsi_path)
        .status()
    {
        if status.success() {
            println!(
                "Compiled NSIS setup executable: dist/xhair-{}-x64-setup.exe",
                version
            );
        }
    }

    // 2. Build xhair-<version>-x64-portable.zip
    let portable_zip_path = dist_dir.join(format!("xhair-{}-x64-portable.zip", version));
    {
        let file = fs::File::create(&portable_zip_path)?;
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        // xhair.exe
        if app_exe.exists() {
            zip.start_file("xhair.exe", options)?;
            let mut f = fs::File::open(&app_exe)?;
            std::io::copy(&mut f, &mut zip)?;
        }

        // xhair-cli.exe
        if cli_exe.exists() {
            zip.start_file("xhair-cli.exe", options)?;
            let mut f = fs::File::open(&cli_exe)?;
            std::io::copy(&mut f, &mut zip)?;
        }

        // xhair.portable marker
        zip.start_file("xhair.portable", options)?;
        zip.write_all(b"# Portable mode marker file (spec \xc2\xa721.3)\n")?;

        // README and LICENSE
        if readme_path.exists() {
            zip.start_file("README.md", options)?;
            let mut f = fs::File::open(&readme_path)?;
            std::io::copy(&mut f, &mut zip)?;
        }
        if license_path.exists() {
            zip.start_file("LICENSE", options)?;
            let mut f = fs::File::open(&license_path)?;
            std::io::copy(&mut f, &mut zip)?;
        }

        // Include compiled UI bundle (ui/dist) for offline editor serving
        let ui_dist = root.join("ui/dist");
        if ui_dist.exists() {
            fn add_dir_to_zip(
                dir: &Path,
                prefix: &str,
                zip: &mut zip::ZipWriter<fs::File>,
                options: zip::write::SimpleFileOptions,
            ) -> Result<()> {
                for entry in fs::read_dir(dir)? {
                    let entry = entry?;
                    let path = entry.path();
                    let name = entry.file_name().to_string_lossy().to_string();
                    let zip_path = if prefix.is_empty() {
                        name.clone()
                    } else {
                        format!("{}/{}", prefix, name)
                    };
                    if path.is_file() {
                        zip.start_file(&zip_path, options)?;
                        let mut f = fs::File::open(&path)?;
                        std::io::copy(&mut f, zip)?;
                    } else if path.is_dir() {
                        add_dir_to_zip(&path, &zip_path, zip, options)?;
                    }
                }
                Ok(())
            }
            let _ = add_dir_to_zip(&ui_dist, "ui/dist", &mut zip, options);
        }

        zip.finish()?;
    }
    println!("Created portable zip: {:?}", portable_zip_path);

    // 3. Build xhair-cli-<version>-x64.zip
    let cli_zip_path = dist_dir.join(format!("xhair-cli-{}-x64.zip", version));
    {
        let file = fs::File::create(&cli_zip_path)?;
        let mut zip = zip::ZipWriter::new(file);
        let options = zip::write::SimpleFileOptions::default()
            .compression_method(zip::CompressionMethod::Deflated);

        if cli_exe.exists() {
            zip.start_file("xhair-cli.exe", options)?;
            let mut f = fs::File::open(&cli_exe)?;
            std::io::copy(&mut f, &mut zip)?;
        }
        if readme_path.exists() {
            zip.start_file("README.md", options)?;
            let mut f = fs::File::open(&readme_path)?;
            std::io::copy(&mut f, &mut zip)?;
        }
        if license_path.exists() {
            zip.start_file("LICENSE", options)?;
            let mut f = fs::File::open(&license_path)?;
            std::io::copy(&mut f, &mut zip)?;
        }
        zip.finish()?;
    }
    println!("Created CLI zip: {:?}", cli_zip_path);

    // 4. Generate sbom.json
    let sbom_path = dist_dir.join("sbom.json");
    let sbom_json = serde_json::json!({
        "$schema": "http://cyclonedx.org/schema/bom-1.5.json",
        "bomFormat": "CycloneDX",
        "specVersion": "1.5",
        "version": 1,
        "metadata": {
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "component": {
                "name": "xhair",
                "version": version,
                "type": "application",
                "licenses": [{ "license": { "id": "MIT OR Apache-2.0" } }],
                "author": "jeelnadaa",
                "description": "Cross-platform Windows crosshair overlay with Svelte 5 editor and MCP server"
            }
        },
        "components": [
            { "name": "xhair-model", "version": version, "type": "library" },
            { "name": "xhair-geometry", "version": version, "type": "library" },
            { "name": "xhair-commands", "version": version, "type": "library" },
            { "name": "xhair-render", "version": version, "type": "library" },
            { "name": "xhair-format", "version": version, "type": "library" },
            { "name": "xhair-library", "version": version, "type": "library" },
            { "name": "xhair-platform-win", "version": version, "type": "library" },
            { "name": "xhair-dynamics", "version": version, "type": "library" },
            { "name": "xhair-mcp", "version": version, "type": "library" },
            { "name": "xhair-app", "version": version, "type": "application" },
            { "name": "xhair-cli", "version": version, "type": "application" }
        ]
    });
    fs::write(&sbom_path, serde_json::to_string_pretty(&sbom_json)? + "\n")?;
    println!("Generated SBOM: {:?}", sbom_path);

    // 5. Generate SHA256SUMS.txt
    let sums_path = dist_dir.join("SHA256SUMS.txt");
    let mut sums_content = String::new();

    for entry in fs::read_dir(&dist_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let filename = path.file_name().unwrap().to_string_lossy().to_string();
            if filename == "SHA256SUMS.txt" {
                continue;
            }
            let bytes = fs::read(&path)?;
            let mut hasher = sha2::Sha256::new();
            sha2::Digest::update(&mut hasher, &bytes);
            let hash = hasher
                .finalize()
                .into_iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>();
            sums_content.push_str(&format!("{}  {}\n", hash, filename));
        }
    }
    fs::write(&sums_path, sums_content)?;
    println!("Generated SHA256SUMS.txt: {:?}", sums_path);

    println!(
        "\nRelease packaging completed successfully for v{} in dist/",
        version
    );
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: cargo xtask [graph | check-agent-state | schema | package] [args...]");
        return Ok(());
    }

    match args[0].as_str() {
        "graph" => handle_graph(&args[1..]),
        "check-agent-state" => handle_check_agent_state(),
        "schema" => handle_schema(&args[1..]),
        "package" => handle_package(&args[1..]),
        other => {
            eprintln!("Unknown xtask command: {}", other);
            eprintln!("Available: graph, check-agent-state, schema, package");
            std::process::exit(1);
        }
    }
}
