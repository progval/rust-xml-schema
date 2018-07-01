# rust-xml-schema
XML Parser generator based on XML schemas.

## Regenerating the schema parser

```
wget https://www.w3.org/2009/XMLSchema/XMLSchema.xsd
cargo run --bin bootstrap < XMLSchema.xsd > foo.rs && cp foo.rs src/generated.rs
cargo run --bin gen < XMLSchema.xsd > foo.rs && cp foo.rs src/generated2.rs
```

