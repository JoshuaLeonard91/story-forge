---
description: Define world rule with guided wizard
allowed-tools: ["mcp__story-db__addWorldRule", "mcp__story-db__listStoryProjects"]
---

You are helping the user define a world rule for their story. World rules ensure consistency in worldbuilding.

## FIRST: Verify MCP Server

**Immediately call the MCP tool** `mcp__story-db__listStoryProjects` with an empty params object `{}`.

DO NOT use Bash commands to check availability. Just call the tool directly.

**If the tool call succeeds:**
Proceed with the wizard below (use the returned projects list).

**If the tool call fails with an error:**
Stop and tell them:

```
The Story Forge MCP server is not configured yet. Please run /writer.mcp.setup first to configure the server, then restart Claude Code.

After restarting, run /writer.world.rule again.
```

---

## Interactive Wizard (Only if MCP server is available)

Guide them through these questions ONE AT A TIME:

1. **Project Selection** - Check if they have multiple projects:
   - If multiple exist, ask: "Which project is this rule for?" and show the list
   - If only one exists, use that project automatically
   - If NO projects exist, tell them to run /writer.start first

2. **Rule Name** - Ask: "What should this rule be called?"
   (Examples: "Magic System: Mana Cost", "Technology: No FTL Travel", "Social Law: Noble Privileges")

3. **Rule Description** - After they provide the name, ask: "Explain how this rule works in detail"
   (This should be a thorough explanation of the rule and its implications)

4. **Scope** - After the description, ask: "What's the scope of this rule?" with options:
   - Universal (Applies everywhere in your world)
   - Regional (Only in certain locations)
   - Situational (Only in specific circumstances)

5. **Examples** - After scope, ask: "Can you provide 1-2 concrete examples of this rule in action?"
   (Optional but highly recommended - helps with consistency later)

6. **Keywords** - Finally, ask: "What keywords should tag this rule for easy searching?"
   (Examples: magic, mana, technology, combat, social, physics - comma separated)

After gathering all information:
1. Show them a summary of the rule
2. Ask "Does this look correct? (yes/no)"
3. If yes, call `mcp__story-db__addWorldRule` with all the gathered details
4. Confirm success and explain that this rule will be used for context retrieval when writing scenes

Suggest next steps:
- Add more world rules to build out the magic system, technology, social structures, etc.
- Start adding characters with `/writer.character.add`
- Begin writing scenes that follow these rules

IMPORTANT: Only ask ONE question at a time. Wait for their answer before proceeding to the next question.
