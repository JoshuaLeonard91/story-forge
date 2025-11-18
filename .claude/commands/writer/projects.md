---
description: List all story projects with status and word counts
allowed-tools: ["mcp__story-db__listStoryProjects"]
---

# List All Story Projects

Let me retrieve all your story projects from the database.

I'll call the `mcp__story-db__listStoryProjects` tool to fetch all projects.

[Tool will return a list of projects with: projectId, title, genre, status, wordCount, updatedAt]

After displaying the list, you can:
- Load a specific project with `/writer.load` + projectId
- Start a new project with `/writer.start`
- View project details with `/writer.info` + projectId
