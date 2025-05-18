# jsonschema Format Validation Configuration

This repository demonstrates how to properly configure the `jsonschema` crate (version 0.30.0) to correctly validate IPv6 addresses.

## Understanding Format Validation in jsonschema

The `jsonschema` crate requires explicitly enabling format validation via the `should_validate_formats(true)` method. This is not a bug but rather by design - format validation is optional according to the JSON Schema specification.

## Correct Configuration

To properly validate formats like IPv6 addresses, the validator must be configured as follows:

```rust
let validator = jsonschema::draft202012::options()
    .should_validate_formats(true)  // This line is required!
    .build(&schema)?;
```

Without this configuration, format validation checks will not be performed, and invalid IPv6 addresses would pass validation.

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

## Testing With Proper Configuration

When properly configured with `.should_validate_formats(true)`, the crate correctly:

- Validates proper IPv6 addresses like `2001:0db8:85a3:0000:0000:8a2e:0370:7334`
- Rejects invalid addresses like `2001:0db8:85a3:0000:0000:8a2e:0370:7334:5678` (too many segments)

## Steps to Demonstrate

1. Clone this repository
2. Run `cargo run`
3. Observe that invalid IPv6 addresses correctly fail validation

## Additional Information

The solution was confirmed by the crate maintainer in [GitHub Issue #743](https://github.com/Stranger6667/jsonschema/issues/743#issuecomment-2888889116).

## Technical Details

A valid IPv6 address must:

- Have exactly 8 segments (when fully expanded)
- Each segment must be a valid hexadecimal value between 0 and FFFF
- Can be compressed using the `::` notation to represent one or more groups of zeros

## Environment Information

- jsonschema: 0.30.0
- Rust: 1.86.0
- OS: Darwin Kernel Version 24.5.0 (x86_64)
