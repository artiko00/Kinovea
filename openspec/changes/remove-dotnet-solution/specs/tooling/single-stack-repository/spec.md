# Spec Delta

## Purpose

Ensures that after legacy retirement the repository holds exactly one buildable application stack and that tooling, documentation and CI describe only that stack, so contributors and agents are never pointed at code that no longer exists.

## ADDED Requirements

### Requirement: No legacy sources remain
The repository SHALL contain no C#, C++/CLI, MSBuild or NSIS sources, no vendored Windows SDKs and no Windows-only workflow.

#### Scenario: Source scan
- **WHEN** the repository tree is scanned for `*.cs`, `*.csproj`, `*.sln`, `*.vcxproj`, `*.nsi`, `*.resx`, `*.targets`, `packages.config`
- **THEN** no file matches outside the archived OpenSpec changes

#### Scenario: CI
- **WHEN** the workflows directory is listed
- **THEN** only Linux workflows exist and the last Windows build run is referenced from the retirement log, not from any active workflow

### Requirement: Legacy remains reachable
The last commit containing the .NET solution SHALL be tagged so the legacy code can be checked out for reference.

#### Scenario: Tag
- **WHEN** a developer checks out the tag named in the retirement log
- **THEN** the .NET solution and the oracle tooling are present at that commit

### Requirement: Assets the Rust build needs are relocated, not deleted
Data files consumed by the application (KVA schemas and migration stylesheets, standard and custom drawing tool definitions, toolbars) SHALL live under an assets directory that the Rust build embeds, with their original relative structure.

#### Scenario: Schemas
- **WHEN** the metadata tests run
- **THEN** they validate against the schemas from the assets directory and pass

#### Scenario: Drawing tools
- **WHEN** the application starts
- **THEN** the drawing tool palette lists the same standard tools and custom postures as the last legacy release

### Requirement: Documentation and agent guide describe the single stack
`readme.md`, `architecture.md`, `localization.md` and `AGENTS.md` SHALL describe only the Rust application, its crates, its commands and its CI.

#### Scenario: Command table
- **WHEN** every command in the agent guide's command table is executed on Linux
- **THEN** each one runs (or prints its usage) without referring to MSBuild, NuGet, NSIS or a Windows host

### Requirement: Quality gates have no legacy exclusions
The linter configurations SHALL exclude only build output, dependencies, fixtures and tooling directories, never application source directories.

#### Scenario: Exclude lists
- **WHEN** the exclude lists are read
- **THEN** no entry names a deleted directory and the four gates pass on the full tree
