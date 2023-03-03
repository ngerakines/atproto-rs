use serde::{Deserialize, Serialize};
use serde_json::Value;

/*
interface LexiconDoc {
  lexicon: 1
  id: string // an NSID
  type: 'query' | 'procedure' | 'record' | 'token'
  revision?: number
  description?: string
  defs?: JSONSchema

  // if type == record
  key?: string
  record?: JSONSchema

  // if type == query or procedure
  parameters?: Record<string, XrpcParameter>
  input?: XrpcBody
  output?: XrpcBody
  errors?: XrpcError[]
}
 */

#[derive(Serialize, Deserialize)]
struct LexiconDoc {
    lexicon: u8,
    id: String,
    #[serde(rename = "type")]
    lexicon_doc_type: String,
    revision: Option<u32>,
    description: Option<String>,
    defs: Option<Value>, // jsonschema

    // if type == record
    key: Option<String>,
    record: Option<Value>, // jsonschema

    // if type == query or procedure
    parameters: Option<Value>, // Record<string, XrpcParameter>
    input: Option<Value>,      // XrpcBody
    output: Option<Value>,     // XrpcBody
    errors: Option<Value>,     // XrpcError[]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexicon_doc_parse() {
        let data = r#"{
            "lexicon": 1,
            "id": "com.example.getProfile",
            "type": "query",
            "parameters": {
              "user": {"type": "string", "required": true}
            },
            "output": {
              "encoding": "application/json",
              "schema": {
                "type": "object",
                "required": ["did", "name"],
                "properties": {
                  "did": {"type": "string"},
                  "name": {"type": "string"},
                  "displayName": {"type": "string", "maxLength": 64},
                  "description": {"type": "string", "maxLength": 256}
                }
              }
            }
          }"#;

        // Parse the string of data into serde_json::Value.
        let v: LexiconDoc = serde_json::from_str(data).expect("JSON was not well-formatted");
        assert_eq!(v.lexicon, 1);
        assert_eq!(v.id, "com.example.getProfile");
    }
}
