---
description: List all story projects with status and word counts
---

Call `mcp__story-db__listStoryProjects` with empty params `{}` to fetch all projects.

Display the results in a clean table format (plain text, no markdown bold or emojis):

```
Your Story Projects
===================

1. [Title] - [Genre]
   Status: [status] | Word Count: [wordCount]
   Last Updated: [updatedAt]

2. [Title] - [Genre]
   Status: [status] | Word Count: [wordCount]
   Last Updated: [updatedAt]
```

After displaying the list, call AskUserQuestion:

```json
{
  "questions": [
    {
      "question": "What would you like to do next?",
      "header": "Next Action",
      "options": [
        {"label": "Add characters", "description": "Create character profiles for your story"},
        {"label": "Add world rules", "description": "Define magic systems, technology, social structures"},
        {"label": "Start new project", "description": "Create another story project"},
        {"label": "Nothing right now", "description": "Exit this wizard"}
      ],
      "multiSelect": false
    }
  ]
}
```

Based on their selection:
- "Add characters" → Run /writer:character:add
- "Add world rules" → Run /writer:world:rule
- "Start new project" → Run /writer:start
- "Nothing right now" → Say "You can start these commands anytime from the command palette."

CRITICAL RULES:
- Display projects in clean text format, no emojis
- MUST use AskUserQuestion for next action
- Execute the selected action immediately
