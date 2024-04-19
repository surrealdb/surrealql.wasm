# surrealql.wasm

WebAssembly utility functions for SurrealQL.

[![](https://img.shields.io/badge/status-beta-ff00bb.svg?style=flat-square)](https://github.com/surrealdb/surrealql.wasm)
[![](https://img.shields.io/badge/license-Apache_License_2.0-00bfff.svg?style=flat-square)](https://github.com/surrealdb/surrealql.wasm)
[![](https://img.shields.io/npm/v/surrealql.wasm?style=flat-square)](https://www.npmjs.com/package/surrealql.wasm)

## Importing the module

A few code snippets to showcase various ways of importing the library.

```js
import { parse, validate } from 'surrealql.wasm';
```

### Via UNPKG
```js
import { parse, validate } from 'https://unpkg.com/surrealql.wasm/lib/v1.js';
import { parse, validate } from 'https://unpkg.com/surrealql.wasm/lib/v2.js';
```
