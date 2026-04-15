use std::collections::HashMap;

use diom_error::{Result, ResultExt};

use crate::cfg::{JwtConfig, JwtKey};

#[derive(serde::Deserialize)]
pub(crate) struct JwtClaims {
    pub(crate) role: String,
    #[serde(default)]
    pub(crate) context: HashMap<String, String>,
}

/// Pre-parsed JWT key and validation settings, built once at startup from [`JwtConfig`].
#[derive(Clone)]
pub(crate) struct JwtVerifier {
    key: jsonwebtoken::DecodingKey,
    validation: jsonwebtoken::Validation,
}

impl JwtVerifier {
    pub(crate) fn try_new(cfg: &JwtConfig) -> Result<Option<Self>> {
        use jsonwebtoken::{Algorithm, DecodingKey, Validation};

        let Some(cfg_key) = &cfg.key else {
            return Ok(None);
        };

        let (algorithm, key) = match cfg_key {
            JwtKey::Hs256 { secret } => (
                Algorithm::HS256,
                DecodingKey::from_secret(secret.as_bytes()),
            ),
            JwtKey::Hs384 { secret } => (
                Algorithm::HS384,
                DecodingKey::from_secret(secret.as_bytes()),
            ),
            JwtKey::Hs512 { secret } => (
                Algorithm::HS512,
                DecodingKey::from_secret(secret.as_bytes()),
            ),
            JwtKey::Rs256 { public_key_pem } => (
                Algorithm::RS256,
                DecodingKey::from_rsa_pem(public_key_pem.as_bytes()).or_internal_error()?,
            ),
            JwtKey::Rs384 { public_key_pem } => (
                Algorithm::RS384,
                DecodingKey::from_rsa_pem(public_key_pem.as_bytes()).or_internal_error()?,
            ),
            JwtKey::Rs512 { public_key_pem } => (
                Algorithm::RS512,
                DecodingKey::from_rsa_pem(public_key_pem.as_bytes()).or_internal_error()?,
            ),
            JwtKey::Es256 { public_key_pem } => (
                Algorithm::ES256,
                DecodingKey::from_ec_pem(public_key_pem.as_bytes()).or_internal_error()?,
            ),
            JwtKey::Es384 { public_key_pem } => (
                Algorithm::ES384,
                DecodingKey::from_ec_pem(public_key_pem.as_bytes()).or_internal_error()?,
            ),
            JwtKey::Ps256 { public_key_pem } => (
                Algorithm::PS256,
                DecodingKey::from_rsa_pem(public_key_pem.as_bytes()).or_internal_error()?,
            ),
            JwtKey::Ps384 { public_key_pem } => (
                Algorithm::PS384,
                DecodingKey::from_rsa_pem(public_key_pem.as_bytes()).or_internal_error()?,
            ),
            JwtKey::Ps512 { public_key_pem } => (
                Algorithm::PS512,
                DecodingKey::from_rsa_pem(public_key_pem.as_bytes()).or_internal_error()?,
            ),
        };

        let mut validation = Validation::new(algorithm);
        // Validate `nbf` (not-before) when present in the token.
        validation.validate_nbf = true;
        // Only validate `aud` when expected audiences are configured; otherwise
        // disable it so tokens with an `aud` claim are still accepted.
        match &cfg.audience {
            Some(aud) => {
                validation.set_audience(aud);
                // Require the claim to be present, not just valid when present.
                validation.required_spec_claims.insert("aud".to_string());
            }
            None => validation.validate_aud = false,
        }
        if let Some(iss) = &cfg.issuer {
            validation.set_issuer(iss);
            // Require the claim to be present, not just valid when present.
            validation.required_spec_claims.insert("iss".to_string());
        }

        Ok(Some(Self { key, validation }))
    }

    /// Returns `true` if `token` has the three-part `header.payload.signature` structure of a JWT.
    pub(crate) fn looks_like_jwt(token: &str) -> bool {
        token.bytes().filter(|&b| b == b'.').count() == 2
    }

    pub(crate) fn verify(&self, token: &str) -> Result<JwtClaims> {
        use diom_error::Error;
        jsonwebtoken::decode::<JwtClaims>(token, &self.key, &self.validation)
            .map(|t| t.claims)
            .map_err(|_| Error::authentication("invalid_token", "Invalid JWT token."))
    }
}
