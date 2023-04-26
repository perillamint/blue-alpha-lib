/*
 * SPDX-FileCopyrightText: 2023 perillamint
 *
 * SPDX-License-Identifier: MPL-2.0
 */

pub struct LatLng {
    pub latitude: f64,
    pub longitude: f64,
}

pub enum RemoteCommand {
    FocusPress,
    FocusRelease,
    ShutterPress,
    ShutterRelease,
    AutofocusPress,
    AutofocusRelease,
    ZoomInPress,
    ZoomInRelease,
    ZoomOutPress,
    ZoomOutRelease,
    C1Press,
    C1Release,
    ToggleRecord,
    FocusInPress,
    FocusInRelease,
    FocusOutPress,
    FocusOutRelease,
}

impl From<RemoteCommand> for Vec<u8> {
    fn from(command: RemoteCommand) -> Self {
        let body: Vec<u8> = match command {
            RemoteCommand::FocusPress => vec![0x07],
            RemoteCommand::FocusRelease => vec![0x06],
            RemoteCommand::ShutterPress => vec![0x09],
            RemoteCommand::ShutterRelease => vec![0x08],
            RemoteCommand::AutofocusPress => vec![0x15],
            RemoteCommand::AutofocusRelease => vec![0x14],
            RemoteCommand::ZoomInPress => vec![0x6d, 0x20],
            RemoteCommand::ZoomInRelease => vec![0x6c, 0x00],
            RemoteCommand::ZoomOutPress => vec![0x6b, 0x20],
            RemoteCommand::ZoomOutRelease => vec![0x6a, 0x00],
            RemoteCommand::C1Press => vec![0x21],
            RemoteCommand::C1Release => vec![0x20],
            RemoteCommand::ToggleRecord => vec![0x0e],
            RemoteCommand::FocusInPress => vec![0x47, 0x20],
            RemoteCommand::FocusInRelease => vec![0x46, 0x00],
            RemoteCommand::FocusOutPress => vec![0x45, 0x20],
            RemoteCommand::FocusOutRelease => vec![0x44, 0x00],
        };

        let mut payload: Vec<u8> = vec![body.len() as u8];
        payload.extend(body);
        payload
    }
}
