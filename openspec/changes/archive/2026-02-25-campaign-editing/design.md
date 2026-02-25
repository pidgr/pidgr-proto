## Context

Campaigns follow a strict state machine: CREATED → RUNNING → COMPLETED/FAILED/CANCELLED. Only CREATED campaigns are safe to edit because no workflow has started and no deliveries exist yet. Once RUNNING, the audience snapshot is frozen in S3 and Temporal workflows are active.

## Goals / Non-Goals

**Goals:**
- Allow editing draft campaigns (CREATED status) before starting
- Field-level updates: name, sender_name, title, template_id, template_version, workflow
- Server-side state guard: reject edits to non-CREATED campaigns

**Non-Goals:**
- Editing RUNNING campaigns (would require Temporal workflow cancellation + restart)
- Audience re-selection (audience snapshot is created at `CreateCampaign` time)
- Partial workflow updates (send the full workflow, server replaces entirely)

## Decisions

**1. All mutable fields in a single UpdateCampaignRequest**

Single RPC with all editable fields. Empty/zero values mean "no change" — only non-empty fields update. This is simpler than field masks for our use case.

**2. campaign_id is required, all other fields are optional**

If `name` is empty string, it's not updated. If `template_version` is 0, it's not updated. If `workflow` is nil, it's not updated. Server applies only non-zero-value fields.

**3. No audience editing**

The audience snapshot is taken at creation and stored in S3. Re-selecting audience would require re-uploading the snapshot, which is a separate concern and not needed for the common "fix a typo before launch" use case.

## Risks / Trade-offs

- [Zero-value ambiguity] An admin can't intentionally set `title` to empty string (it would be treated as "no change"). Acceptable since title is optional and clearing it is rare.
