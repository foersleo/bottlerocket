//! The `v2` module contains the second version of the network configuration and implements the
//! appropriate traits.

use super::{error, Interfaces, Result, Validate};
use crate::interface_name::InterfaceName;
use crate::net_config::devices::interface::NetInterfaceV2;
use crate::wicked::{
    wicked_from, WickedDhcp4, WickedDhcp6, WickedInterface, WickedRoutes, WickedStaticAddress,
};
use indexmap::IndexMap;
use serde::Deserialize;
use snafu::ensure;

#[derive(Debug, Deserialize)]
pub(crate) struct NetConfigV2 {
    #[serde(flatten)]
    pub(crate) interfaces: IndexMap<InterfaceName, NetInterfaceV2>,
}

impl Interfaces for NetConfigV2 {
    fn primary_interface(&self) -> Option<String> {
        self.interfaces
            .iter()
            .find(|(_, v)| v.primary == Some(true))
            .or_else(|| self.interfaces.first())
            .map(|(n, _)| n.to_string())
    }

    fn has_interfaces(&self) -> bool {
        !self.interfaces.is_empty()
    }

    fn as_wicked_interfaces(&self) -> Vec<WickedInterface> {
        let mut wicked_interfaces = Vec::with_capacity(self.interfaces.len());
        for (name, config) in &self.interfaces {
            let interface = wicked_from!(name, config);

            wicked_interfaces.push(interface);
        }

        wicked_interfaces
    }
}

impl Validate for NetConfigV2 {
    fn validate(&self) -> Result<()> {
        for (_name, config) in &self.interfaces {
            config.validate()?;
        }

        let primary_count = self
            .interfaces
            .values()
            .filter(|v| v.primary == Some(true))
            .count();
        ensure!(
            primary_count <= 1,
            error::InvalidNetConfigSnafu {
                reason: "multiple primary interfaces defined, expected 1"
            }
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::net_config::test_macros::{basic_tests, dhcp_tests, static_address_tests};

    basic_tests!(2);
    dhcp_tests!(2);
    static_address_tests!(2);
}
