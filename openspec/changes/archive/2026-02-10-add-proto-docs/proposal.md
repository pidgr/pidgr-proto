## Why

There is no browsable reference for the Pidgr gRPC services. Developers consuming the proto (pidgr-api, pidgr-mobile, pidgr-renderer) must read raw .proto files to understand available RPCs, message types, and field semantics. A generated HTML documentation page gives a single-page, browsable reference with field descriptions and cross-linked types.

## What Changes

- Add proto comments (doc strings) to all services, RPCs, messages, and fields across all 8 proto files
- Add `protoc-gen-doc` plugin to `buf.gen.yaml` to generate HTML docs on `buf generate`
- Generate docs to `docs/` directory
- Add `docs/` to `.gitattributes` (hide from PR diffs, like generated code)
- Add a `make docs` target and update README with docs generation instructions

## Capabilities

### New Capabilities

- `proto-docs`: HTML documentation generation from proto file comments

### Modified Capabilities

_(none)_

## Impact

- **Proto files**: All 8 files get doc comments (purely additive — no structural changes)
- **buf.gen.yaml**: New plugin entry for `protoc-gen-doc`
- **Generated output**: `docs/index.html` committed alongside generated code
- **CI**: `buf generate` already runs in release workflow, docs regenerated automatically
