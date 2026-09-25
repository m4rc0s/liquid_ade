use crate::workspace::{self, PathGuardError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ScpeOutline {
    pub features: Vec<FeatureOutline>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct FeatureOutline {
    pub slug: String,
    pub title: String,
    pub path: String,
    pub epics: Vec<EpicOutline>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct EpicOutline {
    pub slug: String,
    pub title: String,
    pub state: String,
    pub tasks_done: usize,
    pub tasks_total: usize,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EpicDetailResponse {
    pub feature: String,
    pub epic: String,
    pub title: String,
    pub state: String,
    pub intent: String,
    pub domain_model: DomainModelDetail,
    pub rules: Vec<RuleDetail>,
    pub examples: Vec<ExampleDetail>,
    pub slices: Vec<SliceDetail>,
    pub open_questions: Vec<String>,
    pub tasks: Vec<TaskDetail>,
    pub raw_sections: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct DomainModelDetail {
    pub terms: Vec<String>,
    pub entities: Vec<String>,
    pub events: Vec<String>,
    pub raw: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct RuleDetail {
    pub id: String,
    pub title: Option<String>,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ExampleDetail {
    pub id: String,
    pub title: String,
    pub given: String,
    pub when: String,
    pub then: String,
    pub raw: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SliceDetail {
    pub number: usize,
    pub title: String,
    pub citations: Vec<String>,
    pub raw: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct TaskDetail {
    pub done: bool,
    pub label: String,
    pub text: String,
    pub examples: Vec<String>,
    pub rules: Vec<String>,
}

/// Extracts state from quick_status.md without coercion (S4).
pub fn parse_quick_status_state(content: &str) -> String {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("- **state:**") {
            let val = trimmed.trim_start_matches("- **state:**").trim();
            if !val.is_empty() {
                return val.to_string();
            }
        }
        if trimmed.starts_with("state:") {
            let val = trimmed.trim_start_matches("state:").trim();
            if !val.is_empty() {
                return val.to_string();
            }
        }
    }
    "Draft".to_string()
}

/// Counts done and total tasks in tasks.md.
pub fn parse_task_counts(content: &str) -> (usize, usize) {
    let mut done = 0;
    let mut total = 0;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]") {
            done += 1;
            total += 1;
        } else if trimmed.starts_with("- [ ]") {
            total += 1;
        }
    }
    (done, total)
}

/// Extracts first markdown heading title (# ...).
pub fn extract_h1_title(content: &str, fallback: &str) -> String {
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("# ") {
            let title = trimmed.trim_start_matches("# ").trim();
            // Strip leading "Plan: ", "Status: ", "Tasks: ", "Feature: "
            for prefix in &["Plan:", "Status:", "Tasks:", "Feature:", "Epic:"] {
                if let Some(stripped) = title.strip_prefix(prefix) {
                    return stripped.trim().to_string();
                }
            }
            return title.to_string();
        }
    }
    fallback.to_string()
}

/// Parses tasks.md into structured TaskDetails with badge parsing (S5).
pub fn parse_tasks(content: &str) -> Vec<TaskDetail> {
    let mut tasks = Vec::new();
    for line in content.lines() {
        let trimmed = line.trim();
        let (done, rest) = if let Some(r) = trimmed.strip_prefix("- [x] ") {
            (true, r)
        } else if let Some(r) = trimmed.strip_prefix("- [X] ") {
            (true, r)
        } else if let Some(r) = trimmed.strip_prefix("- [ ] ") {
            (false, r)
        } else {
            continue;
        };

        let (label, rest_text) = if let Some((lbl, text)) = rest.split_once(':') {
            (lbl.trim().to_string(), text.trim().to_string())
        } else {
            ("TASK".to_string(), rest.to_string())
        };

        // Extract badge metadata like (S1, S2, R1)
        let mut examples = Vec::new();
        let mut rules = Vec::new();
        let clean_text = if let Some(open_paren) = rest_text.rfind('(') {
            if rest_text.ends_with(')') {
                let badge_content = &rest_text[open_paren + 1..rest_text.len() - 1];
                for token in badge_content.split(',') {
                    let t = token.trim();
                    if t.starts_with('S') {
                        examples.push(t.to_string());
                    } else if t.starts_with('R') {
                        rules.push(t.to_string());
                    }
                }
                rest_text[..open_paren].trim().to_string()
            } else {
                rest_text
            }
        } else {
            rest_text
        };

        tasks.push(TaskDetail {
            done,
            label,
            text: clean_text,
            examples,
            rules,
        });
    }
    tasks
}

/// Parses sections from plan.md into key-value map by ## Heading.
pub fn parse_sections(content: &str) -> HashMap<String, String> {
    let mut sections = HashMap::new();
    let mut current_heading: Option<String> = None;
    let mut current_lines = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("## ") {
            if let Some(h) = current_heading.take() {
                sections.insert(h, current_lines.join("\n").trim().to_string());
                current_lines.clear();
            }
            let heading = trimmed.trim_start_matches("## ").trim().to_string();
            current_heading = Some(heading);
        } else if current_heading.is_some() {
            current_lines.push(line);
        }
    }

    if let Some(h) = current_heading {
        sections.insert(h, current_lines.join("\n").trim().to_string());
    }

    sections
}

/// Parses rules from ## Business Rules section.
pub fn parse_rules(body: &str) -> Vec<RuleDetail> {
    let mut rules = Vec::new();
    for line in body.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("- **") {
            if let Some((id_part, desc_part)) = rest.split_once("**") {
                let id = id_part.trim().trim_end_matches(':').trim().to_string();
                let desc = desc_part.trim().trim_start_matches(':').trim().to_string();
                rules.push(RuleDetail {
                    id,
                    title: None,
                    description: desc,
                });
            }
        }
    }
    rules
}

/// Parses examples from ## Examples section.
pub fn parse_examples(body: &str) -> Vec<ExampleDetail> {
    let mut examples = Vec::new();
    for line in body.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("- **") {
            if let Some((tag_part, desc_part)) = rest.split_once("**") {
                let tag = tag_part.trim().trim_end_matches(':').trim();
                let (id, title) = if let Some((id_str, title_str)) = tag.split_once('—') {
                    (id_str.trim().to_string(), title_str.trim().to_string())
                } else if let Some((id_str, title_str)) = tag.split_once('-') {
                    (id_str.trim().to_string(), title_str.trim().to_string())
                } else {
                    (tag.to_string(), String::new())
                };

                let desc = desc_part.trim().trim_start_matches(':').trim();
                let mut given = String::new();
                let mut when = String::new();
                let mut then = String::new();

                if let Some(given_idx) = desc.find("Given ") {
                    if let Some(when_idx) = desc.find(", When ") {
                        given = desc[given_idx + 6..when_idx].trim().to_string();
                        if let Some(then_idx) = desc.find(", Then ") {
                            when = desc[when_idx + 7..then_idx].trim().to_string();
                            then = desc[then_idx + 7..].trim().to_string();
                        }
                    }
                }

                examples.push(ExampleDetail {
                    id,
                    title,
                    given,
                    when,
                    then,
                    raw: desc.to_string(),
                });
            }
        }
    }
    examples
}

/// Parses slices from ## Slices section.
pub fn parse_slices(body: &str) -> Vec<SliceDetail> {
    let mut slices = Vec::new();
    for line in body.lines() {
        let trimmed = line.trim();
        if let Some(num_end) = trimmed.find('.') {
            let num_str = &trimmed[..num_end];
            if let Ok(num) = num_str.parse::<usize>() {
                let rest = trimmed[num_end + 1..].trim();
                let mut citations = Vec::new();
                let title = if let Some(dash_idx) = rest.rfind('—') {
                    let cite_part = &rest[dash_idx + 3..];
                    for t in cite_part.split(',') {
                        let token = t.trim();
                        if !token.is_empty() {
                            citations.push(token.to_string());
                        }
                    }
                    rest[..dash_idx].trim().to_string()
                } else if let Some(dash_idx) = rest.rfind(" - ") {
                    let cite_part = &rest[dash_idx + 3..];
                    for t in cite_part.split(',') {
                        let token = t.trim();
                        if !token.is_empty() {
                            citations.push(token.to_string());
                        }
                    }
                    rest[..dash_idx].trim().to_string()
                } else {
                    rest.to_string()
                };

                slices.push(SliceDetail {
                    number: num,
                    title,
                    citations,
                    raw: trimmed.to_string(),
                });
            }
        }
    }
    slices
}

/// Scans features/ and constructs the ScpeOutline projection (S1).
pub fn build_scpe_outline(root: &Path) -> Result<ScpeOutline, PathGuardError> {
    let features_dir = root.join("features");
    if !features_dir.exists() || !features_dir.is_dir() {
        return Ok(ScpeOutline {
            features: Vec::new(),
        });
    }

    let mut feature_dirs: Vec<PathBuf> = std::fs::read_dir(&features_dir)
        .map_err(|e| PathGuardError::Io(e.to_string()))?
        .filter_map(|res| res.ok().map(|e| e.path()))
        .filter(|p| p.is_dir())
        .collect();

    feature_dirs.sort();

    let mut features = Vec::new();

    for fpath in feature_dirs {
        let slug = fpath
            .file_name()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        if slug.starts_with('.') {
            continue;
        }

        let f_index = fpath.join("index.md");
        let title = if f_index.exists() {
            let content = std::fs::read_to_string(&f_index).unwrap_or_default();
            extract_h1_title(&content, &slug)
        } else {
            slug.clone()
        };

        let epics_dir = fpath.join("epics");
        let mut epics = Vec::new();

        if epics_dir.exists() && epics_dir.is_dir() {
            let mut epic_dirs: Vec<PathBuf> = std::fs::read_dir(&epics_dir)
                .map_err(|e| PathGuardError::Io(e.to_string()))?
                .filter_map(|res| res.ok().map(|e| e.path()))
                .filter(|p| p.is_dir())
                .collect();

            epic_dirs.sort();

            for epath in epic_dirs {
                let e_slug = epath
                    .file_name()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_default();
                if e_slug.starts_with('.') {
                    continue;
                }

                let e_plan = epath.join("plan.md");
                let e_status = epath.join("quick_status.md");
                let e_tasks = epath.join("tasks.md");

                let e_title = if e_plan.exists() {
                    let content = std::fs::read_to_string(&e_plan).unwrap_or_default();
                    extract_h1_title(&content, &e_slug)
                } else {
                    e_slug.clone()
                };

                let state = if e_status.exists() {
                    let content = std::fs::read_to_string(&e_status).unwrap_or_default();
                    parse_quick_status_state(&content)
                } else {
                    "Draft".to_string()
                };

                let (tasks_done, tasks_total) = if e_tasks.exists() {
                    let content = std::fs::read_to_string(&e_tasks).unwrap_or_default();
                    parse_task_counts(&content)
                } else {
                    (0, 0)
                };

                let canonical_doc_path = format!("/features/{slug}/epics/{e_slug}/plan.md");

                epics.push(EpicOutline {
                    slug: e_slug,
                    title: e_title,
                    state,
                    tasks_done,
                    tasks_total,
                    path: canonical_doc_path,
                });
            }
        }

        let canonical_feat_path = format!("/features/{slug}/index.md");
        features.push(FeatureOutline {
            slug,
            title,
            path: canonical_feat_path,
            epics,
        });
    }

    Ok(ScpeOutline { features })
}

/// Reads and parses an epic's plan.md, tasks.md, and quick_status.md into EpicDetailResponse (S2).
pub fn read_epic_detail(
    root: &Path,
    feature: &str,
    epic: &str,
) -> Result<EpicDetailResponse, PathGuardError> {
    // Path traversal check
    if feature.contains("..") || epic.contains("..") {
        return Err(PathGuardError::TraversalDetected);
    }

    let rel_epic_path = format!("features/{feature}/epics/{epic}");
    let epic_dir = workspace::PathGuard::resolve_safe_path(root, &rel_epic_path)?;

    if !epic_dir.exists() || !epic_dir.is_dir() {
        return Err(PathGuardError::NotFound);
    }

    let plan_path = epic_dir.join("plan.md");
    let status_path = epic_dir.join("quick_status.md");
    let tasks_path = epic_dir.join("tasks.md");

    let plan_content = std::fs::read_to_string(&plan_path).unwrap_or_default();
    let status_content = std::fs::read_to_string(&status_path).unwrap_or_default();
    let tasks_content = std::fs::read_to_string(&tasks_path).unwrap_or_default();

    let title = extract_h1_title(&plan_content, epic);
    let state = parse_quick_status_state(&status_content);
    let tasks = parse_tasks(&tasks_content);

    let raw_sections = parse_sections(&plan_content);
    let intent = raw_sections.get("Intent").cloned().unwrap_or_default();

    let domain_model_body = raw_sections
        .get("Domain Model")
        .cloned()
        .unwrap_or_default();
    let mut terms = Vec::new();
    let mut entities = Vec::new();
    let mut events = Vec::new();

    for line in domain_model_body.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("- **Terms:**") {
            for t in rest.split(',') {
                let term = t.trim();
                if !term.is_empty() {
                    terms.push(term.to_string());
                }
            }
        } else if let Some(rest) = trimmed.strip_prefix("- **Entities:**") {
            for t in rest.split(',') {
                let ent = t.trim();
                if !ent.is_empty() {
                    entities.push(ent.to_string());
                }
            }
        } else if let Some(rest) = trimmed.strip_prefix("- **Domain events:**") {
            for t in rest.split(',') {
                let ev = t.trim();
                if !ev.is_empty() {
                    events.push(ev.to_string());
                }
            }
        }
    }

    let domain_model = DomainModelDetail {
        terms,
        entities,
        events,
        raw: domain_model_body,
    };

    let rules = raw_sections
        .get("Business Rules")
        .map(|b| parse_rules(b))
        .unwrap_or_default();

    let examples = raw_sections
        .get("Examples")
        .map(|b| parse_examples(b))
        .unwrap_or_default();

    let slices = raw_sections
        .get("Slices")
        .map(|b| parse_slices(b))
        .unwrap_or_default();

    let open_questions = raw_sections
        .get("Open Questions")
        .map(|b| {
            b.lines()
                .map(|l| l.trim().trim_start_matches("- ").to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default();

    Ok(EpicDetailResponse {
        feature: feature.to_string(),
        epic: epic.to_string(),
        title,
        state,
        intent,
        domain_model,
        rules,
        examples,
        slices,
        open_questions,
        tasks,
        raw_sections,
    })
}
