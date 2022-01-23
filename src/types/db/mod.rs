use std::fmt::{Debug, Display};
use std::str::FromStr;

use uuid::Uuid;
use chrono::{DateTime, Utc, self};
use serde::{de, Deserialize, Deserializer};
use derive_more::{From, Into, FromStr, Display};
use string_cache::{DefaultAtom};

pub mod artist;

// Basics
#[derive(Debug, Display)]
pub struct DBBool (bool);

#[derive(Debug, Deserialize, From, Into, Display)]
pub struct DBString (DefaultAtom);

// Integers
#[derive(Debug, Deserialize, From, Into, FromStr, Display, Copy, Clone)]
pub struct DBInt (i32);

#[derive(Debug, Deserialize, From, Into, FromStr, Display)]
pub struct DBSmallInt (i16);

// UUID
#[derive(Debug, Deserialize, From, Into, FromStr, Display)]
pub struct GID (Uuid);

// Date & Time
#[derive(Debug, Display)]
pub struct DBTime (DateTime<Utc>);

// Recursive
#[derive(Debug, Display)]
pub struct Opt<A> (pub Option<A>);

// Composite
#[derive(Debug, Deserialize, Display)]
pub struct CompOf<A> (A);

// Traits

pub trait DBFields {
  const FIELDS: &'static [&'static str];
  const NAME: &'static str;
  type PK;

  fn pk(&self) -> Self::PK;
}

pub trait FromDB {
  type DBRepr;
  fn from_db (x: Self::DBRepr) -> Self;
}

pub trait DBInto<A> 
  //where
  //  A::DBRepr = Self,
{
  fn db_into(self) -> A; 
}

impl<A> DBInto<A> for A::DBRepr 
  where
    A: FromDB,
    
{
  fn db_into(self) -> A {
    A::from_db(self)
  }
}

//////////////////////////////
// Various instances

impl Default for DBString {
  fn default() -> Self {
    DBString("".into())
  }
}

//////////////////////////////
// Conversion instances

impl FromStr for DBBool where

{
  type Err = String;

  fn from_str (s: &str) -> Result<Self, Self::Err>{
    match s {
      "t" => Ok(DBBool(true)),
      "f" => Ok(DBBool(false)),
      _ => Err(format!("Invalid Bool: expected (t|f), found {}", s))
    }
  }

}

impl<'de> Deserialize<'de> for DBBool
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
      let s = 
        String::deserialize(deserializer)?;

      DBBool::from_str(&s)
        .map_err(de::Error::custom)
    }
}


impl FromStr for DBTime where
{
  type Err = chrono::format::ParseError;

  fn from_str (s: &str) -> Result<Self, Self::Err> {

    DateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S.%f%#z".into())
      .map(|x| { DBTime(x.into()) })
  }
}

impl<'de, T> Deserialize<'de> for Opt<T>
  where
    T: FromStr,
    T::Err: Display,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        let s = 
          String::deserialize(deserializer)?;

        match s.as_ref() {
          "\\N" => Ok(Opt(None)),
          _ => 
            FromStr::from_str(&s)
              .map_err(de::Error::custom)
              .map(|x| { Opt(Some(x)) })
        }
    }
}





