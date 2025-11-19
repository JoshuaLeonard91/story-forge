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

**Step 3:** Call AskUserQuestion with this EXACT configuration (all questions in one form):

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
        {"label": "Protagonist", "description": "Main hero of the story"},
        {"label": "Antagonist", "description": "Primary villain or opposing force"},
        {"label": "Supporting", "description": "Secondary character with important role"},
        {"label": "Minor", "description": "Background or minor character"}
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
        {"label": "Enter appearance", "description": "Height, build, hair, eyes, distinctive features"}
      ],
      "multiSelect": false
    }
  ]
}
```

**Step 4:** Wait for user to submit the form with all their answers.

**Step 5:** Extract answers:
- Name: answers["0"] - Use "Other" text input
- Role: answers["1"] - Can be preset option or "Other"
- Personality: answers["2"] - Use "Other" text input
- Appearance: answers["3"] - Use "Other" text input

**Step 6:** Show confirmation summary and ask for approval:

Display summary (plain text, no markdown bold or emojis):
```
Character Summary
=================

Name: {name}
Role: {role}
Personality: {personality}
Appearance: {appearance}

This character will be added to your story project.
```

Then call AskUserQuestion:
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

**Step 7:** Create the character by calling `mcp__story-db__addCharacter`:

```json
{
  "projectId": <projectId from step 2>,
  "name": <name from step 5>,
  "role": <role from step 5, convert to lowercase>,
  "personalityTraits": <personality from step 5>,
  "physicalDescription": <appearance from step 5>,
  "backstory": "",
  "currentState": ""
}
```

**Step 8:** Confirm success (plain text, no markdown bold or emojis):

```
Character added successfully!

Character Details:
- Name: <name>
- Role: <role>
- Project: <project title>

Next steps:
- /writer:character:add - Add more characters
- /writer:world:rule - Define world-building rules
- Start writing scenes with your characters
```

CRITICAL RULES:
- MUST check project availability FIRST
- MUST use ONE AskUserQuestion call with ALL character questions in a single form
- User fills out entire form and submits ONCE
- MUST use AskUserQuestion for confirmation
- MUST use camelCase parameter names (projectId, personalityTraits, physicalDescription, currentState)
- NO emojis or markdown bold in output
