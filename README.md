# rust-xml-schema
XML Parser generator based on XML schemas.

## Regenerating the schema parser

```
wget https://www.w3.org/2009/XMLSchema/XMLSchema.xsd
cargo run --bin gen XMLSchema.xsd > foo.rs && cp foo.rs src/parser.rs
cargo test
```

