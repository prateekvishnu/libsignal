//
// Copyright 2020 Signal Messenger, LLC.
// SPDX-License-Identifier: AGPL-3.0-only
//

use crate::common::simple_types::*;
use crate::crypto;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct ProfileKeyCredentialV3 {
    pub(crate) reserved: ReservedBytes,
    pub(crate) credential: crypto::credentials::ProfileKeyCredentialV3,
    pub(crate) uid_bytes: UidBytes,
    pub(crate) profile_key_bytes: ProfileKeyBytes,
}
