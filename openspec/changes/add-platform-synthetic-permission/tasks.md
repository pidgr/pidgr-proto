## 1. Permission enum value

- [x] 1.1 Append `PERMISSION_PLATFORM_SYNTHETIC = 28` to the `Permission` enum in `proto/pidgr/v1/common.proto` with a doc comment matching the neighboring platform permissions
- [x] 1.2 Run `buf format -w`
- [x] 1.3 Run `buf lint` — clean
- [x] 1.4 Run `buf breaking --against .git#branch=main` — confirm additive/non-breaking

## 2. Code generation + validation

- [x] 2.1 Run `buf generate` to regenerate Go/Rust/TypeScript stubs + docs
- [x] 2.2 Verify the new value appears at 28 in `gen/go/pidgr/v1/common.pb.go`, `gen/ts/pidgr/v1/common_pb.ts`, `gen/rust/pidgr/v1/pidgr.v1.rs`, and `docs/index.md`
- [x] 2.3 Verify `gen/go` compiles (`go build ./...`)
