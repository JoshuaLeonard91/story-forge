---
description: Add new character with guided wizard
allowed-tools: ["mcp__story-db__addCharacter", "mcp__story-db__listStoryProjects"]
---

You are helping the user add a new character to their story. Guide them through these questions ONE AT A TIME:

1. **Project Selection** - First, check if they have multiple projects:
   - Call `mcp__story-db__listStoryProjects` to see available projects
   - If multiple exist, ask: "Which project is this character for?" and show the list
   - If only one exists, use that project automatically

2. **Character Name** - Ask: "What's the character's name?"

3. **Character Role** - After they provide the name, ask: "What role does this character play?" with options:
   - Protagonist (Main hero)
   - Antagonist (Primary villain/opposition)
   - Supporting (Important secondary character)
   - Minor (Background character)

4. **Personality Traits** - After they choose role, ask: "Describe their personality in a few words or sentences"
   (Examples: brave and strategic, cunning manipulator, compassionate healer, etc.)

5. **Physical Description** - After personality, ask: "What do they look like?"
   (Height, build, hair, eyes, distinctive features, clothing style)

6. **Backstory** - After physical description, ask: "What's their backstory?"
   (Upbringing, past events, what shaped them, their motivations)

7. **Current State** - Finally, ask: "What's their state at the beginning of the story?"
   (Emotional condition, where they are, their situation)

After gathering all information:
1. Show them a summary of the character
2. Ask "Does this look correct? (yes/no)"
3. If yes, call `mcp__story-db__addCharacter` with all the gathered details
4. Confirm success and suggest next steps:
   - Add character relationships with `/writer.character.relationship`
   - Add more characters with `/writer.character.add`
   - Start writing scenes that feature this character

IMPORTANT: Only ask ONE question at a time. Wait for their answer before proceeding to the next question.
