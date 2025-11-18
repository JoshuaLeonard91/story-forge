# Quick Start Guide - Story Writing Engine

Get started writing your first story in 5 minutes!

## Step 1: Create Your First Project

In Claude Code, type:

```
/writer.start
```

Answer the wizard questions:

**1. Story Title**: "The Last Kingdom"

**2. Genre**: Epic Fantasy (option 1)

**3. Target Length**: Novel (option 3)

**4. Plot Structure**: Three-Act Structure (option 1)

Claude will create your project and initialize the three-act structure.

**Expected Output:**
```json
{
  "projectId": "uuid-here",
  "title": "The Last Kingdom",
  "genre": "Epic Fantasy",
  "intendedLength": "novel",
  "status": "draft",
  "structureType": "three_act",
  "acts": [
    {"actId": "...", "name": "Act 1: Setup", "position": 1},
    {"actId": "...", "name": "Act 2: Confrontation", "position": 2},
    {"actId": "...", "name": "Act 3: Resolution", "position": 3}
  ]
}
```

---

## Step 2: Add Your Protagonist

```
/writer.character.add
```

Provide details:

**1. Character Name**: "Aria Stormwind"

**2. Character Role**: Protagonist (option 1)

**3. Personality Traits**: "Brave, determined, compassionate but haunted by past failures"

**4. Physical Description**: "Tall woman with silver hair and piercing blue eyes. Bears a scar across her left cheek from a childhood accident."

**5. Backstory**: "Former captain of the Royal Guard who resigned after failing to prevent the assassination of the king. Now seeks redemption by protecting the last heir to the throne."

**6. Current State**: "Living in exile in a remote village, training in secret"

**Expected Output:**
```json
{
  "characterId": "uuid-here",
  "name": "Aria Stormwind",
  "role": "protagonist",
  "personalityTraits": "Brave, determined, compassionate...",
  "physicalDescription": "Tall woman with silver hair...",
  "backstory": "Former captain of the Royal Guard...",
  "currentState": "Living in exile..."
}
```

---

## Step 3: Add Your Antagonist

```
/writer.character.add
```

**1. Character Name**: "Lord Malkor"

**2. Character Role**: Antagonist (option 2)

**3. Personality Traits**: "Cunning, ruthless, patient strategist. Believes the ends justify any means."

**4. Physical Description**: "Middle-aged man with graying temples and cold, calculating eyes. Always wears black and gold."

**5. Backstory**: "Former advisor to the king who orchestrated the assassination to seize power. Now rules as regent while hunting the last heir."

**6. Current State**: "Consolidated power in the capital, sending agents to find the heir"

---

## Step 4: Define Your Magic System

```
/writer.world.rule
```

**1. Rule Name**: "Magic System: Elemental Binding"

**2. Description**: "Magic users can bind one of four elements (Fire, Water, Earth, Air) at age 16. The element chooses the mage based on their personality. Binding grants control over that element but requires physical stamina - overuse leads to exhaustion and unconsciousness."

**3. Scope**: Universal (option 1)

**4. Examples**:
- "Fire mages can conjure flames but become exhausted after large-scale attacks"
- "Water mages can heal wounds but can't bring back the dead"
- "Earth mages can shape stone but need rest between major constructions"
- "Air mages can fly short distances but tire quickly"

**5. Keywords**: magic, elemental, binding, stamina, fire, water, earth, air

**Expected Output:**
```json
{
  "ruleId": "uuid-here",
  "name": "Magic System: Elemental Binding",
  "description": "Magic users can bind one of four elements...",
  "scope": "universal",
  "keywords": ["magic", "elemental", "binding", "stamina", "fire", "water", "earth", "air"]
}
```

---

## Step 5: View Your Project

```
/writer.projects
```

You'll see your project listed with current status and word count (0 initially).

**Expected Output:**
```json
{
  "projects": [
    {
      "projectId": "uuid-here",
      "title": "The Last Kingdom",
      "genre": "Epic Fantasy",
      "intendedLength": "novel",
      "status": "draft",
      "wordCount": 0,
      "updatedAt": "2025-11-18T..."
    }
  ]
}
```

---

## What You've Built So Far

