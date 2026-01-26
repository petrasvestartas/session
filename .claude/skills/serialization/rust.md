# Serialization - Rust

## JSON (serde_json)

```rust
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
pub struct ClassName {
    pub guid: String,
    pub name: String,
    #[serde(rename = "type")]
    type_name: String,
    #[serde(rename = "x")]
    _x: f64,
    #[serde(rename = "y")]
    _y: f64,
    #[serde(rename = "z")]
    _z: f64,
}

impl ClassName {
    pub fn jsondump(&self) -> serde_json::Value {
        serde_json::json!({
            "guid": self.guid,
            "name": self.name,
            "type": "ClassName",
            "x": self._x,
            "y": self._y,
            "z": self._z
        })
    }

    pub fn jsonload(data: &serde_json::Value) -> Self {
        serde_json::from_value(data.clone()).unwrap()
    }

    pub fn json_dump(&self, filename: &str) {
        let json = serde_json::to_string_pretty(&self.jsondump()).unwrap();
        fs::write(filename, json).unwrap();
    }

    pub fn json_load(filename: &str) -> Self {
        let data = fs::read_to_string(filename).unwrap();
        let v: serde_json::Value = serde_json::from_str(&data).unwrap();
        Self::jsonload(&v)
    }
}
```

## Protobuf (prost)

```rust
use prost::Message;

// Generated from .proto file
mod proto {
    include!(concat!(env!("OUT_DIR"), "/session.rs"));
}

impl ClassName {
    pub fn to_proto(&self) -> Vec<u8> {
        let msg = proto::ClassName {
            guid: self.guid.clone(),
            name: self.name.clone(),
            x: self._x,
            y: self._y,
            z: self._z,
        };
        msg.encode_to_vec()
    }

    pub fn from_proto(data: &[u8]) -> Self {
        let msg = proto::ClassName::decode(data).unwrap();
        Self {
            guid: msg.guid,
            name: msg.name,
            _x: msg.x,
            _y: msg.y,
            _z: msg.z,
        }
    }

    pub fn protobuf_dump(&self, filename: &str) {
        fs::write(filename, self.to_proto()).unwrap();
    }

    pub fn protobuf_load(filename: &str) -> Self {
        let data = fs::read(filename).unwrap();
        Self::from_proto(&data)
    }
}
```
