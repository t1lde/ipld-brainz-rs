
use std::fmt::{Debug};

use libipld::{DagCbor};

use crate::types::ipld::{*};
use crate::types::db::artist as DB;
use crate::types::db::{FromDB, DBInto};


#[derive(DagCbor, Debug)]
pub struct Artist {
  pub musicbrainz_id: MBID<Artist>,
  pub name: Name<Artist>,
  pub sort_name: SortName<Artist>,
  pub comment: Comment<Artist>,
  pub begin_date: Option<PartialDate>,
  pub end_date: Option<PartialDate>,
}

#[derive(DagCbor, Debug)]
pub struct Alias {
  pub name: Name<Alias>,
  pub locale: AtomStr,
}

// FromDB impl
impl FromDB for Artist {
  type DBRepr = DB::Artist;
  fn from_db (x: DB::Artist) -> Artist {

    Artist {
      musicbrainz_id: x.gid.db_into(),
      name: x.name.db_into(),
      sort_name: x.sort_name.db_into(),
      comment: x.comment.db_into(),
      begin_date: 
        PartialDate::from_db_composite(x.begin_date_year, x.begin_date_month, x.begin_date_day),
      end_date:
        PartialDate::from_db_composite(x.end_date_year, x.end_date_month, x.end_date_day),
    }
  }
}