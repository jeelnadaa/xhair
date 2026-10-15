use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
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

fn handle_package(args: &[String]) -> Result<()> {
    println!("Packaging xhair...");
    let mut version = "0.1.0";
    if let Some(pos) = args.iter().position(|a| a == "--version") {
        if let Some(v) = args.get(pos + 1) {
            version = v;
        }
    }
    println!("Target version: {}", version);
    println!("Package step ready (stub exits 0).");
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: cargo xtask [graph | check-agent-state | package] [args...]");
        return Ok(());
    }

    match args[0].as_str() {
        "graph" => handle_graph(&args[1..]),
        "check-agent-state" => handle_check_agent_state(),
        "package" => handle_package(&args[1..]),
        other => {
            eprintln!("Unknown xtask command: {}", other);
            eprintln!("Available: graph, check-agent-state, package");
            std::process::exit(1);
        }
    }
}
