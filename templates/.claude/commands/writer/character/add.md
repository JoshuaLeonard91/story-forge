---
description: Add new character with guided wizard
---

You are helping the user add a new character to their story. This wizard will guide them through creating a detailed character profile.

## FIRST: Check if MCP Tools are Available

Look at your available tools list. Do you see any tools starting with `mcp__story-db__`?

**If NO** (you don't see any `mcp__story-db__` tools):
Stop and tell them the MCP server is not loaded. Refer them to /writer:start for troubleshooting steps.

**If YES** (MCP tools are available):
Proceed with the wizard below.

---

## Interactive Wizard

**Step 1:** Call `mcp__story-db__listStoryProjects` with empty params `{}`

**Step 2:** Determine which project to use:
- If NO projects exist: Tell user "No projects found. Please run /writer:start to create a project first." and STOP
- If ONE project exists: Use that project automatically (save the projectId), proceed to Step 3
- If MULTIPLE projects exist: Call AskUserQuestion to let them select project, then proceed to Step 3

**Step 3:** Ask for character name directly.

Say: "What's the character's name?"

Wait for the user to reply. Save this as `characterName`.

**Step 4:** Call AskUserQuestion with EXACTLY this JSON (do not modify):

```json
{
  "questions": [
    {
      "question": "What role does this character play?",
      "header": "Role",
      "options": [
        {"label": "Protagonist", "description": "Main hero of the story"},
        {"label": "Antagonist", "description": "Primary villain or opposing force"},
        {"label": "Supporting", "description": "Secondary character with important role"},
        {"label": "Minor", "description": "Background or minor character"}
      ],
      "multiSelect": false
    }
  ]
}
```

**Step 5:** Wait for user to submit the form.

**Step 6:** Ask for personality directly.

Say: "Describe their personality (traits, demeanor, quirks):"

Wait for the user to reply. Save this as `personality`.

**Step 7:** Ask for appearance directly.

Say: "What do they look like? (height, build, hair, eyes, distinctive features):"

Wait for the user to reply. Save this as `appearance`.

**Step 8:** Show confirmation summary and ask for approval:

Display summary (plain text, no markdown bold or emojis):
```
Character Summary
=================

Name: {characterName}
Role: {role}
Personality: {personality}
Appearance: {appearance}

This character will be added to your story project.
```

Then call AskUserQuestion with EXACTLY this JSON:
```json
{
  "questions": [
    {
      "question": "Add this character to your project?",
      "header": "Confirm",
      "options": [
        {"label": "Yes, add character", "description": "Create this character"},
        {"label": "No, cancel", "description": "Cancel character creation"}
      ],
      "multiSelect": false
    }
  ]
}
```

Wait for user to submit. If they select "No, cancel", stop and say "Character creation cancelled."

**Step 9:** Create the character by calling `mcp__story-db__addCharacter`:

```json
{
  "projectId": <projectId from step 2>,
  "name": <characterName from step 3>,
  "role": <role from step 5, convert to lowercase>,
  "personalityTraits": <personality from step 6>,
  "physicalDescription": <appearance from step 7>,
  "backstory": "",
  "currentState": ""
}
```

**Step 10:** Confirm success (plain text, no markdown bold or emojis):

```
Character added successfully!

Character Details:
- Name: <characterName>
- Role: <role>
- Project: <project title>

Next steps:
- /writer:character:add - Add more characters
- /writer:world:rule - Define world-building rules
- Start writing scenes with your characters
```

CRITICAL RULES:
- MUST check project availability FIRST
- MUST ask for name, personality, and appearance as plain text (NOT in forms)
- MUST use AskUserQuestion ONLY for role selection (has 4 options)
- MUST use AskUserQuestion for confirmation
- MUST use camelCase parameter names (projectId, personalityTraits, physicalDescription, currentState)
- NO emojis or markdown bold in output
