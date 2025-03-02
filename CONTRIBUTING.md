# Contributing to AstroCore

ATTENTION NEW RECRUITS! *bzzt* This is AstroCore Command speaking!

So you've wandered into our little corner of the universe and want to help build the ultimate spaceship management simulator, eh? EXCELLENT CHOICE, CADET! Your contributions might just save future virtual astronauts from becoming highly educational space debris! 

This guide contains your standard operating procedures. Proper pull requests prevent plasma fires! Tag your issues, test your code, and for the love of space, document anything that might confuse the next engineer who stumbles into the maintenance shaft!

Remember: In space, no one can hear you code... but they CAN see your pull requests!

*Mission Control, over and out bzzt*

## The Basics

- **Found a bug?** Open an issue with a clear description and steps to reproduce
- **Fixed a bug?** Submit a pull request referencing the issue
- **Adding a feature?** Open an issue to discuss it first before coding
- **Have questions?** Use GitHub discussions rather than issues

## Getting Started

1. Fork the repository
2. Clone your fork:
    
    ``` bash
    git clone https://github.com/username/astrocore.git
    ```
    
3. Create a branch:
    
    ``` bash
    git checkout -b fix-awesome-bug
    ```
    
4. Make your changes
5. Commit using [Conventional Commits](https://www.conventionalcommits.org/).
6. Push to your fork:
    
    ``` bash
    git push origin fix-awesome-bug
    ```
    
7. Submit a pull request!

## Development Process

- Follow our [Git Workflow](https://github.com/8f00ff/knowledge-base/blob/main/Git/Git%20Workflow.md) guidelines
- Write tests for your changes
- Update documentation if needed
- Be kind and respectful in all interactions

## Code Style

- Follow Rust standard formatting (use `cargo fmt`) with these specific overrides:
  - Use 2-space indentation (not the Rust default 4-space)
  - Place colons for type annotations directly after variable names with one space after
  - Place commas directly after elements with one space after
  - No spaces inside square brackets: `[a, b]` not `[ a, b ]`
  - Yes spaces inside curly braces: `{ a, b }` not `{a, b}`
  - Indent chained method calls on new lines
  - Place `else` followed by newline + `if` for else-if chains: 
    ```rust
    } else
    if condition {
    ```
    not 
    ```rust
    } else if condition {
    ```
- Run `cargo clippy` before submitting PRs
- Comment complex parts of your code

## Project Structure

- `assets/` - Game assets
- `docs/` - Documentation
- `src/` - Source code
  - `entities/` - Entities
  - `systems/` - Systems
  - `components.rs` - Components
  - `resources/` - Resources
  - `plugins/` - Plugins
  - `main.rs` - Main entry point
- `tests/` - Tests

## Attribution Requirements

This project follows specific attribution guidelines that MUST be maintained in all derivatives and forks.

### [ATTRIBUTIONS.md](ATTRIBUTIONS.md) Structure

#### Maintainers

Current active maintainers only. Original authors should NOT be listed here unless actively maintaining this fork. Temporary PR forks (intended to be merged & deleted) don't need to modify this section, but long-term or permanent forks MUST update it to reflect actual maintainers.

#### Original Authors

Original creators of the project. This section MUST remain intact and unmodified in ALL derivatives and forks.

#### Additional Contributors

All original contributors MUST be kept, new ones may be appended. PR authors should add themselves here.

#### Supporters

All original supporters MUST be preserved, new ones may be appended.

#### Third-Party Assets & Libraries

All third-party assets, libraries, and tools used by the project. Each entry MUST include:
- Name with link to source
- Brief description of the asset/tool
- License information in parentheses

All entries MUST remain in this section as long as the corresponding asset or library is being used in any part of the project. New dependencies should be appended as they are added. When a dependency is removed, its entry may be removed from this section, as git history preserves the attribution record.

### Donation Links Policy

- Original creator donation/support links, if any exist, MUST always be maintained in all derivatives.
- Derivative works may include their own donation/support links.
- Derivative creator links may be positioned above original creator links, but original links must remain present and functional.

### In-Game Credits / UI

- Any credits screen or about page MUST maintain attribution to original creators.
- Format: "Originally created by [ORIGINAL AUTHORS]. See ATTRIBUTIONS.md for full credits."

### Fork Responsibility

- Original authors reserve the right to request correction of attribution in any fork that misrepresents maintainer status.
- "PR-only" forks that become long-term projects MUST update the Maintainers section to accurately reflect who is actively maintaining the fork.

**Using this project constitutes agreement to follow these attribution guidelines.**

## Questions?

Feel free to open a discussion if you have any questions!
