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
use location::LocationService;

#[macro_use]
extern crate lazy_static;

mod camera_control;
mod location;
pub mod types;

pub struct Camera {
    camera: Arc<Peripheral>,
    location_svc: LocationService,
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
                if let Some(sonydata) = properties.manufacturer_data.get(&0x2d01) {
                    match (
                        LittleEndian::read_u16(&sonydata[0..2]),
                        sonydata[2],
                        String::from_utf8(sonydata[3..5].to_vec())?,
                    ) {
                        (0x012D, _protocol_version, _model) => {
                            cameras.push(p);
                        }
                        (_, _, _) => {} // Not a sony camera. skip.
                    }
                }
            }
        }

        Ok(cameras)
    }

    pub async fn new() -> Result<Vec<Self>, anyhow::Error> {
        Ok(Self::search()
            .await?
            .into_iter()
            .map(|p| {
                let camera = Arc::new(p);
                let location_svc = LocationService::new(camera.clone());
                let camera_control_svc = CameraControlService::new(camera.clone());

                Self {
                    camera,
                    location_svc,
                    camera_control_svc,
                }
            })
            .collect())
    }

    pub async fn get_name(&self) -> Result<String, anyhow::Error> {
        self.camera
            .properties()
            .await?
            .ok_or_else(|| anyhow::anyhow!("Properties not found!"))?
            .local_name
            .ok_or_else(|| anyhow::anyhow!("name not found"))
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
