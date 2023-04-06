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

const CAMERA_CONTROL_SERVICE_UUID: Uuid = uuid!("8000cc00-cc00-ffff-ffff-ffffffffffff");
lazy_static! {
    static ref CAMERA_CONTROL_NOTIFY_CHARACTERISTIC: Characteristic = Characteristic {
        uuid: uuid_from_u16(0xCC01),
        service_uuid: CAMERA_CONTROL_SERVICE_UUID,
        properties: CharPropFlags::NOTIFY,
    };
    static ref CAMERA_CONTROL_CHARACTERISTIC: Characteristic = Characteristic {
        uuid: uuid_from_u16(0xCC02),
        service_uuid: CAMERA_CONTROL_SERVICE_UUID,
        properties: CharPropFlags::WRITE,
    };
    static ref WIFI_STATUS_NOTIFY_CHARACTERISTIC: Characteristic = Characteristic {
        uuid: uuid_from_u16(0xCC05),
        service_uuid: CAMERA_CONTROL_SERVICE_UUID,
        properties: (CharPropFlags::READ | CharPropFlags::NOTIFY),
    };
    static ref WIFI_SSID_CHARACTERISTIC: Characteristic = Characteristic {
        uuid: uuid_from_u16(0xCC06),
        service_uuid: CAMERA_CONTROL_SERVICE_UUID,
        properties: CharPropFlags::READ,
    };
    static ref WIFI_PASSWORD_CHARACTERISTIC: Characteristic = Characteristic {
        uuid: uuid_from_u16(0xCC07),
        service_uuid: CAMERA_CONTROL_SERVICE_UUID,
        properties: CharPropFlags::READ
    };
    static ref FTP_STATUS_CHARACTERISTIC: Characteristic = Characteristic {
        uuid: uuid_from_u16(0xCC21),
        service_uuid: CAMERA_CONTROL_SERVICE_UUID,
        properties: CharPropFlags::READ | CharPropFlags::NOTIFY,
    };
    // TODO: CC22-CC2C
    static ref FTP_SERVER_NAME_CHARACTERISTIC: Characteristic = Characteristic {
        uuid: uuid_from_u16(0xCC40),
        service_uuid: CAMERA_CONTROL_SERVICE_UUID,
        properties: CharPropFlags::READ,
    };
}

pub(crate) struct CameraControlService {
    camera: Arc<Peripheral>,
}

impl CameraControlService {
    pub fn new(camera: Arc<Peripheral>) -> Self {
        Self { camera }
    }
}
