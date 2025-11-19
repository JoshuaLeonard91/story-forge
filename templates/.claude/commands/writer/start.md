---
description: Create new story project with guided wizard
---

You are helping the user create a new story project. This wizard will guide them through setting up their story step-by-step.

## FIRST: Check if MCP Tools are Available

Look at your available tools list. Do you see any tools starting with `mcp__story-db__`?

**If YES** (you see `mcp__story-db__createStoryProject`, `mcp__story-db__listStoryProjects`, etc.):
Proceed with the interactive wizard below.

**If NO** (you don't see any `mcp__story-db__` tools):
Stop and tell them:

```
The Story Forge MCP server is not loaded. This could mean:

1. You haven't run /writer:mcp.setup yet
2. You need to restart VS Code/Claude Code for the MCP server to load
3. There's an issue with the MCP server configuration

Please try:
1. Run /writer:mcp.setup if you haven't already
2. Completely close and restart VS Code
3. Start a NEW Claude Code conversation (not this one)
4. Run /writer:start again

If the issue persists, check the VS Code Output panel (View → Output) and look for "MCP Servers" in the dropdown for error messages.
```

---

## Interactive Wizard (Only if MCP server is available)

**Step 1:** Ask for the story title directly.

Say: "Let's create your story project! What's the title of your story?"

Wait for the user to reply with their title. Save this as `storyTitle`.

**Step 2:** Call the AskUserQuestion tool with the EXACT JSON below. DO NOT modify it. Copy it EXACTLY as shown:

```json
{
  "questions": [
    {
      "question": "Is this book part of a series?",
      "header": "Series",
      "options": [
        {"label": "Standalone", "description": "Not part of a series"},
        {"label": "Enter series name", "description": "This book is part of a series"}
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
    }
  ]
}
```

WARNING: DO NOT ADD ANY OTHER OPTIONS. DO NOT modify the labels. The JSON above is COMPLETE. Use it EXACTLY as written.

**Step 3:** Wait for user to submit the form with all their answers.

**Step 4:** Ask for plot structure directly.

Say: "What plot structure would you like to use? (Examples: Three-Act Structure, Five-Act Structure, Hero's Journey, Seven-Point Story, Save the Cat, Kishotenketsu, In Medias Res, Custom/Freeform, or describe your own)"

Wait for the user to reply. Save this as `plotStructure`.

**Step 5:** Extract answers:
- Title: Use `storyTitle` from Step 1
- Series: answers["0"] - If "Standalone" use "standalone", else use "Other" text input
- Genre: answers["1"] - Can be preset option or "Other" for custom genre
- Length: answers["2"] - Can be preset option or "Other" for custom length
- Plot: Use `plotStructure` from Step 4

**Step 6:** Show detailed confirmation and ask for approval:

First, display a summary like this (plain text, no markdown bold or emojis):
```
Story Project Summary
=====================

Title: {title}
Series: {series}
Genre: {genre}
Target Length: {length}
Plot Structure: {plot}

Folder Structure:
stories/{series}/{title}/
  chapters/
  metadata.json

This will create a new story project in your database and set up the folder structure.
```

Then call the AskUserQuestion tool with EXACTLY this JSON (do not modify):
```json
{
  "questions": [
    {
      "question": "Do you want to create this project?",
      "header": "Confirm",
      "options": [
        {"label": "Yes, create project", "description": "Create the project with these settings"},
        {"label": "No, cancel", "description": "Cancel project creation"}
      ],
      "multiSelect": false
    }
  ]
}
```

Wait for user to submit. If they select "No, cancel", stop and say "Project creation cancelled."

If they select "Yes, create project", proceed to Step 7.

**Step 7:** Determine structure type from user's plot choice:

Based on what they said in Step 4, map it to one of these structure types:
- If they mentioned "Three-Act" or "3-Act" → use "three_act"
- If they mentioned "Five-Act" or "5-Act" → use "five_act"  
- If they mentioned "Hero's Journey" or "Monomyth" → use "hero_journey"
- For anything else (Seven-Point, Save the Cat, Kishotenketsu, In Medias Res, Custom, Freeform, or other) → use "custom"

Save this as `structureType`.

**Step 8:** Create the story project:

Call `mcp__story-db__createStoryProject` with:
```json
{
  "title": <title from step 1>,
  "seriesName": <series from step 5>,
  "description": "A <genre> <length>",
  "genre": <genre from step 5, convert to lowercase snake_case if needed>,
  "targetLength": <length converted to: "short_story", "novella", "novel", or "series">
}
```

**IMPORTANT:** After calling, extract the `projectId` from the response. If you don't see a response, the tool still worked - MCP tools execute silently in VS Code.

**Step 9:** Verify the project was created:

Call `mcp__story-db__listStoryProjects` with empty params `{}`.

From the response, find the project with the title you just created and get its `projectId`.

**Step 10:** Initialize plot structure:

Call `mcp__story-db__initializePlotStructure` with:
```json
{
  "projectId": <projectId from step 9>,
  "structureType": <structureType from step 7>
}
```

**Step 11:** Confirm success to the user:

Tell them (plain text, no markdown bold or emojis):
```
Story project created successfully!

Project Details:
- Title: <title>
- Series: <series>
- Genre: <genre>
- Target Length: <length>
- Plot Structure: <plotStructure from step 4>
- Project ID: <projectId>

Folder Location:
stories/<series>/<title>/

Your story content will be saved here as you write chapters and scenes.

Next steps:
- /writer:character:add - Add characters to your story
- /writer:world:rule - Define world-building rules
- /writer:projects - View all your projects
```

CRITICAL RULES:
- MUST ask for title as plain text FIRST (Step 1)
- MUST ask for plot structure as plain text (Step 4)
- MUST use the AskUserQuestion JSON EXACTLY as provided for series/genre/length - DO NOT MODIFY
- DO NOT add extra options (max 4 options per question)
- User fills out the 3-question form and submits ONCE
- MUST show detailed confirmation summary
- MUST use AskUserQuestion for final confirmation (Yes/No options)
- MUST create project only after user confirms "Yes, create project"
- NO emojis or markdown bold in output
