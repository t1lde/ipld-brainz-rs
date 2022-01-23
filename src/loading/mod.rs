use std::path::{Path, PathBuf};
use std::fs::File;
use std::collections::{HashMap};
use std::hash::{Hash};
use std::iter::{Map};

use csv::{ReaderBuilder, Reader};
use serde::de::{Deserialize, DeserializeOwned};
use anyhow::{Error, Context, anyhow};


use crate::types::db as DB;
use crate::types::ipld::artist::{Artist, Alias};
use crate::types::db::{DBFields, FromDB, DBInt };


fn load_table<A, I> (base: PathBuf) -> 
  where
    A: DBFields,
    A: DeserializeOwned,
    A::PK: Hash,
    A::PK: Eq,
    A::PK: Clone,
{
  let path: PathBuf = base.join::<PathBuf>(A::NAME.into());
  let mut reader: Reader<File> =
    ReaderBuilder::new()
      .delimiter(b'\t')
      .flexible(false)
      .from_path(path)
      .with_context(
        || {
          "failed to create reader from path:"
        })?;

  reader.set_headers(A::FIELDS.into());

  reader
    .deserialize::<A>()
    .map(|x| { 
      match x {
        Ok(y) => Ok((y.pk(), y)),
        Err(e) => Err(anyhow!(e))
      }})
}


fn load_alias (base: PathBuf) ->
  Result<HashMap<i32, Alias>, Error>
{
  let alias_table 
    = load_table::<Alias>();

  let artist_table = load_table::<Artist>();
}
//
//fn load_artist (base: PathBuf) -> 
//  Result <(HashMap<i32, Artist>, HashMap<i32, Alias>), Error> 
//{
//  let alias = 
//}

