/*
 * SPDX-FileCopyrightText: 2023 perillamint
 *
 * SPDX-License-Identifier: MPL-2.0
 */

use std::sync::Arc;

use btleplug::api::{bleuuid::uuid_from_u16, Peripheral as _};
use btleplug::api::{CharPropFlags, Characteristic, WriteType};
use btleplug::platform::Peripheral;
use byteorder::{BigEndian, ByteOrder};
use chrono::{DateTime, Datelike, FixedOffset, Local, Timelike, Utc};
use futures_util::StreamExt;
use uuid::{uuid, Uuid};

use crate::types::LatLng;

const GNSS_SERVICE_UUID: Uuid = uuid!("8000DD00-DD00-FFFF-FFFF-FFFFFFFFFFFF");
pub(crate) const GNSS_INFO_CHARACTERISTIC: Characteristic = Characteristic {
    uuid: uuid_from_u16(0xDD01),
    service_uuid: GNSS_SERVICE_UUID,
    properties: CharPropFlags::NOTIFY,
};
const GNSS_NOTIFY_CHARACTERISTIC: Characteristic = Characteristic {
    uuid: uuid_from_u16(0xDD11),
    service_uuid: GNSS_SERVICE_UUID,
    properties: CharPropFlags::WRITE,
};
const GNSS_FEATURE_CHARACTERISTIC: Characteristic = Characteristic {
    uuid: uuid_from_u16(0xDD21),
    service_uuid: GNSS_SERVICE_UUID,
    properties: CharPropFlags::READ,
};

fn gnss_payload(latlng: &LatLng, now: &DateTime<Local>, include_tz: bool) -> Vec<u8> {
    // Payload format reference: https://github.com/whc2001/ILCE7M3ExternalGps/blob/main/PROTOCOL_EN.md
    let mut data: Vec<u8> = match include_tz {
        true => vec![0; 95],
        false => vec![0; 93],
    };

    let len = data.len() - 2;
    BigEndian::write_u16(&mut data[0..2], len as u16);

    // Magic
    BigEndian::write_u24(&mut data[2..5], 0x0802fc);

    // Report type: With timezone offset and DST.
    data[5] = match include_tz {
        true => 0x03,
        false => 0x00,
    };

    // Another magic.
    data[6] = 0x00;
    BigEndian::write_u32(&mut data[7..11], 0x0010_1010);

    BigEndian::write_i32(
        &mut data[11..15],
        (latlng.latitude * (10000000 as f64)) as i32,
    );
    BigEndian::write_i32(
        &mut data[15..19],
        (latlng.longitude * (10000000 as f64)) as i32,
    );

    let now_payload: DateTime<FixedOffset> = match include_tz {
        true => Into::<DateTime<Utc>>::into(*now).into(),
        false => Into::<DateTime<FixedOffset>>::into(*now),
    };
    BigEndian::write_u16(&mut data[19..21], now_payload.year() as u16);
    data[21] = now_payload.month() as u8;
    data[22] = now_payload.day() as u8;
    data[23] = now_payload.hour() as u8;
    data[24] = now_payload.minute() as u8;
    data[25] = now_payload.second() as u8;

    // Zeros from 26 to 90

    if include_tz {
        BigEndian::write_u16(
            &mut data[91..93],
            (now.offset().local_minus_utc() / 60) as u16,
        );
        BigEndian::write_u16(&mut data[93..95], 0x0000); // DST offset in munute
    }

    data
}

pub struct GnssService {
    camera: Arc<Peripheral>,
}

impl GnssService {
    pub fn new(camera: Arc<Peripheral>) -> Self {
        Self { camera }
    }

    pub async fn send_location(&self, latlng: &LatLng) -> Result<(), anyhow::Error> {
        let now = Local::now();
        let payload = gnss_payload(latlng, &now, true);

        // TODO: Find out camera requires timezone offset.

        self.camera
            .write(
                &GNSS_NOTIFY_CHARACTERISTIC,
                &payload,
                WriteType::WithResponse,
            )
            .await?;

        Ok(())
    }
}
