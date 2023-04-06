/*
 * SPDX-FileCopyrightText: 2023 perillamint
 *
 * SPDX-License-Identifier: MPL-2.0
 */

use std::sync::Arc;

use btleplug::api::{
    bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::{Adapter, Manager, Peripheral};
use byteorder::{BigEndian, ByteOrder, LittleEndian};
use camera_control::CameraControlService;
use gnss::GnssService;
use notification::{NotificationService, NotificationSource};

#[macro_use]
extern crate lazy_static;

mod camera_control;
mod gnss;
mod notification;
pub mod types;

pub struct Camera {
    camera: Arc<Peripheral>,
    notification_svc: NotificationService,
    gnss_svc: GnssService,
    camera_control_svc: CameraControlService,
}

impl Camera {
    async fn search() -> Result<Vec<Peripheral>, anyhow::Error> {
        let manager = Manager::new().await.unwrap();

        // get the first bluetooth adapter
        let adapters = manager.adapters().await?;
        let central = adapters.into_iter().nth(0).unwrap();

        // std::async_iter is in nigtly only so...
        let mut cameras: Vec<Peripheral> = vec![];
        for p in central.peripherals().await? {
            if let Some(properties) = p.properties().await? {
                if let Some(sonydata) = properties.manufacturer_data.get(&0x012d) {
                    match (
                        LittleEndian::read_u16(&sonydata[0..2]),
                        sonydata[2],
                        String::from_utf8(sonydata[3..5].to_vec())?,
                    ) {
                        (0x0003, _protocol_version, _model) => {
                            p.discover_services().await?;
                            cameras.push(p);
                        }
                        (_, _, _) => {} // Not a sony camera. skip.
                    }
                }
            }
        }

        println!("{:?}", cameras);
        Ok(cameras)
    }

    pub async fn new() -> Result<Vec<Self>, anyhow::Error> {
        Ok(Self::search()
            .await?
            .into_iter()
            .map(|p| {
                let camera = Arc::new(p);
                let notification_svc = NotificationService::new(camera.clone());
                let gnss_svc = GnssService::new(camera.clone());
                let camera_control_svc = CameraControlService::new(camera.clone());

                Self {
                    camera,
                    notification_svc,
                    gnss_svc,
                    camera_control_svc,
                }
            })
            .collect())
    }

    pub async fn init(&self) -> Result<(), anyhow::Error> {
        // Subscribe to notifications
        self.notification_svc.subscribe(NotificationSource::GNSS).await?;

        Ok(())
    }

    pub async fn get_name(&self) -> Result<String, anyhow::Error> {
        self.camera
            .properties()
            .await?
            .ok_or_else(|| anyhow::anyhow!("Properties not found!"))?
            .local_name
            .ok_or_else(|| anyhow::anyhow!("name not found"))
    }

    pub fn get_gnss_service(&self) -> &GnssService {
        &self.gnss_svc
    }
}

#[cfg(test)]
mod tests {
    use crate::types::LatLng;

    use super::*;

    #[tokio::test]
    async fn search_and_send_latlng() {
        let cameras = Camera::new().await.unwrap();
        let camera = cameras.get(0).unwrap();
        let everest = LatLng {
            latitude: 27.988056,
            longitude: 86.925278,
        };

        camera.init().await.unwrap();
        camera.get_gnss_service().send_location(&everest).await.unwrap();
        //camera.get_gnss_service().wait_for_request().await.unwrap();

    }
}
