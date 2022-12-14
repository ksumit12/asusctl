//! This crate functions as a wrapper of all the relevant ASUS functionality
//! on ROG, Strix, and TUF laptops.

pub mod error;
pub mod hid_raw;
pub mod keyboard_led;
pub(crate) mod macros;
pub mod platform;
pub mod power;
pub mod supported;
pub mod usb_raw;

use std::path::Path;

use error::{PlatformError, Result};
use udev::Device;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub(crate) fn to_device(sys_path: &Path) -> Result<Device> {
    Device::from_syspath(sys_path)
        .map_err(|e| PlatformError::Udev("Couldn't transform syspath to device".to_string(), e))
}

pub fn has_attr(device: &Device, attr_name: &str) -> bool {
    for attr in device.attributes() {
        if attr.name() == attr_name {
            return true;
        }
    }
    false
}

pub fn read_attr_bool(device: &Device, attr_name: &str) -> Result<bool> {
    if let Some(value) = device.attribute_value(attr_name) {
        let tmp = value.to_string_lossy();
        if tmp.trim() == "0" {
            return Ok(false);
        }
        return Ok(true);
    }
    Err(PlatformError::AttrNotFound(attr_name.to_string()))
}

pub fn write_attr_bool(device: &mut Device, attr: &str, value: bool) -> Result<()> {
    device
        .set_attribute_value(attr, &(value as u8).to_string())
        .map_err(|e| PlatformError::IoPath(attr.into(), e))
}

pub fn read_attr_u8(device: &Device, attr_name: &str) -> Result<u8> {
    if let Some(value) = device.attribute_value(attr_name) {
        let tmp = value.to_string_lossy();
        return tmp.parse::<u8>().map_err(|_| PlatformError::ParseNum);
    }
    Err(PlatformError::AttrNotFound(attr_name.to_string()))
}

pub fn write_attr_u8(device: &mut Device, attr: &str, value: u8) -> Result<()> {
    device
        .set_attribute_value(attr, &(value).to_string())
        .map_err(|e| PlatformError::IoPath(attr.into(), e))
}

pub fn read_attr_u8_array(device: &Device, attr_name: &str) -> Result<Vec<u8>> {
    if let Some(value) = device.attribute_value(attr_name) {
        let tmp = value.to_string_lossy();
        let tmp = tmp
            .split(' ')
            .map(|v| v.parse::<u8>().unwrap_or(0))
            .collect();
        return Ok(tmp);
    }
    Err(PlatformError::AttrNotFound(attr_name.to_string()))
}

pub fn write_attr_u8_array(device: &mut Device, attr: &str, values: &[u8]) -> Result<()> {
    let tmp: String = values.iter().map(|v| format!("{} ", v)).collect();
    let tmp = tmp.trim();
    device
        .set_attribute_value(attr, &tmp)
        .map_err(|e| PlatformError::IoPath(attr.into(), e))
}

#[cfg(test)]
mod tests {
    #[test]
    fn check() {
        let data = [1, 2, 3, 4, 5];

        let tmp: String = data.iter().map(|v| format!("{} ", v)).collect();
        let tmp = tmp.trim();
        assert_eq!(tmp, "1 2 3 4 5");

        let tmp: Vec<u8> = tmp
            .split(' ')
            .map(|v| v.parse::<u8>().unwrap_or(0))
            .collect();
        assert_eq!(tmp, &[1, 2, 3, 4, 5]);
    }
}

// pub fn find_led_node(id_product: &str) -> Result<Device, PlatformError> {
//     let mut enumerator = udev::Enumerator::new().map_err(|err| {
//         warn!("{}", err);
//         PlatformError::Udev("enumerator failed".into(), err)
//     })?;
//     enumerator.match_subsystem("hidraw").map_err(|err| {
//         warn!("{}", err);
//         PlatformError::Udev("match_subsystem failed".into(), err)
//     })?;

//     for device in enumerator.scan_devices().map_err(|err| {
//         warn!("{}", err);
//         PlatformError::Udev("scan_devices failed".into(), err)
//     })? {
//         if let Some(parent) = device
//             .parent_with_subsystem_devtype("usb", "usb_device")
//             .map_err(|err| {
//                 warn!("{}", err);
//                 PlatformError::Udev("parent_with_subsystem_devtype failed".into(), err)
//             })?
//         {
//             if parent
//                 .attribute_value("idProduct")
//                 .ok_or_else(|| PlatformError::NotFound("LED idProduct".into()))?
//                 == id_product
//             {
//                 if let Some(dev_node) = device.devnode() {
//                     info!("Using device at: {:?} for LED control", dev_node);
//                     return Ok(device);
//                 }
//             }
//         }
//     }
//     Err(PlatformError::MissingFunction(
//         "ASUS LED device node not found".into(),
//     ))
// }
