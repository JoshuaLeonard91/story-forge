---
description: Add new character with guided wizard
allowed-tools: ["mcp__story-db__addCharacter", "mcp__story-db__listStoryProjects"]
---

You are helping the user add a new character to their story. This wizard will guide them through creating a detailed character profile.

## FIRST: Check MCP Server Availability

Try to call `mcp__story-db__listStoryProjects` to verify the MCP server is running.

**If the MCP server is NOT available (tool call fails):**
Stop immediately and tell them:

```
The Story Forge MCP server is not configured yet. Please run /writer.mcp.setup first to configure the server, then restart Claude Code.

After restarting, run /writer.character.add again.
```

Do NOT ask any questions if the MCP server isn't available.

**If the MCP server IS available:**
Proceed with the wizard below.

---

## Interactive Wizard (Only if MCP server is available)

First, check which project to use:
- Call `mcp__story-db__listStoryProjects` to get available projects
- If NO projects exist, tell them to run /writer.start first and stop
- If only ONE project exists, use it automatically
- If MULTIPLE projects exist, use AskUserQuestion to let them select which project

Then use AskUserQuestion to collect all character information at once:

```
AskUserQuestion with these questions:
```

1. **Character Name**:
   - question: "What's the character's name?"
   - header: "Name"
   - options: [{label: "Enter name", description: "Provide character's full name"}]
   - multiSelect: false

2. **Character Role**:
   - question: "What role does this character play?"
   - header: "Role"
   - options: [
       {label: "Protagonist", description: "Main hero of the story"},
       {label: "Antagonist", description: "Primary villain or opposition"},
       {label: "Supporting", description: "Important secondary character"},
       {label: "Minor", description: "Background character"}
     ]
   - multiSelect: false

3. **Personality Traits**:
   - question: "Describe their personality"
   - header: "Personality"
   - options: [{label: "Describe personality", description: "Personality traits, demeanor, quirks"}]
   - multiSelect: false

4. **Physical Description**:
   - question: "What do they look like?"
   - header: "Appearance"
   - options: [{label: "Describe appearance", description: "Height, build, hair, eyes, distinctive features"}]
   - multiSelect: false

After collecting answers:
1. Show them a summary of the character
2. Call `mcp__story-db__addCharacter` with all the gathered details
3. Confirm success and suggest next steps:
   - Add character relationships with `/writer.character.relationship`
   - Add more characters with `/writer.character.add`
   - Start writing scenes that feature this character

IMPORTANT: Use AskUserQuestion to ask ALL questions at once, then process answers.
