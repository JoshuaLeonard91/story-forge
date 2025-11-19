---
description: Define world rule with guided wizard
---

You are helping the user define a world rule for their story. World rules ensure consistency in worldbuilding.

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
      "question": "What should this rule be called?",
      "header": "Rule Name",
      "options": [
        {"label": "Enter rule name", "description": "Examples: 'Magic System: Mana Cost', 'Technology: No FTL Travel', 'Social Law: Noble Privileges'"}
      ],
      "multiSelect": false
    },
    {
      "question": "Explain how this rule works in detail:",
      "header": "Description",
      "options": [
        {"label": "Enter description", "description": "Detailed explanation of the rule and its implications"}
      ],
      "multiSelect": false
    },
    {
      "question": "What's the scope of this rule?",
      "header": "Scope",
      "options": [
        {"label": "Universal", "description": "Applies everywhere in your world"},
        {"label": "Regional", "description": "Only in certain locations"},
        {"label": "Situational", "description": "Only in specific circumstances"}
      ],
      "multiSelect": false
    }
  ]
}
```

**Step 4:** Wait for user to submit the form with all their answers.

**Step 5:** Extract answers:
- Rule Name: answers["0"] - Use "Other" text input
- Description: answers["1"] - Use "Other" text input
- Scope: answers["2"] - Can be preset option or "Other"

**Step 6:** Show confirmation summary and ask for approval:

Display summary (plain text, no markdown bold or emojis):
```
World Rule Summary
==================

Name: {ruleName}
Scope: {scope}

Description:
{description}

This rule will be added to your worldbuilding database for consistency checking.
```

Then call AskUserQuestion:
```json
{
  "questions": [
    {
      "question": "Add this world rule to your project?",
      "header": "Confirm",
      "options": [
        {"label": "Yes, add rule", "description": "Create this world rule"},
        {"label": "No, cancel", "description": "Cancel rule creation"}
      ],
      "multiSelect": false
    }
  ]
}
```

Wait for user to submit. If they select "No, cancel", stop and say "World rule creation cancelled."

**Step 7:** Create the world rule by calling `mcp__story-db__addWorldRule`:

```json
{
  "projectId": <projectId from step 2>,
  "ruleName": <ruleName from step 5>,
  "description": <description from step 5>,
  "scope": <scope from step 5, convert to lowercase>
}
```

**Step 8:** Confirm success (plain text, no markdown bold or emojis):

```
World rule added successfully!

Rule Details:
- Name: <ruleName>
- Scope: <scope>
- Project: <project title>

This rule will be used for context retrieval when writing scenes to maintain consistency.

Next steps:
- /writer:world:rule - Add more world rules (magic systems, technology, social structures)
- /writer:character:add - Add characters to your story
- Start writing scenes that follow these rules
```

CRITICAL RULES:
- MUST check project availability FIRST
- MUST use ONE AskUserQuestion call with ALL world rule questions in a single form
- User fills out entire form and submits ONCE
- MUST use AskUserQuestion for confirmation
- MUST use camelCase parameter names (projectId, ruleName)
- NO emojis or markdown bold in output
