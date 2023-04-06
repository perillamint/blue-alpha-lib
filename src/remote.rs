/*
 * SPDX-FileCopyrightText: 2023 perillamint
 *
 * SPDX-License-Identifier: MPL-2.0
 */

use std::sync::Arc;

use btleplug::api::{bleuuid::uuid_from_u16, Peripheral as _};
use btleplug::api::{CharPropFlags, Characteristic};
use btleplug::platform::Peripheral;
use byteorder::{BigEndian, ByteOrder};
use uuid::{uuid, Uuid};

const REMOTE_SERVICE_UUID: Uuid = uuid!("8000ff00-ff00-ffff-ffff-ffffffffffff");
pub(crate) const REMOTE_INFO_CHARACTERISTIC: Characteristic = Characteristic {
    uuid: uuid_from_u16(0xFF02),
    service_uuid: REMOTE_SERVICE_UUID,
    properties: CharPropFlags::NOTIFY,
};

const REMOTE_NOTIFY_CHARACTERISTIC: Characteristic = Characteristic {
    uuid: uuid_from_u16(0xFF01),
    service_uuid: REMOTE_SERVICE_UUID,
    properties: CharPropFlags::WRITE,
};

pub struct RemoteService {
    camera: Arc<Peripheral>,
}

impl RemoteService {
    pub fn new(camera: Arc<Peripheral>) -> Self {
        Self { camera }
    }
}
