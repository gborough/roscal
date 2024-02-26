use std::env;

use chrono::prelude::*;
use lazy_static::lazy_static;
use pulldown_cmark::{Options, Parser};
use regex::Regex;
use serde::{de, ser::SerializeSeq, Deserialize, Serializer};
use uriparse::{uri::URI, uri_reference::URIReference};

lazy_static! {
    static ref BASE64: Regex = Regex::new(r"^[0-9A-Za-z+/]+={0,2}$").unwrap();
    static ref DATE: Regex = Regex::new(r"^(((2000|2400|2800|(19|2[0-9](0[48]|[2468][048]|[13579][26])))-02-29)|(((19|2[0-9])[0-9]{2})-02-(0[1-9]|1[0-9]|2[0-8]))|(((19|2[0-9])[0-9]{2})-(0[13578]|10|12)-(0[1-9]|[12][0-9]|3[01]))|(((19|2[0-9])[0-9]{2})-(0[469]|11)-(0[1-9]|[12][0-9]|30)))(Z|[+-][0-9]{2}:[0-9]{2})?$").unwrap();
    static ref EMAIL: Regex = Regex::new(r"^.+@.+$").unwrap();
    static ref TOKEN: Regex =
        Regex::new(r"^(\p{L}|_)(\p{L}|\p{N}|[.\-_])*$").unwrap();
    static ref SHA_SHA3_224: Regex = Regex::new(r"^[0-9a-fA-F]{28}$").unwrap();
    static ref SHA_SHA3_256: Regex = Regex::new(r"^[0-9a-fA-F]{32}$").unwrap();
    static ref SHA_SHA3_384: Regex = Regex::new(r"^[0-9a-fA-F]{48}$").unwrap();
    static ref SHA_SHA3_512: Regex = Regex::new(r"^[0-9a-fA-F]{64}$").unwrap();
}

fn is_valid_based64(pat: &str) -> bool {
    BASE64.is_match(pat)
}

fn is_valid_date(pat: &str) -> bool {
    DATE.is_match(pat)
}

fn is_valid_dttz(pat: &str) -> bool {
    DateTime::parse_from_rfc3339(pat).is_ok()
}

fn is_valid_email(pat: &str) -> bool {
    EMAIL.is_match(pat)
}

fn is_valid_hash(pat: &str) -> bool {
    SHA_SHA3_224.is_match(pat)
        || SHA_SHA3_256.is_match(pat)
        || SHA_SHA3_384.is_match(pat)
        || SHA_SHA3_512.is_match(pat)
}

fn is_valid_token(pat: &str) -> bool {
    TOKEN.is_match(pat)
}

fn is_valid_uri(pat: &str) -> bool {
    URI::try_from(pat).is_ok()
}

fn is_valid_uri_ref(pat: &str) -> bool {
    URIReference::try_from(pat).is_ok()
}

fn is_valid_uuid(pat: &str) -> bool {
    let res = uuid::Uuid::try_parse(pat);
    if let Ok(res) = res {
        if let Some(version) = res.clone().get_version() {
            version == uuid::Version::Random || version == uuid::Version::Sha1
        } else {
            false
        }
    } else {
        false
    }
}

pub(crate) fn ser_base64<S: Serializer>(
    s: &str,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if is_valid_based64(s) {
        serializer.serialize_str(s)
    } else {
        Err(serde::ser::Error::custom("invalid base64 pattern"))
    }
}

pub(crate) fn deser_base64<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(v) => {
            if is_valid_based64(&v) {
                Ok(v)
            } else {
                Err(de::Error::custom("invalid base64 pattern".to_owned()))
            }
        }
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

pub(crate) fn ser_date<S: Serializer>(
    s: &str,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if is_valid_date(s) {
        serializer.serialize_str(s)
    } else {
        Err(serde::ser::Error::custom("invalid date pattern"))
    }
}

pub(crate) fn ser_date_opt<S: Serializer>(
    s: &Option<String>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if let Some(s) = s {
        if is_valid_date(s) {
            serializer.serialize_str(s)
        } else {
            Err(serde::ser::Error::custom("invalid date pattern"))
        }
    } else {
        serializer.serialize_none()
    }
}

pub(crate) fn deser_date<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(v) => {
            if is_valid_date(&v) {
                Ok(v)
            } else {
                Err(de::Error::custom("invalid date pattern".to_owned()))
            }
        }
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

