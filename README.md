# rust-xml-schema
XML Parser generator based on XML schemas.

## Regenerating the schema parser

```
wget https://www.w3.org/2009/XMLSchema/XMLSchema.xsd
cargo run --features=bootstrap --bin bootstrap < XMLSchema.xsd > src/generated.rs
cargo run --bin gen < XMLSchema.xsd > src/generated2.rs
cargo test
```

