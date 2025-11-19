---
description: Create new story project with guided wizard
allowed-tools: ["mcp__story-db__createStoryProject", "mcp__story-db__initializePlotStructure", "mcp__story-db__listStoryProjects"]
---

You are helping the user create a new story project. This wizard will guide them through setting up their story step-by-step.

## FIRST: Check MCP Server Availability

Before starting the wizard, try to call `mcp__story-db__listStoryProjects` to verify the MCP server is running.

**If the MCP server is NOT available (tool call fails):**
Stop immediately and tell them:

```
The Story Forge MCP server is not configured yet. Please run /writer.mcp.setup first to configure the server, then restart Claude Code.

After restarting, run /writer.start again to create your project.
```

Do NOT ask any questions if the MCP server isn't available.

**If the MCP server IS available:**
Proceed with the interactive wizard below.

---

## Interactive Wizard (Only if MCP server is available)

**Step 1:** Call the AskUserQuestion tool with this EXACT configuration:

```json
{
  "questions": [
    {
      "question": "What's the title of your story?",
      "header": "Title",
      "options": [
        {"label": "Enter title", "description": "Your story title"}
      ],
      "multiSelect": false
    },
    {
      "question": "What genre is this story?",
      "header": "Genre",
      "options": [
        {"label": "Epic Fantasy", "description": "Lord of the Rings, Wheel of Time"},
        {"label": "LitRPG", "description": "Solo Leveling, The Primal Hunter"},
        {"label": "Progression Fantasy", "description": "Cradle, Mother of Learning"},
        {"label": "Urban Fantasy", "description": "Dresden Files, modern magic"}
      ],
      "multiSelect": false
    },
    {
      "question": "What's your target length?",
      "header": "Length",
      "options": [
        {"label": "Short Story", "description": "Under 20,000 words"},
        {"label": "Novella", "description": "20,000-50,000 words"},
        {"label": "Novel", "description": "50,000-150,000 words"},
        {"label": "Series", "description": "150,000+ words, multiple books"}
      ],
      "multiSelect": false
    },
    {
      "question": "Which plot structure?",
      "header": "Plot",
      "options": [
        {"label": "Three-Act", "description": "Setup → Conflict → Resolution"},
        {"label": "Five-Act", "description": "Classic dramatic arc"},
        {"label": "Hero's Journey", "description": "Epic transformation"},
        {"label": "Custom", "description": "No predefined structure"}
      ],
      "multiSelect": false
    }
  ]
}
```

**Step 2:** Wait for user to submit the form with their answers.

**Step 3:** Extract answers:
- Title: answers["0"] (use Other text input)
- Genre: answers["1"]
- Length: answers["2"]
- Plot: answers["3"]

**Step 4:** Create the story project:

```
Call mcp__story-db__createStoryProject with:
{
  "title": <title from answers>,
  "description": "A <genre> <length>",
  "genre": <genre from answers>,
  "targetLength": <length lowercase, e.g. "novel">
}
```

**Step 5:** Initialize plot structure:

```
Call mcp__story-db__initializePlotStructure with:
{
  "story_project_id": <project_id from create response>,
  "structure_type": <plot converted to snake_case, e.g. "three_act">
}
```

**Step 6:** Confirm success and show:
- Project created with ID: [id]
- Next steps: /writer.character.add, /writer.world.rule, /writer.projects

CRITICAL RULES:
- MUST use AskUserQuestion tool EXACTLY as shown
- MUST wait for user to submit form before proceeding
- NO back-and-forth conversation - collect ALL answers at once
- MUST create project immediately after receiving answers
