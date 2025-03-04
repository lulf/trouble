use bt_hci::cmd::le::LeConnUpdate;
use bt_hci::cmd::link_control::Disconnect;
use bt_hci::cmd::status::ReadRssi;
use bt_hci::controller::{Controller, ControllerCmdAsync, ControllerCmdSync};
use bt_hci::param::{BdAddr, ConnHandle, DisconnectReason, LeConnRole};
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_time::Duration;

use crate::adapter::Adapter;
use crate::scan::ScanConfig;
use crate::AdapterError;

#[derive(Clone)]
pub struct Connection {
    handle: ConnHandle,
}

pub struct ConnectConfig<'d> {
    pub scan_config: ScanConfig<'d>,
    pub connect_params: ConnectParams,
}

pub struct ConnectParams {
    pub min_connection_interval: Duration,
    pub max_connection_interval: Duration,
    pub max_latency: u16,
    pub event_length: Duration,
    pub supervision_timeout: Duration,
}

impl Default for ConnectParams {
    fn default() -> Self {
        Self {
            min_connection_interval: Duration::from_millis(80),
            max_connection_interval: Duration::from_millis(80),
            max_latency: 0,
            event_length: Duration::from_secs(0),
            supervision_timeout: Duration::from_secs(8),
        }
    }
}

impl Connection {
    pub(crate) fn new(handle: ConnHandle) -> Self {
        Self { handle }
    }

    pub fn handle(&self) -> ConnHandle {
        self.handle
    }

    pub fn disconnect<
        M: RawMutex,
        T: Controller + ControllerCmdSync<Disconnect>,
        const CONNS: usize,
        const CHANNELS: usize,
        const L2CAP_MTU: usize,
        const L2CAP_TXQ: usize,
        const L2CAP_RXQ: usize,
    >(
        &mut self,
        adapter: &Adapter<'_, M, T, CONNS, CHANNELS, L2CAP_MTU, L2CAP_TXQ, L2CAP_RXQ>,
    ) -> Result<(), AdapterError<T::Error>> {
        adapter.try_command(Disconnect::new(self.handle, DisconnectReason::RemoteUserTerminatedConn))?;
        Ok(())
    }

    pub fn role<
        M: RawMutex,
        T: Controller,
        const CONNS: usize,
        const CHANNELS: usize,
        const L2CAP_MTU: usize,
        const L2CAP_TXQ: usize,
        const L2CAP_RXQ: usize,
    >(
        &self,
        adapter: &Adapter<'_, M, T, CONNS, CHANNELS, L2CAP_MTU, L2CAP_TXQ, L2CAP_RXQ>,
    ) -> Result<LeConnRole, AdapterError<T::Error>> {
        let role = adapter.connections.role(self.handle)?;
        Ok(role)
    }

    pub fn peer_address<
        M: RawMutex,
        T: Controller,
        const CONNS: usize,
        const CHANNELS: usize,
        const L2CAP_MTU: usize,
        const L2CAP_TXQ: usize,
        const L2CAP_RXQ: usize,
    >(
        &self,
        adapter: &Adapter<'_, M, T, CONNS, CHANNELS, L2CAP_MTU, L2CAP_TXQ, L2CAP_RXQ>,
    ) -> Result<BdAddr, AdapterError<T::Error>> {
        let addr = adapter.connections.peer_address(self.handle)?;
        Ok(addr)
    }

    pub async fn rssi<
        M: RawMutex,
        T,
        const CONNS: usize,
        const CHANNELS: usize,
        const L2CAP_TXQ: usize,
        const L2CAP_RXQ: usize,
    >(
        &self,
        adapter: &Adapter<'_, M, T, CONNS, CHANNELS, L2CAP_TXQ, L2CAP_RXQ>,
    ) -> Result<i8, AdapterError<T::Error>>
    where
        T: ControllerCmdSync<ReadRssi>,
    {
        let ret = adapter.command(ReadRssi::new(self.handle)).await?;
        Ok(ret.rssi)
    }

    pub async fn set_connection_params<
        M: RawMutex,
        T,
        const CONNS: usize,
        const CHANNELS: usize,
        const L2CAP_TXQ: usize,
        const L2CAP_RXQ: usize,
    >(
        &self,
        adapter: &Adapter<'_, M, T, CONNS, CHANNELS, L2CAP_TXQ, L2CAP_RXQ>,
        params: ConnectParams,
    ) -> Result<(), AdapterError<T::Error>>
    where
        T: ControllerCmdAsync<LeConnUpdate>,
    {
        adapter
            .async_command(LeConnUpdate::new(
                self.handle,
                params.min_connection_interval.into(),
                params.max_connection_interval.into(),
                params.max_latency,
                params.supervision_timeout.into(),
                bt_hci::param::Duration::from_secs(0),
                bt_hci::param::Duration::from_secs(0),
            ))
            .await?;
        Ok(())
    }
}
