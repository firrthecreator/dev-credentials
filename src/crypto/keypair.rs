use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use chrono::Utc;
use rivide::crypto::Sha3;
use rivide::dsa::{MlDsa65, MlDsa65KeyPair, MlDsa65PublicKey, MlDsa65SecretKey};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthorityKeypairDto {
    pub algorithm: String,
    pub fingerprint: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    #[serde(rename = "secretKey")]
    pub secret_key: String,
    pub created_at: String,
    pub issuer: String,
    pub authority: String,
}

pub struct AuthorityKeypair {
    pub public_key: MlDsa65PublicKey,
    pub secret_key: MlDsa65SecretKey,
    pub fingerprint: String,
    pub algorithm: String,
    pub created_at: String,
    pub issuer: String,
    pub authority: String,
}

impl AuthorityKeypair {
    pub fn generate() -> Result<Self, Box<dyn std::error::Error>> {
        let kp: MlDsa65KeyPair = MlDsa65::keypair()?;
        let pk_bytes = kp.public_key.as_bytes();
        let pk_hash = Sha3::sha3_256(pk_bytes);
        let hex_hash = hex::encode(pk_hash).to_uppercase();
        let fingerprint = format!(
            "PK-MLDSA65-{}-{}-{}-{}",
            &hex_hash[0..4],
            &hex_hash[4..8],
            &hex_hash[8..12],
            &hex_hash[12..16]
        );

        Ok(Self {
            public_key: kp.public_key,
            secret_key: kp.secret_key,
            fingerprint,
            algorithm: "NIST FIPS 204 ML-DSA-65".to_string(),
            created_at: Utc::now().to_rfc3339(),
            issuer: "dev : community".to_string(),
            authority: "Moh. Ananda Firmansyah Putra".to_string(),
        })
    }

    pub fn to_dto(&self) -> AuthorityKeypairDto {
        AuthorityKeypairDto {
            algorithm: self.algorithm.clone(),
            fingerprint: self.fingerprint.clone(),
            public_key: BASE64.encode(self.public_key.as_bytes()),
            secret_key: BASE64.encode(self.secret_key.as_bytes()),
            created_at: self.created_at.clone(),
            issuer: self.issuer.clone(),
            authority: self.authority.clone(),
        }
    }

    pub fn from_dto(dto: &AuthorityKeypairDto) -> Result<Self, Box<dyn std::error::Error>> {
        let pk_bytes = BASE64.decode(&dto.public_key)?;
        let sk_bytes = BASE64.decode(&dto.secret_key)?;

        if pk_bytes.len() != 1952 {
            return Err("Invalid public key length".into());
        }
        if sk_bytes.len() != 4032 {
            return Err("Invalid secret key length".into());
        }

        let mut pk_arr = [0u8; 1952];
        pk_arr.copy_from_slice(&pk_bytes);

        let mut sk_arr = [0u8; 4032];
        sk_arr.copy_from_slice(&sk_bytes);

        Ok(Self {
            public_key: MlDsa65PublicKey::from_bytes(pk_arr),
            secret_key: MlDsa65SecretKey::from_bytes(sk_arr),
            fingerprint: dto.fingerprint.clone(),
            algorithm: dto.algorithm.clone(),
            created_at: dto.created_at.clone(),
            issuer: dto.issuer.clone(),
            authority: dto.authority.clone(),
        })
    }
}
