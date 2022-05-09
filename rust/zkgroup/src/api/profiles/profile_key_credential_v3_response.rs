//
// Copyright 2020 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use crate::common::simple_types::*;
use crate::crypto;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProfileKeyCredentialV3Response {
    pub(crate) reserved: ReservedBytes,
    pub(crate) blinded_credential: crypto::credentials::BlindedProfileKeyCredentialV3,
    pub(crate) proof: crypto::proofs::ProfileKeyCredentialV3IssuanceProof,
}
