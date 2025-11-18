use rusqlite::Connection;
use anyhow::Result;

pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        -- Story Projects table
        CREATE TABLE IF NOT EXISTS story_projects (
            id TEXT PRIMARY KEY NOT NULL,
            title TEXT NOT NULL,
            genre TEXT,
            intended_length TEXT NOT NULL CHECK(intended_length IN ('short_story', 'novella', 'novel', 'series')),
            description TEXT,
            status TEXT NOT NULL DEFAULT 'draft' CHECK(status IN ('draft', 'in_progress', 'complete', 'archived')),
            word_count INTEGER NOT NULL DEFAULT 0,
            metadata TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            UNIQUE(title)
        );

        -- Characters table
        CREATE TABLE IF NOT EXISTS characters (
            id TEXT PRIMARY KEY NOT NULL,
            story_project_id TEXT NOT NULL,
            name TEXT NOT NULL,
            role TEXT NOT NULL CHECK(role IN ('protagonist', 'antagonist', 'supporting', 'minor')),
            personality_traits TEXT,
            physical_description TEXT,
            backstory TEXT,
            current_state TEXT,
            first_appearance_scene_id TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (story_project_id) REFERENCES story_projects(id) ON DELETE CASCADE,
            FOREIGN KEY (first_appearance_scene_id) REFERENCES scenes(id) ON DELETE SET NULL,
            UNIQUE(story_project_id, name)
        );

        CREATE INDEX IF NOT EXISTS idx_characters_project ON characters(story_project_id);
        CREATE INDEX IF NOT EXISTS idx_characters_role ON characters(story_project_id, role);

        -- Character Relationships table
        CREATE TABLE IF NOT EXISTS character_relationships (
            id TEXT PRIMARY KEY NOT NULL,
            source_character_id TEXT NOT NULL,
            target_character_id TEXT NOT NULL,
            relationship_type TEXT NOT NULL CHECK(relationship_type IN ('ally', 'enemy', 'family', 'romantic', 'mentor', 'rival', 'neutral', 'unknown')),
            description TEXT,
            strength INTEGER CHECK(strength BETWEEN 1 AND 10),
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (source_character_id) REFERENCES characters(id) ON DELETE CASCADE,
            FOREIGN KEY (target_character_id) REFERENCES characters(id) ON DELETE CASCADE,
            CHECK(source_character_id != target_character_id),
            UNIQUE(source_character_id, target_character_id)
        );

        -- World Rules table
        CREATE TABLE IF NOT EXISTS world_rules (
            id TEXT PRIMARY KEY NOT NULL,
            story_project_id TEXT NOT NULL,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            scope TEXT NOT NULL CHECK(scope IN ('universal', 'regional', 'situational')),
            examples TEXT,
            keywords TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (story_project_id) REFERENCES story_projects(id) ON DELETE CASCADE,
            UNIQUE(story_project_id, name)
        );

        CREATE INDEX IF NOT EXISTS idx_world_rules_project ON world_rules(story_project_id);

        -- Plot Structure table
        CREATE TABLE IF NOT EXISTS plot_structures (
            id TEXT PRIMARY KEY NOT NULL,
            story_project_id TEXT NOT NULL UNIQUE,
            structure_type TEXT NOT NULL DEFAULT 'three_act' CHECK(structure_type IN ('three_act', 'five_act', 'hero_journey', 'custom')),
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (story_project_id) REFERENCES story_projects(id) ON DELETE CASCADE
        );

        -- Acts table
        CREATE TABLE IF NOT EXISTS acts (
            id TEXT PRIMARY KEY NOT NULL,
            plot_structure_id TEXT NOT NULL,
            name TEXT NOT NULL,
            position INTEGER NOT NULL,
            description TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (plot_structure_id) REFERENCES plot_structures(id) ON DELETE CASCADE,
            UNIQUE(plot_structure_id, position)
        );

        CREATE INDEX IF NOT EXISTS idx_acts_plot ON acts(plot_structure_id);

        -- Chapters table
        CREATE TABLE IF NOT EXISTS chapters (
            id TEXT PRIMARY KEY NOT NULL,
            act_id TEXT NOT NULL,
            title TEXT,
            number INTEGER NOT NULL,
            position INTEGER NOT NULL,
            status TEXT NOT NULL DEFAULT 'planned' CHECK(status IN ('planned', 'draft', 'complete', 'needs_revision')),
            summary TEXT,
            word_count INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (act_id) REFERENCES acts(id) ON DELETE CASCADE,
            UNIQUE(act_id, number)
        );

        CREATE INDEX IF NOT EXISTS idx_chapters_act ON chapters(act_id);

        -- Scenes table
        CREATE TABLE IF NOT EXISTS scenes (
            id TEXT PRIMARY KEY NOT NULL,
            chapter_id TEXT NOT NULL,
            title TEXT,
            position INTEGER NOT NULL,
            location TEXT,
            time_description TEXT,
            content TEXT NOT NULL,
            word_count INTEGER NOT NULL DEFAULT 0,
            status TEXT NOT NULL DEFAULT 'planned' CHECK(status IN ('planned', 'draft', 'complete', 'needs_revision')),
            scene_outline TEXT,
            ai_generated INTEGER NOT NULL DEFAULT 0,
            summary TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (chapter_id) REFERENCES chapters(id) ON DELETE CASCADE,
            UNIQUE(chapter_id, position)
        );

        CREATE INDEX IF NOT EXISTS idx_scenes_chapter ON scenes(chapter_id);
        CREATE INDEX IF NOT EXISTS idx_scenes_position ON scenes(chapter_id, position);

        -- Scene Characters junction table
        CREATE TABLE IF NOT EXISTS scene_characters (
            id TEXT PRIMARY KEY NOT NULL,
            scene_id TEXT NOT NULL,
            character_id TEXT NOT NULL,
            role_in_scene TEXT NOT NULL DEFAULT 'active' CHECK(role_in_scene IN ('protagonist', 'active', 'mentioned', 'background')),
            FOREIGN KEY (scene_id) REFERENCES scenes(id) ON DELETE CASCADE,
            FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
            UNIQUE(scene_id, character_id)
        );

        CREATE INDEX IF NOT EXISTS idx_scene_characters_scene ON scene_characters(scene_id);
        CREATE INDEX IF NOT EXISTS idx_scene_characters_char ON scene_characters(character_id);

        -- Character Arcs table
        CREATE TABLE IF NOT EXISTS character_arcs (
            id TEXT PRIMARY KEY NOT NULL,
            story_project_id TEXT NOT NULL,
            character_id TEXT NOT NULL,
            arc_name TEXT NOT NULL,
            start_state TEXT NOT NULL,
            end_state TEXT NOT NULL,
            current_progress INTEGER NOT NULL DEFAULT 0 CHECK(current_progress BETWEEN 0 AND 100),
            status TEXT NOT NULL DEFAULT 'planned' CHECK(status IN ('planned', 'in_progress', 'complete', 'abandoned')),
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (story_project_id) REFERENCES story_projects(id) ON DELETE CASCADE,
            FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_arcs_project ON character_arcs(story_project_id);
        CREATE INDEX IF NOT EXISTS idx_arcs_character ON character_arcs(character_id);

        -- Arc Milestones table
        CREATE TABLE IF NOT EXISTS arc_milestones (
            id TEXT PRIMARY KEY NOT NULL,
            character_arc_id TEXT NOT NULL,
            position INTEGER NOT NULL,
            description TEXT NOT NULL,
            target_chapter INTEGER,
            completed INTEGER NOT NULL DEFAULT 0,
            completed_at_scene_id TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (character_arc_id) REFERENCES character_arcs(id) ON DELETE CASCADE,
            FOREIGN KEY (completed_at_scene_id) REFERENCES scenes(id) ON DELETE SET NULL,
            UNIQUE(character_arc_id, position)
        );

        -- Scene Milestones junction table
        CREATE TABLE IF NOT EXISTS scene_milestones (
            id TEXT PRIMARY KEY NOT NULL,
            scene_id TEXT NOT NULL,
            arc_milestone_id TEXT NOT NULL,
            FOREIGN KEY (scene_id) REFERENCES scenes(id) ON DELETE CASCADE,
            FOREIGN KEY (arc_milestone_id) REFERENCES arc_milestones(id) ON DELETE CASCADE,
            UNIQUE(scene_id, arc_milestone_id)
        );

        -- Story Summaries table
        CREATE TABLE IF NOT EXISTS story_summaries (
            id TEXT PRIMARY KEY NOT NULL,
            story_project_id TEXT NOT NULL,
            scope TEXT NOT NULL CHECK(scope IN ('scene', 'chapter', 'act', 'overall')),
            reference_id TEXT,
            summary_text TEXT NOT NULL,
            key_events TEXT,
            character_developments TEXT,
            word_count INTEGER NOT NULL DEFAULT 0,
            generated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            ai_generated INTEGER NOT NULL DEFAULT 1,
            FOREIGN KEY (story_project_id) REFERENCES story_projects(id) ON DELETE CASCADE,
            UNIQUE(story_project_id, scope, reference_id)
        );

        CREATE INDEX IF NOT EXISTS idx_summaries_project ON story_summaries(story_project_id);
        CREATE INDEX IF NOT EXISTS idx_summaries_scope ON story_summaries(story_project_id, scope);

        -- Continuity Alerts table
        CREATE TABLE IF NOT EXISTS continuity_alerts (
            id TEXT PRIMARY KEY NOT NULL,
            story_project_id TEXT NOT NULL,
            scene_id TEXT,
            alert_type TEXT NOT NULL CHECK(alert_type IN ('world_rule_violation', 'character_state_conflict', 'timeline_contradiction', 'factual_inconsistency')),
            severity TEXT NOT NULL DEFAULT 'medium' CHECK(severity IN ('low', 'medium', 'high')),
            description TEXT NOT NULL,
            conflicting_elements TEXT,
            suggested_resolution TEXT,
            author_decision TEXT NOT NULL DEFAULT 'pending' CHECK(author_decision IN ('pending', 'revised_content', 'updated_fact', 'dismissed')),
            author_notes TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            resolved_at TEXT,
            FOREIGN KEY (story_project_id) REFERENCES story_projects(id) ON DELETE CASCADE,
            FOREIGN KEY (scene_id) REFERENCES scenes(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_alerts_project ON continuity_alerts(story_project_id);
        CREATE INDEX IF NOT EXISTS idx_alerts_decision ON continuity_alerts(story_project_id, author_decision);
        CREATE INDEX IF NOT EXISTS idx_alerts_scene ON continuity_alerts(scene_id);

        -- Character State History table
        CREATE TABLE IF NOT EXISTS character_state_history (
            id TEXT PRIMARY KEY NOT NULL,
            character_id TEXT NOT NULL,
            scene_id TEXT NOT NULL,
            state_snapshot TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
            FOREIGN KEY (scene_id) REFERENCES scenes(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_state_history_char ON character_state_history(character_id);
        CREATE INDEX IF NOT EXISTS idx_state_history_scene ON character_state_history(scene_id);

        -- Progression Systems table (optional features)
        CREATE TABLE IF NOT EXISTS progression_systems (
            id TEXT PRIMARY KEY NOT NULL,
            story_project_id TEXT NOT NULL,
            system_name TEXT NOT NULL,
            system_type TEXT NOT NULL CHECK(system_type IN ('game_stats', 'cultivation', 'magic_tiers', 'skill_trees', 'custom')),
            template_data TEXT,
            enabled INTEGER NOT NULL DEFAULT 1,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (story_project_id) REFERENCES story_projects(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_progression_project ON progression_systems(story_project_id);
        "#
    )?;

    // Create FTS5 virtual tables for full-text search
    conn.execute_batch(
        r#"
        -- FTS5 for character search
        CREATE VIRTUAL TABLE IF NOT EXISTS characters_fts USING fts5(
            character_id UNINDEXED,
            name,
            personality_traits,
            physical_description,
            backstory,
            content='characters',
            content_rowid='rowid'
        );

        -- FTS5 for world rules search
        CREATE VIRTUAL TABLE IF NOT EXISTS world_rules_fts USING fts5(
            rule_id UNINDEXED,
            name,
            description,
            examples,
            keywords,
            content='world_rules',
            content_rowid='rowid'
        );

        -- FTS5 for scene content search
        CREATE VIRTUAL TABLE IF NOT EXISTS scenes_fts USING fts5(
            scene_id UNINDEXED,
            content,
            scene_outline,
            summary,
            content='scenes',
            content_rowid='rowid'
        );

        -- FTS5 for story summaries search
        CREATE VIRTUAL TABLE IF NOT EXISTS summaries_fts USING fts5(
            summary_id UNINDEXED,
            summary_text,
            content='story_summaries',
            content_rowid='rowid'
        );
        "#
    )?;

    log::info!("Database migrations completed successfully");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_migrations_run_successfully() {
        let conn = Connection::open_in_memory().unwrap();
        let result = run_migrations(&conn);
        assert!(result.is_ok());

        // Verify a few key tables exist
        let table_exists: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='story_projects'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(table_exists, 1);
    }
}