pub(crate) fn deser_date_opt<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match Option::<String>::deserialize(deserializer) {
        Ok(Some(v)) => {
            if is_valid_date(&v) {
                Ok(Some(v))
            } else {
                Err(de::Error::custom("invalid date pattern".to_owned()))
            }
        }
        Ok(None) => Ok(None),
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

pub(crate) fn ser_dttz<S: Serializer>(
    s: &str,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if is_valid_dttz(s) {
        serializer.serialize_str(s)
    } else {
        Err(serde::ser::Error::custom("invalid datetime pattern"))
    }
}

pub(crate) fn ser_dttz_opt<S: Serializer>(
    s: &Option<String>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if let Some(s) = s {
        if is_valid_dttz(s) {
            serializer.serialize_str(s)
        } else {
            Err(serde::ser::Error::custom("invalid datetime pattern"))
        }
    } else {
        serializer.serialize_none()
    }
}

pub(crate) fn deser_dttz<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(v) => {
            if is_valid_dttz(&v) {
                Ok(v)
            } else {
                Err(de::Error::custom("invalid datetime pattern".to_owned()))
            }
        }
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

pub(crate) fn deser_dttz_opt<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match Option::<String>::deserialize(deserializer) {
        Ok(Some(v)) => {
            if is_valid_dttz(&v) {
                Ok(Some(v))
            } else {
                Err(de::Error::custom("invalid datetime pattern".to_owned()))
            }
        }
        Ok(None) => Ok(None),
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

pub(crate) fn ser_email_vec_opt<S: Serializer>(
    s: &Option<Vec<String>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if let Some(s) = s {
        if s.iter().all(|s| is_valid_email(s)) {
            let mut seq = serializer.serialize_seq(Some(s.len()))?;
            for elt in s {
                seq.serialize_element(elt)?;
            }
            seq.end()
        } else {
            Err(serde::ser::Error::custom("invalid email pattern"))
        }
    } else {
        serializer.serialize_none()
    }
}

pub(crate) fn deser_email_vec_opt<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<String>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match Option::<Vec<String>>::deserialize(deserializer) {
        Ok(Some(v)) => {
            if v.iter().all(|s| is_valid_email(s)) {
                Ok(Some(v))
            } else {
                Err(de::Error::custom("invalid email pattern".to_owned()))
            }
        }
        Ok(None) => Ok(None),
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

pub(crate) fn ser_hash<S: Serializer>(
    s: &str,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if is_valid_hash(s) {
        serializer.serialize_str(s)
    } else {
        Err(serde::ser::Error::custom("invalid rlink hash pattern"))
    }
}

pub(crate) fn deser_hash<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(v) => {
            if is_valid_hash(&v) {
                Ok(v)
            } else {
                Err(de::Error::custom("invalid rlink hash pattern".to_owned()))
            }
        }
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

pub(crate) fn ser_positive_int<S: Serializer>(
    s: &i64,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if *s >= 1 {
        serializer.serialize_i64(*s)
    } else {
        Err(serde::ser::Error::custom(
            "invalid positive integer pattern",
        ))
    }
}

pub(crate) fn deser_positive_int<'de, D>(
    deserializer: D,
) -> Result<i64, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match i64::deserialize(deserializer) {
        Ok(v) => {
            if v >= 1 {
                Ok(v)
            } else {
                Err(de::Error::custom(
                    "invalid positive integer pattern".to_owned(),
                ))
            }
        }
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

pub(crate) fn ser_non_neg_int_opt<S: Serializer>(
    s: &Option<i64>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if let Some(s) = s {
        if *s >= 0 {
            serializer.serialize_i64(*s)
        } else {
            Err(serde::ser::Error::custom(
                "invalid non negative integer pattern",
            ))
        }
    } else {
        serializer.serialize_none()
    }
}

pub(crate) fn deser_non_neg_int_opt<'de, D>(
    deserializer: D,
) -> Result<Option<i64>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match Option::<i64>::deserialize(deserializer) {
        Ok(Some(v)) => {
            if v >= 0 {
                Ok(Some(v))
            } else {
                Err(de::Error::custom(
                    "invalid non negative integer pattern".to_owned(),
                ))
            }
        }
        Ok(None) => Ok(None),
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

pub(crate) fn ser_token<S: Serializer>(
    s: &str,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if is_valid_token(s) {
        serializer.serialize_str(s)
    } else {
        Err(serde::ser::Error::custom("invalid token pattern"))
    }
}

pub(crate) fn ser_token_opt<S: Serializer>(
    s: &Option<String>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if let Some(s) = s {
        if is_valid_token(s) {
            serializer.serialize_str(s)
        } else {
            Err(serde::ser::Error::custom("invalid token pattern"))
        }
    } else {
        serializer.serialize_none()
    }
}

pub(crate) fn ser_token_vec_opt<S: Serializer>(
    s: &Option<Vec<String>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if let Some(s) = s {
        if s.iter().all(|s| is_valid_token(s)) {
            let mut seq = serializer.serialize_seq(Some(s.len()))?;
            for elt in s {
                seq.serialize_element(elt)?;
            }
            seq.end()
        } else {
            Err(serde::ser::Error::custom("invalid token pattern"))
        }
    } else {
        serializer.serialize_none()
    }
}

pub(crate) fn deser_token<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(v) => {
            if is_valid_token(&v) {
                Ok(v)
            } else {
                Err(de::Error::custom("invalid token pattern".to_owned()))
            }
        }
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

pub(crate) fn deser_token_opt<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match Option::<String>::deserialize(deserializer) {
        Ok(Some(v)) => {
            if is_valid_token(&v) {
                Ok(Some(v))
            } else {
                Err(de::Error::custom("invalid token pattern".to_owned()))
            }
        }
        Ok(None) => Ok(None),
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

pub(crate) fn deser_token_vec_opt<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<String>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match Option::<Vec<String>>::deserialize(deserializer) {
        Ok(Some(v)) => {
            if v.iter().all(|s| is_valid_token(s)) {
                Ok(Some(v))
            } else {
                Err(de::Error::custom("invalid token pattern".to_owned()))
            }
        }
        Ok(None) => Ok(None),
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

pub(crate) fn ser_uri<S: Serializer>(
    s: &str,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if is_valid_uri(s) {
        serializer.serialize_str(s)
    } else {
        Err(serde::ser::Error::custom("invalid uri pattern"))
    }
}

pub(crate) fn ser_uri_opt<S: Serializer>(
    s: &Option<String>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if let Some(s) = s {
        if is_valid_uri(s) {
            serializer.serialize_str(s)
        } else {
            Err(serde::ser::Error::custom("invalid uri pattern"))
        }
    } else {
        serializer.serialize_none()
    }
}

pub(crate) fn ser_uri_vec_opt<S: Serializer>(
    s: &Option<Vec<String>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if let Some(s) = s {
        if s.iter().all(|s| is_valid_uri(s)) {
            let mut seq = serializer.serialize_seq(Some(s.len()))?;
            for elt in s {
                seq.serialize_element(elt)?;
            }
            seq.end()
        } else {
            Err(serde::ser::Error::custom("invalid uri pattern"))
        }
    } else {
        serializer.serialize_none()
    }
}

pub(crate) fn deser_uri<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(v) => {
            if is_valid_uri(&v) {
                Ok(v)
            } else {
                Err(de::Error::custom("invalid uri pattern".to_owned()))
            }
        }
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

pub(crate) fn deser_uri_opt<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match Option::<String>::deserialize(deserializer) {
        Ok(Some(v)) => {
            if is_valid_uri(&v) {
                Ok(Some(v))
            } else {
                Err(de::Error::custom("invalid uri pattern".to_owned()))
            }
        }
        Ok(None) => Ok(None),
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

pub(crate) fn deser_uri_vec_opt<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<String>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match Option::<Vec<String>>::deserialize(deserializer) {
        Ok(Some(v)) => {
            if v.iter().all(|s| is_valid_uri(s)) {
                Ok(Some(v))
            } else {
                Err(de::Error::custom("invalid uri pattern".to_owned()))
            }
        }
        Ok(None) => Ok(None),
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

pub(crate) fn ser_uri_ref<S: Serializer>(
    s: &str,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if is_valid_uri_ref(s) {
        serializer.serialize_str(s)
    } else {
        Err(serde::ser::Error::custom("invalid uri reference pattern"))
    }
}

pub(crate) fn ser_uri_ref_opt<S: Serializer>(
    s: &Option<String>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if let Some(s) = s {
        if is_valid_uri_ref(s) {
            serializer.serialize_str(s)
        } else {
            Err(serde::ser::Error::custom("invalid uri reference pattern"))
        }
    } else {
        serializer.serialize_none()
    }
}

pub(crate) fn deser_uri_ref<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(v) => {
            if is_valid_uri(&v) || is_valid_uri_ref(&v) {
                Ok(v)
            } else {
                Err(de::Error::custom(
                    "invalid uri reference pattern".to_owned(),
                ))
            }
        }
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

pub(crate) fn deser_uri_ref_opt<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match Option::<String>::deserialize(deserializer) {
        Ok(Some(v)) => {
            if is_valid_uri(&v) || is_valid_uri_ref(&v) {
                Ok(Some(v))
            } else {
                Err(de::Error::custom(
                    "invalid uri reference pattern".to_owned(),
                ))
            }
        }
        Ok(None) => Ok(None),
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

pub(crate) fn ser_uuid<S: Serializer>(
    s: &str,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if is_valid_uuid(s) {
        serializer.serialize_str(s)
    } else {
        Err(serde::ser::Error::custom("invalid uuid pattern"))
    }
}

pub(crate) fn ser_uuid_vec<S: Serializer>(
    s: &Vec<String>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if s.iter().all(|s| is_valid_uuid(s)) {
        let mut seq = serializer.serialize_seq(Some(s.len()))?;
        for elt in s {
            seq.serialize_element(elt)?;
        }
        seq.end()
    } else {
        Err(serde::ser::Error::custom("invalid uuid pattern"))
    }
}

pub(crate) fn ser_uuid_opt<S: Serializer>(
    s: &Option<String>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if let Some(s) = s {
        if is_valid_uuid(s) {
            serializer.serialize_str(s)
        } else {
            Err(serde::ser::Error::custom("invalid uuid pattern"))
        }
    } else {
        serializer.serialize_none()
    }
}

pub(crate) fn ser_uuid_vec_opt<S: Serializer>(
    s: &Option<Vec<String>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    if let Some(s) = s {
        if s.iter().all(|s| is_valid_uuid(s)) {
            let mut seq = serializer.serialize_seq(Some(s.len()))?;
            for elt in s {
                seq.serialize_element(elt)?;
            }
            seq.end()
        } else {
            Err(serde::ser::Error::custom("invalid uuid pattern"))
        }
    } else {
        serializer.serialize_none()
    }
}

pub(crate) fn deser_uuid<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match String::deserialize(deserializer) {
        Ok(v) => {
            if is_valid_uuid(&v) {
                Ok(v)
            } else {
                Err(de::Error::custom("invalid uuid pattern".to_owned()))
            }
        }
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

pub(crate) fn deser_uuid_opt<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match Option::<String>::deserialize(deserializer) {
        Ok(Some(v)) => {
            if is_valid_uuid(&v) {
                Ok(Some(v))
            } else {
                Err(de::Error::custom("invalid uuid pattern".to_owned()))
            }
        }
        Ok(None) => Ok(None),
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

pub(crate) fn deser_uuid_vec<'de, D>(
    deserializer: D,
) -> Result<Vec<String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match Vec::<String>::deserialize(deserializer) {
        Ok(v) => {
            if v.iter().all(|s| is_valid_uuid(s)) {
                Ok(v)
            } else {
                Err(de::Error::custom("invalid uuid pattern".to_owned()))
            }
        }
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

pub(crate) fn deser_uuid_vec_opt<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<String>>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    match Option::<Vec<String>>::deserialize(deserializer) {
        Ok(Some(v)) => {
            if v.iter().all(|s| is_valid_uuid(s)) {
                Ok(Some(v))
            } else {
                Err(de::Error::custom("invalid uuid pattern".to_owned()))
            }
        }
        Ok(None) => Ok(None),
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

fn parse_markup(input: String) -> String {
    let parser = Parser::new_ext(&input, Options::all());

    let mut output = String::new();
    pulldown_cmark::html::push_html(&mut output, parser);

    output
}

pub(crate) fn deser_markup<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let mode = env::var("OSCAL_MARKUP_RENDER_MODE").unwrap_or_default();

    match String::deserialize(deserializer) {
        Ok(v) => {
            if mode.eq("ENABLED") {
                Ok(parse_markup(v))
            } else {
                Ok(v)
            }
        }
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}

pub(crate) fn deser_markup_opt<'de, D>(
    deserializer: D,
) -> Result<Option<String>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let mode = env::var("OSCAL_MARKUP_RENDER_MODE").unwrap_or_default();

    match Option::<String>::deserialize(deserializer) {
        Ok(Some(v)) => {
            if mode.eq("ENABLED") {
                Ok(Some(parse_markup(v)))
            } else {
                Ok(Some(v))
            }
        }
        Ok(None) => Ok(None),
        Err(e) => Err(de::Error::custom(e.to_string())),
    }
}
