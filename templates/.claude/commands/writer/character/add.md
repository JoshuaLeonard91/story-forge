---
description: Add new character with guided wizard
allowed-tools: ["mcp__story-db__addCharacter", "mcp__story-db__listStoryProjects"]
---

You are helping the user add a new character to their story. This wizard will guide them through creating a detailed character profile.

## FIRST: Verify MCP Server

**Immediately call the MCP tool** `mcp__story-db__listStoryProjects` with an empty params object `{}`.

DO NOT use Bash commands to check availability. Just call the tool directly.

**If the tool call succeeds:**
Proceed with the wizard below (use the returned projects list).

**If the tool call fails with an error:**
Stop and tell them:

```
The Story Forge MCP server is not configured yet. Please run /writer.mcp.setup first to configure the server, then restart Claude Code.

After restarting, run /writer.character.add again.
```

---

## Interactive Wizard (Only if MCP server is available)

**Step 1:** Get available projects by calling `mcp__story-db__listStoryProjects`

**Step 2:** Determine which project to use:
- If NO projects exist: Tell user to run /writer.start first and STOP
- If ONE project exists: Use that project automatically (save the project_id)
- If MULTIPLE projects exist: Ask user to select which project (use AskUserQuestion with project list)

**Step 3:** Call AskUserQuestion with this EXACT configuration:

```json
{
  "questions": [
    {
      "question": "What's the character's name?",
      "header": "Name",
      "options": [
        {"label": "Enter name", "description": "Character's full name"}
      ],
      "multiSelect": false
    },
    {
      "question": "What role does this character play?",
      "header": "Role",
      "options": [
        {"label": "Protagonist", "description": "Main hero"},
        {"label": "Antagonist", "description": "Primary villain"},
        {"label": "Supporting", "description": "Secondary character"},
        {"label": "Minor", "description": "Background character"}
      ],
      "multiSelect": false
    },
    {
      "question": "Describe their personality",
      "header": "Personality",
      "options": [
        {"label": "Enter personality", "description": "Traits, demeanor, quirks"}
      ],
      "multiSelect": false
    },
    {
      "question": "What do they look like?",
      "header": "Appearance",
      "options": [
        {"label": "Enter appearance", "description": "Height, build, hair, eyes, features"}
      ],
      "multiSelect": false
    }
  ]
}
```

**Step 4:** Wait for user to submit the form.

**Step 5:** Extract answers:
- Name: answers["0"] (Other text)
- Role: answers["1"]
- Personality: answers["2"] (Other text)
- Appearance: answers["3"] (Other text)

**Step 6:** Create the character:

```
Call mcp__story-db__addCharacter with:
{
  "story_project_id": <project_id from step 2>,
  "name": <name from answers>,
  "role": <role lowercase, e.g. "protagonist">,
  "personality_traits": <personality from answers>,
  "physical_description": <appearance from answers>,
  "backstory": "",
  "current_state": ""
}
```

**Step 7:** Confirm success and show:
- Character "[name]" added successfully
- Next steps: /writer.character.add (more characters), /writer.world.rule

CRITICAL RULES:
- MUST check project availability FIRST
- MUST use AskUserQuestion EXACTLY as shown
- MUST wait for form submission
- NO back-and-forth - collect ALL at once
