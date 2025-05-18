// Copyright (c) 2025 Ronan Le Meillat, SCTG Development
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

// filepath: /Users/rlemeill/Development/jsonschema-ipv6-bug/src/main.rs
use anyhow::Result;
use jsonschema::{self};
use serde_json::json;

fn main() -> Result<()> {
    // Load the schema
    let schema = json!({
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
    });

    println!("Testing IPv6 validation with jsonschema 0.30.0");
    println!("-----------------------------------------------\n");

    // Valid IPv4 and IPv6 addresses to test
    let valid_ipv4_ipv6 = vec![
        "2001:0db8:85a3:0000:0000:8a2e:0370:7334", // Full form
        "2001:db8::1",                             // Compressed form
        "::1",                                     // Localhost
        "::",                                      // Unspecified address
        "256.0.0.1",                               // Valid IPv4
    ];

    // Invalid IPv6 addresses that should fail validation
    let invalid_ipv6 = vec![
        "2001:0db8:85a3:0000:0000:8a2e:0370:7334:5678", // Too many segments (9 instead of 8)
        "2001:0db8:85a3:0000:0000:8a2e:0370:zzzz",      // Invalid characters
        "2001:0db8:85a3:0000:0000:8a2e:0370",           // Too few segments
    ];

    // Create validator
    let validator = jsonschema::draft202012::options()
        .should_validate_formats(true)
        .build(&schema)?;

    // Test valid addresses
    println!("Testing VALID IPv4 or IPv6 addresses (all should pass):");
    for address in valid_ipv4_ipv6 {
        let instance = json!({ "address": address });
        let result = validator.validate(&instance);
        println!(
            "  {} - {}",
            address,
            if result.is_ok() {
                "PASS ✓"
            } else {
                "FAIL ✗"
            }
        );
    }

    // Test invalid addresses
    println!("\nTesting INVALID IPv6 addresses (all should fail):");
    for address in invalid_ipv6 {
        let instance = json!({ "address": address });
        let result = validator.validate(&instance);
        println!(
            "  {} - {}",
            address,
            if result.is_err() {
                "FAIL ✓"
            } else {
                "PASS ✗"
            }
        );

        if result.is_ok() {
            println!("    BUG DETECTED: Address should be invalid but passed validation");
        }
    }

    Ok(())
}
