//! # DBus interface proxy for: `org.asuslinux.Daemon`
//!
//! This code was generated by `zbus-xmlgen` `1.0.0` from DBus introspection data.
//! Source: `Interface '/org/asuslinux/Anime' from service 'org.asuslinux.Daemon' on session bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This DBus object implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::PeerProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PropertiesProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.
#![allow(clippy::too_many_arguments)]

use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.asuslinux.Daemon")]
trait Daemon {
    /// InsertAsusGif method
    fn insert_asus_gif(
        &self,
        index: u32,
        file: &str,
        time: u32,
        count: u32,
        brightness: f64,
    ) -> zbus::Result<String>;

    /// InsertImage method
    fn insert_image(
        &self,
        index: u32,
        file: &str,
        scale: f64,
        angle: f64,
        xy: &(f64, f64),
        brightness: f64,
    ) -> zbus::Result<String>;

    /// InsertImageGif method
    fn insert_image_gif(
        &self,
        index: u32,
        file: &str,
        scale: f64,
        angle: f64,
        xy: &(f64, f64),
        time: u32,
        count: u32,
        brightness: f64,
    ) -> zbus::Result<String>;

    /// InsertPause method
    fn insert_pause(&self, index: u32, millis: u64) -> zbus::Result<String>;

    /// RemoveItem method
    fn remove_item(&self, index: u32) -> zbus::Result<String>;

    /// SetState method
    fn set_state(&self, on: bool) -> zbus::Result<()>;
}
