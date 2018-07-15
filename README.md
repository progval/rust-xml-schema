# rust-xml-schema
XML Parser generator based on XML schemas.

## Regenerating the schema parser

```
wget https://www.w3.org/2009/XMLSchema/XMLSchema.xsd -O xml-schema/XMLSchema.xsd
wget https://www.w3.org/2009/XMLSchema/derived.nxsd -O xml-schema/derived.nxsd
cargo run --package xml-schema --bin gen xml-schema/derived.nxsd xml-schema/XMLSchema.xsd > foo.rs
cargo test
```

