# jsonschema IPv6 Validation Bug

This repository demonstrates a bug in the `jsonschema` crate (version 0.30.0) where it incorrectly validates certain IPv6 addresses.

## Bug Description

The `jsonschema` crate (v0.30.0) fails to properly validate IPv6 addresses when using the `format: "ipv6"` validation in a JSON Schema with draft-2020-12. Specifically, it allows invalid IPv6 addresses like `2001:0db8:85a3:0000:0000:8a2e:0370:7334:5678` to pass validation, despite having too many segments (9 instead of the maximum 8).

## Steps to Reproduce

1. Clone this repository
2. Run `cargo run`

## Expected Output

The validation should fail for invalid IPv6 addresses like `2001:0db8:85a3:0000:0000:8a2e:0370:7334:5678`.

## Validation Schema

```json
{
  "$schema": "http://json-schema.org/draft-2020-12/schema",
  "type": "object",
  "properties": {
    "address": {
      "type": "string",
      "anyOf": [
        {
          "format": "ipv4"
        },
        {
          "format": "ipv6"
        }
      ]
    }
  },
  "required": [
    "address"
  ]
}
```

## Testing with jsonschemavalidator.net

https://www.jsonschemavalidator.net/s/W2b3vHJC

## Actual Output

The validation incorrectly passes for invalid IPv6 addresses.

## Environment Information

- jsonschema: 0.30.0
- Rust: 1.86.0 (but likely affects all versions)
- OS: Darwin Kernel Version 24.5.0 (x86_64)

## Technical Analysis

The `jsonschema` crate's implementation of the IPv6 format validation does not properly check if the IPv6 address has the correct number of segments. IPv6 addresses must have exactly 8 segments, but the current implementation allows addresses with more segments to pass validation.

A valid IPv6 address must:
- Have exactly 8 segments (when fully expanded)
- Each segment must be a valid hexadecimal value between 0 and FFFF
- Can be compressed using the `::` notation to represent one or more groups of zeros

In RFC 5952, the format of IPv6 addresses is clearly defined, and addresses like `2001:0db8:85a3:0000:0000:8a2e:0370:7334:5678` are invalid because they have 9 segments.