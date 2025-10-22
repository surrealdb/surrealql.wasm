use crate::err::Error;
use serde::ser::Serialize;
use serde_json::ser::PrettyFormatter;
use serde_json::Value as Json;
use serde_wasm_bindgen::from_value;
use surrealdb_core::dbs::capabilities::Targets;
use surrealdb_core::dbs::Capabilities;
use surrealdb_types::ToSql;
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::prelude::*;
use web_sys::js_sys::Uint8Array;

macro_rules! caps {
	() => {
		&Capabilities::all().with_experimental(Targets::All)
	};
}

#[wasm_bindgen(start)]
pub fn setup() {
	crate::log::init();
}

#[wasm_bindgen]
pub fn parse(_sql: &str) -> Result<JsValue, Error> {
	Err(Error::from("Not implemented: Use validate() instead"))
}

#[wasm_bindgen]
pub fn extract_tables_from_kind(kind_sql: &str) -> Result<Vec<String>, Error> {
	let tables = surrealdb_core::syn::extract_tables_from_kind(kind_sql)?;
	Ok(tables)
}

#[wasm_bindgen]
pub fn format(sql: &str, pretty: bool) -> Result<String, Error> {
	let ast = surrealdb_core::syn::parse(sql)?;
	Ok(match pretty {
		true => ast.to_sql(), // TODO: Add pretty formatting back in.
		false => ast.to_sql(),
	})
}

#[wasm_bindgen]
pub fn validate(sql: &str) -> Result<(), Error> {
	surrealdb_core::syn::parse_with_capabilities(sql, caps!())?;
	Ok(())
}

#[wasm_bindgen]
pub struct Value {
	inner: surrealdb_types::Value,
}

#[wasm_bindgen]
impl Value {
	/// Create a new SurrealQL value
	///
	/// ```js
	/// const value = Value.from_string("{ test: true }");
	/// ```
	#[wasm_bindgen]
	pub fn from_string(val: String) -> Result<Value, Error> {
		let val =
			surrealdb_core::syn::value(&val).map_err(|_| "Failed to parse value from string")?;
		Ok(Value {
			inner: val,
		})
	}
	/// Create a new SurrealQL value from a JSON string
	/// Errors when it encounters non-json values
	///
	/// ```js
	/// const value = Value.from_json_string("{ test: true }");
	/// ```
	#[wasm_bindgen]
	pub fn from_json_string(val: String) -> Result<Value, Error> {
		let val = surrealdb_core::syn::json(&val)
			.map_err(|_| "Failed to parse value from JSON string")?;
		Ok(Value {
			inner: val,
		})
	}

	/// Create a new SurrealQL value
	///
	/// ```js
	/// const value = Value.from_json({ test: true });
	/// ```
	#[wasm_bindgen]
	pub fn from_json(val: JsValue) -> Result<Value, Error> {
		let val = surrealdb_core::syn::json(&from_value::<Json>(val)?.to_string())?;
		Ok(Value {
			inner: val,
		})
	}

	/// Create a new SurrealQL value
	///
	/// ```js
	/// const value = Value.from_cbor(<binary data>);
	/// ```
	#[wasm_bindgen]
	pub fn from_cbor(val: Uint8Array) -> Result<Value, Error> {
		let val: surrealdb_types::Value =
			surrealdb_core::rpc::format::cbor::decode(val.to_vec().as_slice())
				.map_err(|_| Error::from("Received invalid binary data"))?;
		Ok(Value {
			inner: val,
		})
	}

	/// Format a SurrealQL value
	///
	/// ```js
	/// const value = Value.from_json({ test: true });
	/// const output = value.format(true);
	///
	#[wasm_bindgen]
	pub fn format(&self, pretty: bool) -> Result<String, Error> {
		Ok(match pretty {
			true => self.inner.to_sql(), // TODO: Add pretty formatting back in.
			false => self.inner.to_sql(),
		})
	}

	/// Format a SurrealQL value into JSON
	///
	/// ```js
	/// const value = Value.from_json({ test: true });
	/// const output = value.json(true);
	///
	#[wasm_bindgen]
	pub fn json(&self, pretty: bool) -> Result<String, Error> {
		Ok(match pretty {
			true => {
				let mut buf = Vec::new();
				let mut serializer = serde_json::Serializer::with_formatter(
					&mut buf,
					PrettyFormatter::with_indent(b"\t"),
				);
				self.inner.clone().into_json_value().serialize(&mut serializer).unwrap();
				String::from_utf8(buf).unwrap()
			}
			false => self.inner.clone().into_json_value().to_string(),
		})
	}

	/// Return a parsed SurrealQL value as CBOR
	///
	/// ```js
	/// const value = Value.from_string("{ test: true }");
	/// const output = value.to_cbor();
	///
	#[wasm_bindgen]
	pub fn to_cbor(&self) -> Result<Uint8Array, Error> {
		// Into CBOR value
		let cbor = surrealdb_core::rpc::format::cbor::encode(self.inner.clone())
			.map_err(|_| "Failed to convert Value to CBOR")?;
		Ok(Uint8Array::from(cbor.as_slice()))
	}
}
