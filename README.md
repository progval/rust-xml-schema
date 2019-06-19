# rust-xml-schema
XML Parser generator based on XML schemas.

Project status: pre-alpha

## Features

* Self-generating, meaning that all features used by `XMLSchema.xsd`
  (XML Schema's definition using XML Schema itself) are supported.
  This includes:
  * namespaces
  * group/choice/sequence/element
  * attributes
* Most datatypes (some natively implemented, some generated via `derived.nxsd`)
* Anonymous elements are given a name using a best-effort heuristic -- they
  are manually overridable

## To do

* simpleContent
* some facets
* anything related to XPath
* assertions
* time data types
* notation
* many other stuff, grep for `TODO`, `unimplemented`, `unwrap`, or `expect` in the code.
* add tests
* check conformance to the XSD specification

## Regenerating the schema parser

```
wget https://www.w3.org/2009/XMLSchema/XMLSchema.xsd -O xml-schema/XMLSchema.xsd
wget https://www.w3.org/2009/XMLSchema/derived.nxsd -O xml-schema/derived.nxsd
cargo run --package xml-schema --bin gen xml-schema/derived.nxsd xml-schema/XMLSchema.xsd > foo.rs
cp foo.rs xml-schema/src/parser.rs
cargo test
```

