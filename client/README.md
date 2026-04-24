# Atomic Patent TypeScript Client

Auto-generated TypeScript client from `docs/openapi.yaml`.

## Setup

```bash
npm install
```

## Generate type definitions (lightweight, no Java)

```bash
npm run generate
```

Outputs `src/client/schema.d.ts` — TypeScript types for all request/response schemas.

## Generate full SDK (requires Java)

```bash
npm run generate:sdk
```

Outputs a full `typescript-fetch` SDK to `src/client/sdk/`.

## Re-generate after API changes

Whenever `docs/openapi.yaml` changes, re-run `npm run generate` to keep the client in sync.
