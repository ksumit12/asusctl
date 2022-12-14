use async_trait::async_trait;
use serde_derive::{Deserialize, Serialize};
use zbus::dbus_interface;
use zbus::Connection;
use zvariant::Type;

use crate::{
    ctrl_anime::CtrlAnime, ctrl_aura::controller::CtrlKbdLed, ctrl_platform::CtrlPlatform,
    ctrl_power::CtrlPower, ctrl_profiles::controller::CtrlPlatformProfile, GetSupported,
};

use rog_platform::supported::*;

#[derive(Serialize, Deserialize, Type)]
pub struct SupportedFunctions {
    pub anime_ctrl: AnimeSupportedFunctions,
    pub charge_ctrl: ChargeSupportedFunctions,
    pub platform_profile: PlatformProfileFunctions,
    pub keyboard_led: LedSupportedFunctions,
    pub rog_bios_ctrl: RogBiosSupportedFunctions,
}

#[dbus_interface(name = "org.asuslinux.Daemon")]
impl SupportedFunctions {
    fn supported_functions(&self) -> &SupportedFunctions {
        self
    }
}

#[async_trait]
impl crate::ZbusRun for SupportedFunctions {
    async fn add_to_server(self, server: &mut Connection) {
        Self::add_to_server_helper(self, "/org/asuslinux/Supported", server).await;
    }
}

impl GetSupported for SupportedFunctions {
    type A = SupportedFunctions;

    fn get_supported() -> Self::A {
        SupportedFunctions {
            anime_ctrl: CtrlAnime::get_supported(),
            keyboard_led: CtrlKbdLed::get_supported(),
            charge_ctrl: CtrlPower::get_supported(),
            platform_profile: CtrlPlatformProfile::get_supported(),
            rog_bios_ctrl: CtrlPlatform::get_supported(),
        }
    }
}
