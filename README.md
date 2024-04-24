# surrealql.wasm

WebAssembly utility functions for SurrealQL.

[![](https://img.shields.io/badge/status-beta-ff00bb.svg?style=flat-square)](https://github.com/surrealdb/surrealql.wasm)
[![](https://img.shields.io/badge/license-Apache_License_2.0-00bfff.svg?style=flat-square)](https://github.com/surrealdb/surrealql.wasm)
[![](https://img.shields.io/npm/v/surrealql.wasm?style=flat-square)](https://www.npmjs.com/package/surrealql.wasm)

## Importing the module

A few code snippets to showcase various ways of importing the library.

```js
import { SurrealQL, Value } from 'surrealql.wasm/v1';
import { SurrealQL, Value } from 'surrealql.wasm/v2';
```

### Via UNPKG
```js
import { SurrealQL, Value } from 'https://unpkg.com/surrealql.wasm/lib/v1.js';
import { SurrealQL, Value } from 'https://unpkg.com/surrealql.wasm/lib/v2.js';
```

## Example usage

```js
import { SurrealQL, Value } from 'surrealql.wasm/v1';

// Creating a SurrealQL Value
const value = Value.from_string("{ id: \"person:tobie\" }");
const value = Value.from_json({ id: "person:tobie" });
const value = Value.from_cbor(/* Uint8Array */);

// Formatting a value
value.format();
value.format(true); // Pretty
value.json();
value.json(true); // Pretty

// Converting a value to CBOR, represented as a Uint8Array
value.to_cbor();

// Parsing queries
SurrealQL.parse("SELECT * FROM person");

// Formatting queries
SurrealQL.format("SELECT * FROM person");

// Validating queries or values
SurrealQL.validate("SELECT * FROM person");
SurrealQL.validate_where("something = true");
SurrealQL.validate_value("[1, 2, 3]");
SurrealQL.validate_thing("person:tobie");
SurrealQL.validate_idiom("person:tobie->likes[WHERE something]");
SurrealQL.validate_subquery("SELECT * FROM person");
```