✅ **Story Project**: "The Last Kingdom" - Epic Fantasy novel
✅ **Characters**:
  - Aria Stormwind (Protagonist) - Exiled royal guard
  - Lord Malkor (Antagonist) - Usurper regent
✅ **World Rule**: Elemental Binding magic system
✅ **Plot Structure**: Three-act structure with acts set up

---

## Next Steps

### Add More Characters

```
/writer.character.add
```

Ideas for supporting characters:
- **Prince Kael** (Supporting) - The last heir, currently in hiding
- **Theron** (Supporting) - Aria's old mentor who trained her
- **Lady Seraphina** (Supporting) - Court spy secretly helping Aria

### Define More World Rules

```
/writer.world.rule
```

Ideas for world rules:
- **Social Structure**: "Kingdom of Aethermoor - Monarchy with Noble Houses"
- **Technology Level**: "Medieval Era - No gunpowder, blacksmithing advanced"
- **Forbidden Magic**: "Necromancy is punishable by death"
- **Geography**: "Four realms separated by the Shattered Mountains"

### Plan Your Chapters (Coming in Future Update)

Once User Story 2 is implemented, you'll be able to:
- Add chapters to each act
- Create scenes within chapters
- Write scene content with AI assistance
- Get context-aware suggestions based on characters and world rules

---

## Tips for Success

### 1. Be Detailed with Characters
The more detail you provide, the better the AI can help you write consistent character behavior.

**Good**: "Determined, strategic, haunted by past failures, protective of innocents"
**Better**: "Determined warrior who meticulously plans every battle. Haunted by the assassination she failed to prevent, she's fiercely protective of innocents to the point of recklessness. Struggles with trust issues."

### 2. Make World Rules Specific
Clear rules help maintain consistency.

**Vague**: "Magic costs energy"
**Specific**: "Each spell requires mana proportional to its power. Casting beyond your mana pool causes physical exhaustion. Recovery requires 8 hours rest."

### 3. Use Keywords Effectively
Keywords help the context system find relevant rules when writing scenes.

**Example**: Magic rule with keywords: `["magic", "mana", "spell", "exhaustion", "casting"]`

### 4. Track Character Relationships
As you add more characters, define their relationships:

```
# After adding both Aria and Lord Malkor:
Call mcp__story-db__addCharacterRelationship with:
{
  "sourceCharacterId": "aria-id",
  "targetCharacterId": "malkor-id",
  "relationshipType": "enemy",
  "description": "Aria seeks justice for the king's assassination that Malkor orchestrated",
  "strength": 10
}
```

---

## Common Workflows

### Starting a New Writing Session

1. List your projects: `/writer.projects`
2. Load your project (when feature available)
3. Review characters and rules
4. Start writing scenes

### Adding a New Character Mid-Story

1. `/writer.character.add` - Fill in details
2. Define relationships with existing characters
3. Update relevant scenes if needed

### Expanding Your World

1. `/writer.world.rule` - Add new rules as needed
2. Use keywords to link rules to existing content
3. Keep rules focused (one concept per rule)

---

## What's Coming Next

### Phase 4: Context-Aware Scene Writing
- Write scenes with automatic context injection
- AI knows which characters are present
- Relevant world rules automatically included
- Recent events summarized
- <5000 word context budget

### Phase 5: Character Arc Tracking
- Define character transformation arcs
- Set milestones for character growth
- Track progress through the story
- Get alerts when arcs stall

### Phase 6: Auto-Generated Summaries
- Scene summaries (2-3 sentences)
- Chapter summaries (paragraph)
- Act summaries (page)
- Overall story summary

### Phase 7: Continuity Validation
- Detect contradictions automatically
- Flag timeline issues
- Check character state consistency
- Validate against world rules

---

## Need Help?

- **Documentation**: See README.md for full feature list
- **Installation Issues**: See INSTALLATION.md
- **Report Bugs**: GitHub Issues
- **Feature Requests**: GitHub Discussions

---

**Ready to write your epic story!** 🚀

Try creating a few more characters and world rules to get comfortable with the workflow. When Phase 4 ships, you'll be ready to start writing scenes with full AI assistance!
