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

Use the AskUserQuestion tool to collect all story information at once:

```
AskUserQuestion with these 4 questions:
```

1. **Story Title** (question 1):
   - question: "What's the title of your story?"
   - header: "Story Title"
   - options: [
       {label: "Enter custom title", description: "Provide your own story title"}
     ]
   - multiSelect: false

   Note: User will select "Other" to enter custom title

2. **Genre** (question 2):
   - question: "What genre is this story?"
   - header: "Genre"
   - options: [
       {label: "Epic Fantasy", description: "Lord of the Rings, Wheel of Time"},
       {label: "LitRPG", description: "Solo Leveling, The Primal Hunter"},
       {label: "Progression Fantasy", description: "Cradle, Mother of Learning"},
       {label: "Urban Fantasy", description: "Dresden Files, magic in modern world"}
     ]
   - multiSelect: false

3. **Target Length** (question 3):
   - question: "What's your target length?"
   - header: "Length"
   - options: [
       {label: "Short Story", description: "Under 20,000 words"},
       {label: "Novella", description: "20,000-50,000 words"},
       {label: "Novel", description: "50,000-150,000 words"},
       {label: "Series", description: "150,000+ words across multiple books"}
     ]
   - multiSelect: false

4. **Plot Structure** (question 4):
   - question: "Which plot structure would you like to use?"
   - header: "Plot"
   - options: [
       {label: "Three-Act", description: "Setup → Conflict → Resolution"},
       {label: "Five-Act", description: "Classic dramatic arc with five acts"},
       {label: "Hero's Journey", description: "Epic transformation arc"},
       {label: "Custom", description: "No predefined structure"}
     ]
   - multiSelect: false

After collecting answers from AskUserQuestion:
1. Extract the answers from the response
2. Show them a summary of what will be created
3. Use the MCP tools to create the project:
   - Call `mcp__story-db__createStoryProject` with:
     - title: from question 1 (use "Other" text if they provided custom)
     - genre: from question 2 (label selected)
     - targetLength: from question 3 (label selected, convert to lowercase)
     - description: Generate a brief description based on their choices
   - Call `mcp__story-db__initializePlotStructure` with:
     - story_project_id: from create response
     - structure_type: from question 4 (label selected, convert to snake_case)
4. Confirm success with the project ID and suggest next steps:
   - /writer.character.add - Add your protagonist and other characters
   - /writer.world.rule - Define magic systems, world laws, etc.
   - /writer.projects - View all your story projects

IMPORTANT:
- ALWAYS check MCP server availability FIRST
- Use AskUserQuestion to ask ALL questions at once
- Process answers after user submits the form
- Create the project immediately after receiving answers
