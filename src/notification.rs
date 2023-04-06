/*
 * SPDX-FileCopyrightText: 2023 perillamint
 *
 * SPDX-License-Identifier: MPL-2.0
 */

use std::sync::Arc;

use btleplug::api::{bleuuid::uuid_from_u16, Peripheral as _};
use btleplug::api::{CharPropFlags, Characteristic, WriteType, ValueNotification};
use btleplug::platform::Peripheral;
use byteorder::{BigEndian, ByteOrder};
use chrono::{DateTime, Datelike, FixedOffset, Local, Timelike, Utc};
use futures_util::StreamExt;
use uuid::{uuid, Uuid};

use crate::gnss::GNSS_INFO_CHARACTERISTIC;
use crate::remote::REMOTE_INFO_CHARACTERISTIC;

use crate::types::LatLng;

const CCC_DESCRIPTOR_UUID: Uuid = uuid!("00002902-0000-1000-8000-00805f9b34fb");

pub enum NotificationSource {
    GNSS,
    CameraControl,
    Remote,
}

pub struct NotificationService {
    camera: Arc<Peripheral>,
}

impl NotificationService {
    pub fn new(camera: Arc<Peripheral>) -> Self {
        Self { camera }
    }

    pub async fn enable_notification() {
        // TODO: Implement me.
    }

    pub async fn disable_notification() {
        //
    }

    pub async fn subscribe(&self, source: NotificationSource) -> Result<(), anyhow::Error> {
        match source {
            NotificationSource::GNSS => {
                self.camera
                    .subscribe(&GNSS_INFO_CHARACTERISTIC).await?;
            }
            NotificationSource::CameraControl => {
                // Implement me.
            }
            NotificationSource::Remote => {
                println!("{:#?}", REMOTE_INFO_CHARACTERISTIC);
                self.camera
                    .subscribe(&REMOTE_INFO_CHARACTERISTIC).await?;
            }
        }

        Ok(())
    }

    pub async fn get_single_notification(&self) -> Result<ValueNotification, anyhow::Error> {
        let stream = self.camera.notifications().await?;
        let lastmsg = stream.take(1).next().await.unwrap();
        println!("{:#?}", lastmsg);

        Ok(lastmsg)
    }

}
