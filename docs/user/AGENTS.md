# User docs: rules for agents and contributors

Everything in this folder (except this file, CLAUDE.md and README.md) is the user guide for this
fork of storytold/designcraft. The file path is the page's path:
`troubleshooting/app-closes-at-start.md` → `troubleshooting/app-closes-at-start`. Renaming a file
breaks links, so add the old path to `redirect_from` when you rename.

## Who reads this

Artists and designers using DesignCraft, people answering their questions, and AI assistants looking things up for
either. Write for an artist, not a developer.

## Page rules

- One question or problem per page. Lead with the answer or the fix in the first two lines.
- Plain words, short sentences, numbered steps for anything the reader does.
- Start the body at `##`. The page title comes from frontmatter.
- Link to other pages instead of repeating them. Use relative links to the `.md` file:
  `[Install DesignCraft](../getting-started/install.md)`.
- No marketing copy, no "simply", no claims you have not checked in the code or a release.
- Call the product "DesignCraft", as the app and its downloads do.
- Name UI labels exactly as the app shows them, in **bold**.
- Document Windows 11 only. Do not name older Windows versions.
- Troubleshooting pages: `## Symptoms`, `## Fix`, `## Why this happens` (optional),
  `## Still stuck?` (link the issue tracker).

## Frontmatter

    ---
    title: App closes right after it starts
    description: One sentence for search results, link previews and AI summaries.
    order: 10                 # optional, sorts within the folder (lower first)
    verified: 0.6.0           # optional, app version you last checked this page against
    redirect_from: [old/path] # optional, old URLs (relative to /docs/designcraft)
    ---

Only `key: value` and `key: [a, b]` lines work. Quote values that start with a quote or contain ` #`.

Set `verified` to the latest release (the newest release tag) after checking the page against that tag with `git show <tag>:<path>`. Pages describe the latest release users download, not unreleased code on main.

## Folders

- `getting-started/` install and first run
- `guides/` how to do one thing
- `troubleshooting/` symptom → fix
- `faq/` short answers
- `reference/` lists and settings

A folder's `index.md` names the folder in the sidebar. Files or folders starting with `_` are
drafts and are not published.

## Check before you push

Check that the front matter is complete and every link resolves.

## Support PRs

Pages written to answer a user question come in as PRs titled `docs(support): <page title>`.
Review them like code: check every claim against the source. Questions and bugs go to
https://github.com/685ks/designcraft/issues.
