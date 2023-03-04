use async_trait::async_trait;
use parking_lot::Mutex;
use serde::{de, Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::marker::PhantomData;
use std::path::Path;

use crate::error::{AtProtoError, Result};

fn string_or_seq_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct StringOrVec(PhantomData<Vec<String>>);

    impl<'de> de::Visitor<'de> for StringOrVec {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![value.to_owned()])
        }

        fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            de::Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(StringOrVec(PhantomData))
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LexiconPrimitive {
    Boolean { description: Option<String> },
    Number { description: Option<String> },
    Integer { description: Option<String> },
    String { description: Option<String> },
    Array { description: Option<String> },
    // Ref { description: Option<String>, #[serde(rename(deserialize = "ref"))] ref_link: String },
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct LexXrpcBody {
    pub description: Option<String>,
    #[serde(deserialize_with = "string_or_seq_string")]
    pub encoding: Vec<String>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub struct LexXrpcParameters {
    pub description: Option<String>,
    pub required: Option<Vec<String>>,
    pub properties: Option<HashMap<String, LexiconPrimitive>>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum LexiconType {
    Token {
        description: Option<String>,
    },
    Query {
        description: Option<String>,
        parameters: Option<LexXrpcParameters>,
        output: Option<LexXrpcBody>,
        // errors: Option<Vec<String>>,
    },
    Object {
        description: Option<String>,
        required: Option<Vec<String>>,
    },
    Procedure {
        description: Option<String>,
    },
    Record {
        description: Option<String>,
    },
    Subscription {
        description: Option<String>,
    },
    Boolean { description: Option<String> },
    Number { description: Option<String> },
    Integer { description: Option<String> },
    String { description: Option<String> },
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct LexiconDoc {
    pub lexicon: u8,
    pub id: String,
    pub revision: Option<u32>,
    pub description: Option<String>,
    pub defs: HashMap<String, LexiconType>,
}

impl LexiconDoc {
    pub fn validate(&self) -> Result<()> {
        if self.lexicon != 1 {
            return Err(anyhow::anyhow!("lexicon must be 1"));
        }
        Ok(())
    }
}

#[async_trait]
pub trait LexiconDocRegistry: Sync + Send {
    async fn get(&self, lexicon_id: String) -> Result<Option<LexiconDoc>>;
    async fn put(&self, lexicon: &LexiconDoc) -> Result<()>;
}

pub struct NullLexiconDocRegistry;

impl Default for NullLexiconDocRegistry {
    fn default() -> Self {
        NullLexiconDocRegistry
    }
}

#[async_trait]
impl LexiconDocRegistry for NullLexiconDocRegistry {
    async fn get(&self, _: String) -> Result<Option<LexiconDoc>> {
        Ok(None)
    }

    async fn put(&self, _: &LexiconDoc) -> Result<()> {
        Ok(())
    }
}

#[derive(Default)]
struct InnerMemoryLexiconDocRegistry {
    lexicons: HashMap<String, LexiconDoc>,
}

#[derive(Default)]
pub struct MemoryLexiconDocRegistry {
    inner: Mutex<RefCell<InnerMemoryLexiconDocRegistry>>,
}

#[async_trait]
impl LexiconDocRegistry for MemoryLexiconDocRegistry {
    async fn get(&self, lexicon_id: String) -> Result<Option<LexiconDoc>> {
        let inner_lock = self.inner.lock();
        let inner = inner_lock.borrow();

        match inner.lexicons.get(&lexicon_id) {
            Some(val_ref) => Ok(Some(val_ref.to_owned())),
            None => Ok(None),
        }
    }

    async fn put(&self, lexicon: &LexiconDoc) -> Result<()> {
        let inner_lock = self.inner.lock();
        let mut inner = inner_lock.borrow_mut();

        inner.lexicons.insert(lexicon.id.clone(), lexicon.clone());

        Ok(())
    }
}

pub fn lexicon_from_file<P: AsRef<Path>>(path: P) -> Result<LexiconDoc, AtProtoError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let lexicon: LexiconDoc = serde_json::from_reader(reader)?;
    //   lexicon.validate()?;
    Ok(lexicon)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexicon::LexiconDoc, lexicon::LexiconDocRegistry};
    use std::path::PathBuf;
    use walkdir::WalkDir;

    #[tokio::test]
    async fn lexicon_manager_crud() {
        let lexicon_manager = MemoryLexiconDocRegistry::default();
        let lexicon = LexiconDoc {
            lexicon: 1,
            id: "com.example.getProfile".to_string(),
            revision: None,
            description: None,
            defs: HashMap::new(),
        };

        lexicon_manager.put(&lexicon).await.expect("put failed");

        let lexicon2 = lexicon_manager
            .get("com.example.getProfile".to_string())
            .await
            .expect("get failed");

        assert_eq!(lexicon, lexicon2.unwrap());
    }

    // #[test]
    // fn lexicon_doc_validate() {
    //   let mut test_data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //   test_data_dir.push("testdata");

    //   let lexicon = lexicon_from_file(test_data_dir.join("lexicon.json")).expect("lexicon.json failed");
    //   assert!(lexicon.validate().is_ok());
    //   assert_eq!(lexicon.lexicon, 1);
    //   assert_eq!(lexicon.id, "com.atproto.account.create");
    //   assert_eq!(lexicon.defs, HashMap::new());
    // }

    #[test]
    fn lexicon_parse_token() {
        let mut test_data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data_dir.push("testdata");

        let lexicon = lexicon_from_file(test_data_dir.join("lexicon_token.json"))
            .expect("lexicon.json failed");
        assert!(lexicon.validate().is_ok());
        assert_eq!(lexicon.lexicon, 1);
        assert_eq!(lexicon.id, "com.socialapp.actorUser");
        assert_eq!(
            lexicon.defs,
            HashMap::from([(
                "main".to_string(),
                LexiconType::Token {
                    description: Some("Actor type of 'User'".to_string())
                }
            )])
        );
    }

    #[test]
    fn parse_lexicons() {
        let mut test_data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_data_dir.push("lexicons");

        for entry in WalkDir::new(test_data_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() && entry.path().extension().unwrap() == "json" {
                let lexicon = lexicon_from_file(entry.path())
                    .expect(format!("parse failed failed: {:?}", entry.path()).as_str());
                assert!(lexicon.validate().is_ok());
            }
        }
    }

    // #[test]
    // fn lexicon_parse_query() {
    //     let mut test_data_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    //     test_data_dir.push("testdata");

    //     let lexicon = lexicon_from_file(test_data_dir.join("lexicon_query.json"))
    //         .expect("lexicon.json failed");
    //     assert!(lexicon.validate().is_ok());
    //     assert_eq!(lexicon.lexicon, 1);
    //     assert_eq!(lexicon.id, "com.atproto.handle.resolve");

    //     assert_eq!(
    //         lexicon.defs,
    //         HashMap::from([(
    //             "main".to_string(),
    //             LexiconType::Query {
    //                 description: Some("Provides the DID of a repo.".to_string()),
    //                 parameters: Some(
    //                     HashMap::from([("handle".to_string(), LexiconPrimitive::String {
    //                         description: Some("The handle to resolve. If not supplied, will resolve the host's own handle.".to_string())
    //                     })])
    //                 ),
    //                 output: Some(LexXrpcBody {
    //                     description: None,
    //                     encoding: vec!["application/json".to_string()],
    //                 }),
    //                 errors: None,
    //             }
    //         )])
    //     );
    // }
}
