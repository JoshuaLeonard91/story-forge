---
description: Add new character with guided wizard (name, role, personality, backstory, physical description)
allowed-tools: ["mcp__story-db__addCharacter"]
---

# Character Creation Wizard

I'll help you add a new character to your story project.

## Required Information

**1. Character Name:** What's the character's name?

**2. Character Role:**
1. Protagonist - Main hero of the story
2. Antagonist - Primary opposition/villain
3. Supporting - Important secondary character
4. Minor - Background character

## Optional Details (highly recommended)

**3. Personality Traits:** Describe their personality (brave, cunning, compassionate, etc.)

**4. Physical Description:** Appearance details (height, build, hair, eyes, distinctive features)

**5. Backstory:** Character's history and background (upbringing, past events, motivations)

**6. Current State:** Initial state when story begins (emotional state, location, condition)

---

After you provide these details, I will:
1. Call `mcp__story-db__addCharacter` to save the character to your project database
2. Display the character ID and summary
3. Suggest next steps (add relationships, create character arc)

**Note**: Character names must be unique within each project.

Please provide the character information above.
