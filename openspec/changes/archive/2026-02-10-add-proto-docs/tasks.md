## 1. Proto Comments

- [x] 1.1 Add doc comments to `common.proto` — all enums, messages, and fields
- [x] 1.2 Add doc comments to `campaign.proto` — service, RPCs, messages, and fields
- [x] 1.3 Add doc comments to `template.proto` — service, RPCs, messages, and fields
- [x] 1.4 Add doc comments to `action.proto` — service, RPCs, messages, and fields
- [x] 1.5 Add doc comments to `inbox.proto` — service, RPCs, messages, and fields
- [x] 1.6 Add doc comments to `device.proto` — service, RPCs, messages, and fields
- [x] 1.7 Add doc comments to `user_org.proto` — service, RPCs, messages, and fields
- [x] 1.8 Add doc comments to `render.proto` — service, RPCs, messages, and fields

## 2. Doc Generation Setup

- [x] 2.1 Add `protoc-gen-doc` plugin to `buf.gen.yaml` (remote: `buf.build/community/pseudomuto-doc`, out: `docs`, opt: `markdown,index.md`)
- [x] 2.2 Add `docs/** linguist-generated=true` to `.gitattributes`
- [x] 2.3 Update release workflow to commit `docs/` alongside `gen/` on release
- [x] 2.4 Run `buf generate` and verify `docs/index.md` is generated with all services

## 3. Verification

- [x] 3.1 Run `buf build` and `buf lint` to verify comments don't break anything
- [x] 3.2 Verify `docs/index.md` lists all 7 services, RPCs, and types with descriptions
