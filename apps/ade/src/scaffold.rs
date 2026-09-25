use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

/// Validates whether a project name is a canonical slug matching `^[a-z0-9][a-z0-9_-]*$`.
pub fn is_valid_slug(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() || c.is_ascii_digit() => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '-')
}

/// Scaffolds a new canonical SCPE v0.3.0 product workspace at `target_path`.
/// Rejects creation if the directory exists and contains files/subdirectories.
pub fn scaffold_scpe_workspace(target_path: &Path, product_name: &str) -> Result<(), Error> {
    if !is_valid_slug(product_name) {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            format!("Invalid project slug '{product_name}'. Must match ^[a-z0-9][a-z0-9_-]*$"),
        ));
    }

    if target_path.exists() {
        if target_path.is_file() {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "Target path exists and is a file",
            ));
        }
        let mut entries = fs::read_dir(target_path)?;
        if entries.next().is_some() {
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "Target directory already exists and is not empty",
            ));
        }
    } else {
        fs::create_dir_all(target_path)?;
    }

    // Create canonical directories
    let apps_dir = target_path.join("apps");
    let features_dir = target_path.join("features");
    let assets_dir = target_path.join("assets");

    fs::create_dir_all(&apps_dir)?;
    fs::create_dir_all(&features_dir)?;
    fs::create_dir_all(&assets_dir)?;

    fs::write(apps_dir.join(".gitkeep"), "")?;
    fs::write(features_dir.join(".gitkeep"), "")?;
    fs::write(assets_dir.join(".gitkeep"), "")?;

    let today = "2026-09-24";

    // index.md
    let index_content = format!(
        r#"# {product_name}

- **product:** {product_name}
- **method:** SCPE — Spec-Compiled Product Engineering
- **created:** {today}

Master guide for people and agents. Start here, then follow the links.

## Product Root

- [product_vision.md](product_vision.md) — macro vision, business goals, profitability, core problem
- [roadmap.md](roadmap.md) — strategic direction and milestones
- [glossary.md](glossary.md) — ubiquitous language
- [architecture.md](architecture.md) — C4 model, systemic choices, integrations
- [technical_deal.md](technical_deal.md) — technical agreements, AI guardrails, approved stack
- [team_playbook.md](team_playbook.md) — rules of engagement and workflow
- [quick_status.md](quick_status.md) — product-wide status panel

## Apps

_No apps yet._

## Features

_No features yet._
"#
    );
    fs::write(target_path.join("index.md"), index_content)?;

    // product_vision.md
    let vision_content = format!(
        r#"# Product Vision: {product_name}

- **product:** {product_name}
- **status:** Draft
- **updated:** {today}

## 1. Executive Summary
Executive vision and business purpose for {product_name}.

## 2. Core Problem & Value Proposition
Target users and core problems solved.
"#
    );
    fs::write(target_path.join("product_vision.md"), vision_content)?;

    // roadmap.md
    let roadmap_content = format!(
        r#"# Roadmap: {product_name}

- **Now:** Initial project setup and architecture
- **Next:** Feature exploration
- **Later:** Long-term horizons
"#
    );
    fs::write(target_path.join("roadmap.md"), roadmap_content)?;

    // glossary.md
    let glossary_content = format!(
        r#"# Glossary: {product_name}

## Terms
- **SCPE:** Spec-Compiled Product Engineering
- **Inception Studio:** Conversational product inception environment
"#
    );
    fs::write(target_path.join("glossary.md"), glossary_content)?;

    // architecture.md
    let arch_content = format!(
        r#"# Architecture: {product_name}

## 1. Context
System context and boundaries.

## 2. Containers & Components
Core component layout.
"#
    );
    fs::write(target_path.join("architecture.md"), arch_content)?;

    // technical_deal.md
    let tech_content = format!(
        r#"# Technical Deal: {product_name}

## 1. Approved Stack
Runtime, frameworks, and tooling.

## 2. Architectural Guardrails
Quality standards and linters.
"#
    );
    fs::write(target_path.join("technical_deal.md"), tech_content)?;

    // team_playbook.md
    let playbook_content = format!(
        r#"# Team Playbook: {product_name}

## Workflow & Git Conventions
Standard operating procedures and commit discipline.
"#
    );
    fs::write(target_path.join("team_playbook.md"), playbook_content)?;

    // quick_status.md
    let status_content = format!(
        r#"# Status: {product_name}

- **summary:** Newly scaffolded SCPE workspace.

## Blockers

None.
"#
    );
    fs::write(target_path.join("quick_status.md"), status_content)?;

    // CLAUDE.md
    let claude_content = format!(
        r#"# Instructions for AI Agents in {product_name}

This project is governed by SCPE v0.3.0.
"#
    );
    fs::write(target_path.join("CLAUDE.md"), claude_content)?;

    Ok(())
}
