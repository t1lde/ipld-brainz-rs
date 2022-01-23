
use std::fmt::{Debug};

use std::marker::{PhantomData};
use std::convert::{From, TryInto};
use std::io::{Read, Write, Seek};

use uuid::Uuid;
use string_cache::{DefaultAtom};
use derive_more::{From, Into};

use libipld::{DagCbor};
use libipld::codec::{Encode, Decode, Codec};
use libipld::cbor::{DagCborCodec};
use libipld::error::{Error};

use crate::types::db::{GID, DBString, FromDB, Opt, DBSmallInt};

pub mod artist;

#[derive(Debug)]
struct Tag<A> (PhantomData<A>);

#[derive(From, Into, Debug)]
pub struct AtomStr (DefaultAtom);

#[derive(Debug)]
pub struct MBID <A> {
  pub id: Uuid,
  _tag: Tag<A>,
}

#[derive(DagCbor, Debug)]
pub struct Name <A> { 
  pub name: AtomStr,
  _tag: Tag<A>,
}

#[derive(DagCbor, Debug)]
pub struct SortName <A> {
  pub sort_name: AtomStr,
  _tag: Tag<A>,
}

#[derive(DagCbor, Debug)]
pub struct Comment <A> {
  pub comment: AtomStr,
  _tag: Tag<A>,
}

#[derive(DagCbor, Debug)]
pub enum PartialDate {
  YMD {y: i16, m: i16, d: i16},
  YM {y: i16, m: i16},
  Y {y: i16},
}

//// Trait Impls
// FromDB

impl<A> FromDB for MBID<A> {
  type DBRepr = GID;

  fn from_db (x: GID) -> MBID<A> {
    MBID {
      id: x.into(),
      _tag: Tag(PhantomData),
    }
  }

}

impl FromDB for AtomStr {
  type DBRepr = DBString;

  fn from_db (x: DBString) -> AtomStr {
    AtomStr(x.into())
  }
}

impl<A> FromDB for Name<A> {
  type DBRepr = DBString;

  fn from_db (x: DBString) -> Name<A> {
    Name {
      name: AtomStr::from_db(x),
      _tag: Tag(PhantomData),
    }
  }
}


impl<A> FromDB for SortName<A> {
  type DBRepr = DBString;

  fn from_db (x: DBString) -> SortName<A> {
    SortName {
      sort_name: AtomStr::from_db(x),
      _tag: Tag(PhantomData),
    }
  }

}

impl<A> FromDB for Comment<A> {
  type DBRepr = DBString;

  fn from_db (x: DBString) -> Comment <A> {
    Comment {
      comment: AtomStr::from_db(x),
      _tag: Tag(PhantomData),
    }
  }

}

// IPLD Codecs

impl<A, C> Decode<C> for Tag<A>
  where 
    C: Codec,
{
  fn decode<R>(_c: C, _r: &mut R) -> Result<Tag<A>, Error> 
    where
      R: Read + Seek,
  {
    Ok(Tag(PhantomData))
  }
}

impl <A, C> Encode<C> for Tag<A>
  where
    C: Codec,
{
  fn encode<W>(&self, _c: C, _w: &mut W) -> Result<(), Error>
    where
      W: Write,
  {
    Ok(())
  }
}

impl Decode<DagCborCodec> for AtomStr
{
  fn decode<R>(c: DagCborCodec, r: &mut R) -> Result<AtomStr, Error> 
    where
      R: Read + Seek,
  {
    let s: String = String::decode(c, r)?;
    Ok(AtomStr(DefaultAtom::from(s)))
      
  }
}

impl Encode<DagCborCodec> for AtomStr
{
  fn encode<W>(&self, c: DagCborCodec, w: &mut W) -> Result<(), Error>
    where
      W: Write,
  {
      let s: String = self.0.to_string();
      s.encode(c, w)
  }
}

impl<A> Decode<DagCborCodec> for MBID<A> {
  fn decode<R>(c: DagCborCodec, r: &mut R) -> Result<MBID<A>, Error> 
    where
      R: Read + Seek,
  {
    let bytes: Box<[u8]> = Box::<[u8]>::decode(c, r)?;
    let arr: [u8; 16] = 
      (*bytes)
        .try_into()
        .map_err(Error::new)?;

    let id: Uuid = Uuid::from_bytes(arr);

    Ok(
      MBID {
       id: id,
       _tag: Tag(PhantomData),
      }
    )
  }
}

impl<A> Encode<DagCborCodec> for MBID<A> {
  fn encode<W>(&self, c: DagCborCodec, w: &mut W) -> Result<(), Error>
    where
      W: Write,
  {
      let u: [u8; 16] = *self.id.as_bytes();
      u.encode(c, w)
  }
}

// PartialDate impl

impl PartialDate {
  pub fn from_db_composite (y: Opt<DBSmallInt>, m: Opt<DBSmallInt>, d: Opt<DBSmallInt>) -> Option<PartialDate> {
    match (y, m, d) {
      (Opt(Some(year)), Opt(Some(month)), Opt(Some(day))) 
        => Some(PartialDate::YMD{y: year.into(), m: month.into(), d: day.into()}),
      (Opt(Some(year)), Opt(Some(month)), _)
        => Some(PartialDate::YM{y: year.into(), m: month.into()}),
      (Opt(Some(year)), _, _) => Some(PartialDate::Y{y: year.into()}),
      _ => None
    }

  }
}