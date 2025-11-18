---
description: Create new story project with guided wizard
allowed-tools: ["mcp__story-db__createStoryProject", "mcp__story-db__initializePlotStructure"]
---

You are helping the user create a new story project. Guide them through these questions ONE AT A TIME:

1. **Story Title** - Ask: "What's the title of your story?"

2. **Genre** - After they respond, ask: "What genre is this story?" and provide these options:
   - Epic Fantasy (Lord of the Rings, Wheel of Time)
   - LitRPG (Solo Leveling, The Primal Hunter)
   - Progression Fantasy (Cradle, Mother of Learning)
   - Cultivation/Xianxia (I Shall Seal the Heavens)
   - Urban Fantasy (Dresden Files)
   - Science Fiction
   - Contemporary Fiction
   - Other (let them specify)

3. **Target Length** - After they choose genre, ask: "What's your target length?" with options:
   - Short Story (under 20,000 words)
   - Novella (20,000-50,000 words)
   - Novel (50,000-150,000 words)
   - Series (150,000+ words)

4. **Plot Structure** - After they choose length, ask: "Which plot structure would you like to use?" with options:
   - Three-Act Structure (Setup → Conflict → Resolution)
   - Five-Act Structure (Classic dramatic arc)
   - Hero's Journey (Epic transformation)
   - Custom/Freeform (No predefined structure)

After gathering all four answers:
1. Show them a summary of what will be created
2. Ask "Does this look correct? (yes/no)"
3. If yes, use the MCP tools to create the project:
   - Call `mcp__story-db__createStoryProject` with the gathered information
   - Call `mcp__story-db__initializePlotStructure` to set up acts based on chosen structure
4. Confirm success and suggest next steps (/writer.character.add, /writer.world.rule)

IMPORTANT: Only ask ONE question at a time. Wait for their answer before proceeding to the next question.
