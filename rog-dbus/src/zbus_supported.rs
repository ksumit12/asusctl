//! # DBus interface proxy for: `org.asuslinux.Daemon`
//!
//! This code was generated by `zbus-xmlgen` `1.0.0` from DBus introspection data.
//! Source: `Interface '/org/asuslinux/Supported' from service 'org.asuslinux.Daemon' on system bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://zeenix.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This DBus object implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::PeerProxy`]
//! * [`zbus::fdo::PropertiesProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.

use rog_types::supported::SupportedFunctions;
use zbus::{dbus_proxy, Connection, Result};

#[dbus_proxy(
    interface = "org.asuslinux.Daemon",
    default_path = "/org/asuslinux/Supported"
)]
trait Daemon {
    /// SupportedFunctions method
    fn supported_functions(&self) -> zbus::Result<SupportedFunctions>;
}

pub struct SupportProxy<'a>(DaemonProxy<'a>);

impl<'a> SupportProxy<'a> {
    #[inline]
    pub fn new(conn: &Connection) -> Result<Self> {
        Ok(SupportProxy(DaemonProxy::new(&conn)?))
    }

    pub fn proxy(&self) -> &DaemonProxy<'a> {
        &self.0
    }

    #[inline]
    pub fn get_supported_functions(&self) -> Result<SupportedFunctions> {
        self.0.supported_functions()
    }
}
