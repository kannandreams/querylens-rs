# Agents

This file defines custom agents for the QueryLens project, used in VS Code for specialized tasks.

## SQL Analyzer Agent

**Purpose:** Analyze and provide insights on SQL files using QueryLens tools.

**Capabilities:**
- Generate SQL reports with structured insights
- Scan SQL files for patterns and potential issues
- Calculate SQL quality scores
- Compare SQL files and structures

**Usage:**
- For code review of SQL queries
- Quality assurance in data pipelines
- Refactoring assistance for SQL code

**Commands:**
- `querylens report` - Generate detailed SQL reports
- `querylens scan` - Deep scan for SQL patterns
- `querylens score` - Quality scoring
- `querylens diff` - Compare SQL files

## Rust Development Agent

**Purpose:** Assist with Rust development, code quality, and project maintenance.

**Capabilities:**
- Code review and suggestions
- Testing and debugging support
- Performance optimization
- Documentation generation

**Usage:**
- During development of QueryLens features
- Code refactoring and cleanup
- CI/CD integration

**Skills:**
- Rust syntax and best practices
- Cargo and dependency management
- Error handling and safety
- Performance profiling

## Configuration Agent

**Purpose:** Manage QueryLens configuration files and settings.

**Capabilities:**
- Generate default configurations
- Validate configuration syntax
- Suggest optimizations
- Handle multiple environments

**Usage:**
- Initial project setup
- Configuration troubleshooting
- Environment-specific settings

**Files:**
- `.querylens.yaml`
- `.querylens.json`
- Custom config templates