## Design Decisions

### D1: protoc-gen-doc via buf remote plugin

**Decision:** Use `buf.build/community/pseudomuto-doc` as a remote plugin in `buf.gen.yaml` to generate HTML documentation.

**Rationale:** Already available on the Buf Schema Registry — no local tool installation needed. Generates a self-contained HTML page with cross-linked types, service/RPC listing, and field tables. Integrates with the existing `buf generate` workflow so docs stay in sync with code generation.

### D2: Markdown format

**Decision:** Generate `docs/index.md` using the built-in Markdown template.

**Rationale:** Markdown renders natively on GitHub — no hosting needed. Opening the file on GitHub gives a browsable, formatted reference page. It also diffs better than HTML when reviewing changes.

### D3: Proto comments as the source of truth

**Decision:** Add `//` doc comments to all services, RPCs, messages, enums, and fields in the proto files. The generated docs pull directly from these comments.

**Rationale:** Comments live next to the code they describe, so they're more likely to stay up to date. protoc-gen-doc extracts leading comments for each element.

### D4: Committed docs, hidden from diffs

**Decision:** Commit `docs/index.md` to the repo and add `docs/` to `.gitattributes` with `linguist-generated=true`.

**Rationale:** Same pattern used for `gen/` directories — makes docs available without running `buf generate`, keeps PR diffs focused on proto source changes.

### D5: Docs regenerated in release workflow

**Decision:** The release workflow (`.github/workflows/release.yml`) already runs `buf generate` — since the doc plugin is in `buf.gen.yaml`, docs are regenerated automatically on every release. The commit step is updated to include `docs/` alongside `gen/`.

**Rationale:** Docs always match the released proto version with zero extra CI config. The existing "Commit generated code" step just needs to `git add docs/` in addition to `gen/`.
