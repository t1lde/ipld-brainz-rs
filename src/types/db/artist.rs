
use std::fmt::{Debug};

use serde::{Deserialize};

pub use crate::types::db::{
  Opt, DBInt, DBSmallInt, GID, DBBool, DBTime, DBFields, DBString
};

#[derive(Deserialize, Debug)]
pub struct Artist {
  pub id: DBInt,
  pub gid: GID,
  pub name: DBString,
  pub sort_name: DBString,

  pub begin_date_year: Opt<DBSmallInt>, 
  pub begin_date_month: Opt<DBSmallInt>, 
  pub begin_date_day: Opt<DBSmallInt>, 

  pub end_date_year: Opt<DBSmallInt>, 
  pub end_date_month: Opt<DBSmallInt>, 
  pub end_date_day: Opt<DBSmallInt>, 

  pub type_id: Opt<DBInt>,

  pub area: Opt<DBInt>,

  pub gender: Opt<DBInt>,

  #[serde(default)]
  pub comment: DBString,

  pub edits_pending: DBInt,

  pub last_updated: Opt<DBTime>,

  pub ended: DBBool,

  pub begin_area: Opt<DBInt>,
  pub end_area: Opt<DBInt>,
}

#[derive(Deserialize, Debug)]
pub struct Alias {
  pub id: DBInt,
  pub artist_id: DBInt,
  pub name: DBString,
  pub locale: DBString,
  pub edits_pending: DBInt,
  pub last_updated: Opt<DBTime>,
  pub type_id: Opt<DBInt>,
  pub sort_name: DBString,
  pub begin_date_year: Opt<DBSmallInt>, 
  pub begin_date_month: Opt<DBSmallInt>, 
  pub begin_date_day: Opt<DBSmallInt>, 
  pub end_date_year: Opt<DBSmallInt>, 
  pub end_date_month: Opt<DBSmallInt>, 
  pub end_date_day: Opt<DBSmallInt>, 
  pub primary_for_locale: DBBool,
  pub ended: DBBool,
}

impl DBFields for Artist {
  const FIELDS: &'static [&'static str] = 
    &["id", "gid", "name", "sort_name", 
      "begin_date_year", "begin_date_month", "begin_date_day",
      "end_date_year", "end_date_month", "end_date_day",
      "type", "area", "gender", 
      "comment", "edits_pending", "last_updated",
      "ended", "begin_area", "end_area"
     ];
  const NAME: &'static str = "artist";

  type PK = i32;

  fn pk (&self) -> i32 {
    self.id.into()
  }
}

impl DBFields for Alias {
  const FIELDS: &'static [&'static str] = 
    &["id", "artist_id", "name", "locale", 
      "edits_pending", "last_updated", "type_id",
      "sort_name",
      "begin_date_year", "begin_date_month", "begin_date_day",
      "end_date_year", "end_date_month", "end_date_day",
      "primary_for_locale", "ended"
     ];
  
  const NAME: &'static str = "alias";

  type PK = i32;

  fn pk (&self) -> i32 {
    self.id.into()
  }
}