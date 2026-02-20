# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**pidgr-proto** is the shared Protocol Buffers definitions repository for the Pidgr platform — an internal communication system that replaces passive announcements with structured, measurable campaigns. This repo is the single source of truth for all gRPC service contracts consumed by downstream services.

## Build Commands

```bash
buf build       # Compile all proto files
buf lint        # Lint against STANDARD rules
buf generate    # Generate Go, Rust, and TypeScript stubs
buf breaking --against .git#branch=main  # Check for breaking changes
```

## Package Structure

```
proto/pidgr/v1/         # All proto source files
  common.proto          # Shared types (User, Role, Permission, enums), Message, WorkflowDefinition, pagination
  organization.proto    # OrganizationService — org CRUD + Industry/CompanySize enums
  member.proto          # MemberService — user invitation, lookup, role changes, deactivation
  role.proto            # RoleService — role listing + permission management
  campaign.proto        # CampaignService — campaign lifecycle
  template.proto        # TemplateService — markdown template management
  action.proto          # ActionService — generic user action submission
  inbox.proto           # InboxService — mobile inbox sync + read tracking
  device.proto          # DeviceService — push token management
  render.proto          # RenderService — internal Go↔Rust template compilation (server-streaming)
```

- Package name: `pidgr.v1`
- Go package: `github.com/pidgr/pidgr-proto/gen/go/pidgr/v1;pidgrv1`
- All files use `proto3` syntax
- Enum values prefixed with enum name in UPPER_SNAKE_CASE (e.g., `CAMPAIGN_STATUS_CREATED`)
- All enums have `UNSPECIFIED = 0` as safety net
- `org_id` is never in request messages — extracted from JWT claims in server middleware

## Code Generation Targets

| Language | Plugins | Output | Consumer |
|----------|---------|--------|----------|
| Go | protocolbuffers/go + grpc/go | `gen/go/` | pidgr-api (monolith) |
| Rust | neoeinstein-prost + neoeinstein-tonic | `gen/rust/` | pidgr-renderer |
| TypeScript | timostamm-protobuf-ts | `gen/ts/` | pidgr-mobile (React Native) |

## Key Design Patterns

- **Canonical Message type**: One `Message` message used across render output, inbox entries, and delivery context
- **Generic Action system**: `SubmitAction` handles all action types (ACK for MVP, future: POLL, CTA)
- **WorkflowDefinition as data**: Campaign workflows are JSON-representable DAGs of steps, not hardcoded logic
- **Read vs Action separation**: `MarkRead` is analytics-only (OTEL + PostHog), `SubmitAction` drives Temporal workflows
- **Server-streaming for rendering**: `RenderBatch` streams results as Rust completes each user in parallel

## CI/CD

- **CI** (`.github/workflows/ci.yml`): `bufbuild/buf-action@v1` runs build, lint, breaking on every push/PR
- **Release** (`.github/workflows/release.yml`): triggered by `v*` tags — generates code, publishes npm, creates GitHub Release
- Breaking change detection skips when main has no proto history

## Package Distribution

| Language | Mechanism | How to Consume |
|----------|-----------|----------------|
| Go | Private Git module | `GOPRIVATE=github.com/pidgr/*` + `go get ...@v0.1.0` |
| Rust | Private Git dependency | `pidgr-proto = { git = "...", tag = "v0.1.0", subdirectory = "gen/rust" }` |
| TypeScript | Private npm on GitHub Packages | `npm install @pidgr/proto@0.1.0` (requires `.npmrc` with `@pidgr:registry`) |

All packages are private within the pidgr organization. Generated code is committed to the repo and hidden from PR diffs via `.gitattributes`.

## OpenSpec

This repo uses OpenSpec for structured change management. Changes live in `openspec/changes/`. Use `/opsx:new` to start a new change.
