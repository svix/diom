// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::fmt::{Debug, Formatter};

use axum::extract::FromRequestParts;
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use http::request::Parts;
use jwt_simple::prelude::*;
use serde::Deserializer;

use crate::{
    error::{HttpError, Result},
    AppState,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct CustomClaim {}

pub struct Permissions {}

pub const INVALID_TOKEN_ERR: &str = "Invalid token";
pub const JWT_SECRET_ERR : &str = "Authentication failed. JWT signing secrets can not be used as tokens, please refer to https://github.com/svix/svix-webhooks#authentication for more information.";

pub async fn permissions_from_bearer(parts: &mut Parts, state: &AppState) -> Result<Permissions> {
    let TypedHeader(Authorization(bearer)) =
        TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
            .await
            .map_err(|_| HttpError::unauthorized(None, Some("Invalid token".to_string())))?;

    let claims = parse_bearer(&state.cfg.jwt_signing_config, &bearer)
        .ok_or_else(|| HttpError::unauthorized(None, Some(INVALID_TOKEN_ERR.to_string())))?;
    let perms = permissions_from_jwt(claims)?;

    Ok(perms)
}

pub fn parse_bearer(
    signing_config: &JwtSigningConfig,
    bearer: &Bearer,
) -> Option<JWTClaims<CustomClaim>> {
    signing_config.verify_token(bearer.token(), None).ok()
}

pub fn permissions_from_jwt(_claims: JWTClaims<CustomClaim>) -> Result<Permissions> {
    Ok(Permissions {})
}

#[allow(dead_code)]
const JWT_ISSUER: &str = env!("CARGO_PKG_NAME");

/// A wrapper for the available JWT signing algorithms exposed by `jwt-simple`
#[derive(Deserialize)]
#[serde(tag = "jwt_algorithm", content = "jwt_secret")]
pub enum JwtSigningConfig {
    #[serde(deserialize_with = "deserialize_hs256")]
    HS256(HS256Key),
    #[serde(deserialize_with = "deserialize_hs384")]
    HS384(HS384Key),
    #[serde(deserialize_with = "deserialize_hs512")]
    HS512(HS512Key),
    #[serde(deserialize_with = "deserialize_rs256")]
    RS256(RS256),
    #[serde(deserialize_with = "deserialize_rs384")]
    RS384(RS384),
    #[serde(deserialize_with = "deserialize_rs512")]
    RS512(RS512),
    #[serde(deserialize_with = "deserialize_eddsa")]
    EdDSA(EdDSA),
}

pub enum RS256 {
    Public(RS256PublicKey),
    Pair(Box<RS256KeyPair>),
}

pub enum RS384 {
    Public(RS384PublicKey),
    Pair(Box<RS384KeyPair>),
}

pub enum RS512 {
    Public(RS512PublicKey),
    Pair(Box<RS512KeyPair>),
}

pub enum EdDSA {
    Public(Ed25519PublicKey),
    Pair(Box<Ed25519KeyPair>),
}

impl JwtSigningConfig {
    pub fn generate(&self, claims: JWTClaims<CustomClaim>) -> Result<String, jwt_simple::Error> {
        match self {
            JwtSigningConfig::HS256(key) => key.authenticate(claims),
            JwtSigningConfig::HS384(key) => key.authenticate(claims),
            JwtSigningConfig::HS512(key) => key.authenticate(claims),
            JwtSigningConfig::RS256(kind) => match kind {
                RS256::Public(_) => Err(jwt_simple::Error::msg("cannot sign JWT with public key")),
                RS256::Pair(key) => key.sign(claims),
            },
            JwtSigningConfig::RS384(kind) => match kind {
                RS384::Public(_) => Err(jwt_simple::Error::msg("cannot sign JWT with public key")),
                RS384::Pair(key) => key.sign(claims),
            },
            JwtSigningConfig::RS512(kind) => match kind {
                RS512::Public(_) => Err(jwt_simple::Error::msg("cannot sign JWT with public key")),
                RS512::Pair(key) => key.sign(claims),
            },
            JwtSigningConfig::EdDSA(kind) => match kind {
                EdDSA::Public(_) => Err(jwt_simple::Error::msg("cannot sign JWT with public key")),
                EdDSA::Pair(key) => key.sign(claims),
            },
        }
    }

    pub fn verify_token(
        &self,
        token: &str,
        options: Option<VerificationOptions>,
    ) -> Result<JWTClaims<CustomClaim>, jwt_simple::Error> {
        match self {
            JwtSigningConfig::HS256(key) => key.verify_token(token, options),
            JwtSigningConfig::HS384(key) => key.verify_token(token, options),
            JwtSigningConfig::HS512(key) => key.verify_token(token, options),
            JwtSigningConfig::RS256(kind) => match kind {
                RS256::Public(key) => key.verify_token(token, options),
                RS256::Pair(pair) => pair.public_key().verify_token(token, options),
            },
            JwtSigningConfig::RS384(kind) => match kind {
                RS384::Public(key) => key.verify_token(token, options),
                RS384::Pair(pair) => pair.public_key().verify_token(token, options),
            },
            JwtSigningConfig::RS512(kind) => match kind {
                RS512::Public(key) => key.verify_token(token, options),
                RS512::Pair(pair) => pair.public_key().verify_token(token, options),
            },
            JwtSigningConfig::EdDSA(kind) => match kind {
                EdDSA::Public(key) => key.verify_token(token, options),
                EdDSA::Pair(pair) => pair.public_key().verify_token(token, options),
            },
        }
    }
}

impl Debug for JwtSigningConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                JwtSigningConfig::HS256(_) => "HS256",
                JwtSigningConfig::HS384(_) => "HS384",
                JwtSigningConfig::HS512(_) => "HS512",
                JwtSigningConfig::RS256(_) => "RS256",
                JwtSigningConfig::RS384(_) => "RS384",
                JwtSigningConfig::RS512(_) => "RS512",
                JwtSigningConfig::EdDSA(_) => "EdDSA",
            }
        )
    }
}

