# Plan: Interactive Roadmap Visualizer & Conversational Adjuster

## Intent
Provide a visual Now / Next / Later grid view of `roadmap.md` and enable conversational co-pilot adjustments where the human can reorder features, add milestones, or shift horizons naturally through dialogue.

## Domain Model
- **Terms:** Interactive Roadmap, Two-Way Sync
- **Entities:** RoadmapHorizon, RoadmapFeatureItem, RoadmapModifier
- **Domain events:** RoadmapHorizonReordered, RoadmapFileUpdated

## Business Rules
- **R1:** Every modification to the visual roadmap or chat-instructed reorganization must persist atomically to roadmap.md on disk.
- **R2:** Features cannot be orphaned without an assigned horizon (Now, Next, or Later).

## Examples
- **S1 — Visual render of roadmap horizons:** Given a valid `roadmap.md` file on disk, When the user opens the Roadmap Studio tab, Then features are parsed and displayed into distinct interactive columns (Now, Next, Later).
- **S2 — Conversational horizon shift:** Given a feature in the Later column, When the user tells the co-pilot "Move payment checkout to Now", Then the co-pilot updates `roadmap.md` and the visual column updates immediately.

## Slices
1. Visual Now/Next/Later grid and conversational roadmap updater — S1, S2, R1, R2

## Open Questions
- None. [resolved]
