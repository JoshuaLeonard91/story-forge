---
description: Create new story project with guided wizard (title, genre, target length, plot structure)
allowed-tools: ["mcp__story-db__createStoryProject", "mcp__story-db__initializePlotStructure"]
---

# Story Project Creation Wizard

I'll help you create a new story project. Let me gather the essential information:

## Step 1: Basic Information

**1. Story Title:** What's the title of your story?

**2. Genre Selection:**
1. Epic Fantasy - Long-form fantasy with worldbuilding (Lord of the Rings, Wheel of Time)
2. LitRPG - Game progression, stats, levels (Solo Leveling, The Primal Hunter)
3. Progression Fantasy - Power growth, less stats focus
4. Cultivation/Xianxia - Realms, qi, martial arts
5. Urban Fantasy - Modern + supernatural
6. Science Fiction - Space, tech, future
7. Contemporary Fiction - Realistic, modern
8. Custom (specify your own)

**3. Target Length:**
1. Short Story (< 20,000 words)
2. Novella (20,000-50,000 words)
3. Novel (50,000-150,000 words)
4. Series (150,000+ words across multiple books)

**4. Plot Structure:**
1. Three-Act Structure - Setup → Conflict → Resolution
2. Five-Act Structure - Classic dramatic structure
3. Hero's Journey - Epic transformation arc
4. Custom/Freeform - No predefined structure

---

After you provide these details, I will:
1. Call `mcp__story-db__createStoryProject` to create the project database entry
2. Call `mcp__story-db__initializePlotStructure` to set up your chosen plot structure
3. Display your new project ID and summary

Once created, you can:
- Add characters with `/writer.character.add`
- Define world rules with `/writer.world.rule`
- Start planning chapters with `/writer.chapter.add`

Let's begin! Please answer the questions above.