fn deserialize_hs256<'de, D>(deserializer: D) -> Result<HS256Key, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(HS256Key::from_bytes(
        String::deserialize(deserializer)?.as_bytes(),
    ))
}

fn deserialize_hs384<'de, D>(deserializer: D) -> Result<HS384Key, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(HS384Key::from_bytes(
        String::deserialize(deserializer)?.as_bytes(),
    ))
}

fn deserialize_hs512<'de, D>(deserializer: D) -> Result<HS512Key, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(HS512Key::from_bytes(
        String::deserialize(deserializer)?.as_bytes(),
    ))
}

fn deserialize_rs256<'de, D>(deserializer: D) -> Result<RS256, D::Error>
where
    D: Deserializer<'de>,
{
    let key = String::deserialize(deserializer)?;
    if let Ok(pair) = RS256KeyPair::from_pem(&key) {
        Ok(RS256::Pair(Box::new(pair)))
    } else if let Ok(public) = RS256PublicKey::from_pem(&key) {
        Ok(RS256::Public(public))
    } else {
        Err(serde::de::Error::custom("could not deserialize key"))
    }
}

fn deserialize_rs384<'de, D>(deserializer: D) -> Result<RS384, D::Error>
where
    D: Deserializer<'de>,
{
    let key = String::deserialize(deserializer)?;
    if let Ok(pair) = RS384KeyPair::from_pem(&key) {
        Ok(RS384::Pair(Box::new(pair)))
    } else if let Ok(public) = RS384PublicKey::from_pem(&key) {
        Ok(RS384::Public(public))
    } else {
        Err(serde::de::Error::custom("could not deserialize key"))
    }
}

fn deserialize_rs512<'de, D>(deserializer: D) -> Result<RS512, D::Error>
where
    D: Deserializer<'de>,
{
    let key = String::deserialize(deserializer)?;
    if let Ok(pair) = RS512KeyPair::from_pem(&key) {
        Ok(RS512::Pair(Box::new(pair)))
    } else if let Ok(public) = RS512PublicKey::from_pem(&key) {
        Ok(RS512::Public(public))
    } else {
        Err(serde::de::Error::custom("could not deserialize key"))
    }
}

fn deserialize_eddsa<'de, D>(deserializer: D) -> Result<EdDSA, D::Error>
where
    D: Deserializer<'de>,
{
    let key = String::deserialize(deserializer)?;
    if let Ok(pair) = Ed25519KeyPair::from_pem(&key) {
        Ok(EdDSA::Pair(Box::new(pair)))
    } else if let Ok(public) = Ed25519PublicKey::from_pem(&key) {
        Ok(EdDSA::Public(public))
    } else {
        Err(serde::de::Error::custom("could not deserialize key"))
    }
}
