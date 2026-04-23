# IP Metadata Schema

The `metadata` field on `IpRecord` is an optional byte payload (max 1 KB) set via `set_ip_metadata`.

## Recommended Format

Use newline-delimited `key:value` pairs encoded as UTF-8:

```
title:My Invention
description:A method for doing X using Y
category:mechanical
version:1.0
```

## Reserved Keys

| Key | Description |
|-----|-------------|
| `title` | Short human-readable name (max 100 chars) |
| `description` | Brief summary of the invention |
| `category` | Domain tag, e.g. `software`, `mechanical`, `chemical` |
| `version` | Semantic version of the design |
| `license_url` | URL to a standard license (e.g. SPDX identifier or IPFS CID) |

## Notes

- Keys and values must not contain newlines or colons (except the delimiter colon).
- Unknown keys are ignored by the contract — clients may extend the schema freely.
- The contract stores raw bytes; encoding/decoding is the caller's responsibility.
- For structured data, JSON encoded as UTF-8 is also acceptable within the 1 KB limit.
