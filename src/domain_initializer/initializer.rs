use std::sync::Arc;
use ipc_channel::ipc;
use ipc_channel::ipc::{IpcReceiver, IpcSender};
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use crate::account::service::account_service_impl::AccountServiceImpl;

use crate::client_socket_accept::controller::client_socket_accept_controller::ClientSocketAcceptController;
use crate::client_socket_accept::controller::client_socket_accept_controller_impl::ClientSocketAcceptControllerImpl;

use crate::server_socket::service::server_socket_service_impl::ServerSocketServiceImpl;
use crate::thread_worker::service::thread_worker_service_impl::ThreadWorkerServiceImpl;

use crate::common::mpsc::mpsc_creator::mpsc_channel::define_channel;
use crate::mysql_config::mysql_connection::MysqlDatabaseConnection;

use crate::receiver::controller::server_receiver_controller::ServerReceiverController;
use crate::receiver::controller::server_receiver_controller_impl::ServerReceiverControllerImpl;
use crate::response_generator::response_type::ResponseType;
use crate::transmitter::controller::transmitter_controller::TransmitterController;
use crate::transmitter::controller::transmitter_controller_impl::TransmitterControllerImpl;

define_channel!(AcceptorReceiverChannel, Arc<Mutex<TcpStream>>);
define_channel!(AcceptorTransmitterChannel, Arc<Mutex<TcpStream>>);
define_channel!(ReceiverTransmitterChannel, Arc<Mutex<ResponseType>>);

pub struct DomainInitializer;

impl DomainInitializer {
    pub fn init_server_socket_domain(&self) {
        let _ = ServerSocketServiceImpl::get_instance();
    }
    pub fn init_thread_worker_domain(&self) {
        let _ = ThreadWorkerServiceImpl::get_instance();
    }

    // pub fn init_mysql_database(&self) {
    //     let _ = MysqlDatabaseConnection::get_instance();
    // }

    pub fn init_account_domain(&self) {
        let _ = AccountServiceImpl::get_instance();
    }

    pub async fn init_client_socket_accept_domain(&self,
                                                  acceptor_receiver_channel_arc: Arc<AcceptorReceiverChannel>,
                                                  acceptor_transmitter_channel_arc: Arc<AcceptorTransmitterChannel>) {

        let client_socket_accept_controller_mutex = ClientSocketAcceptControllerImpl::get_instance();
        let mut client_socket_accept_controller = client_socket_accept_controller_mutex.lock().await;

        client_socket_accept_controller.inject_acceptor_receiver_channel(acceptor_receiver_channel_arc).await;
        client_socket_accept_controller.inject_acceptor_transmitter_channel(acceptor_transmitter_channel_arc).await;
        // client_socket_accept_controller.inject_acceptor_transmitter_channel()
    }

    pub async fn init_receiver_domain(&self,
                                      acceptor_receiver_channel_arc: Arc<AcceptorReceiverChannel>,
                                      receiver_transmitter_channel_arc: Arc<ReceiverTransmitterChannel>) {
        let server_receiver_controller_mutex = ServerReceiverControllerImpl::get_instance();
        let mut server_receiver_controller = server_receiver_controller_mutex.lock().await;

        server_receiver_controller.inject_acceptor_receiver_channel(acceptor_receiver_channel_arc).await;
        server_receiver_controller.inject_receiver_transmitter_channel(receiver_transmitter_channel_arc).await;
    }

    pub async fn init_transmitter_domain(&self,
                                         acceptor_transmitter_channel_arc: Arc<AcceptorTransmitterChannel>,
                                         receiver_transmitter_channel_arc: Arc<ReceiverTransmitterChannel>) {

        let transmitter_controller_mutex = TransmitterControllerImpl::get_instance();
        let mut transmitter_controller = transmitter_controller_mutex.lock().await;

        transmitter_controller.inject_acceptor_transmitter_channel(acceptor_transmitter_channel_arc).await;
        transmitter_controller.inject_receiver_transmitter_channel(receiver_transmitter_channel_arc).await;
    }

    pub async fn init_every_domain(&self) {
        /* IPC Channel List */
        let acceptor_reciever_channel = AcceptorReceiverChannel::new(1);
        let acceptor_reciever_channel_arc = Arc::new(acceptor_reciever_channel.clone());

        let acceptor_transmitter_channel = AcceptorTransmitterChannel::new(1);
        let acceptor_transmitter_channel_arc = Arc::new(acceptor_transmitter_channel.clone());

        let receiver_transmitter_channel = ReceiverTransmitterChannel::new(1);
        let receiver_transmitter_channel_arc = Arc::new(receiver_transmitter_channel.clone());

        /* Business Domain List */
        self.init_account_domain();

        /* Core Domain List */
        self.init_server_socket_domain();
        self.init_thread_worker_domain();
        self.init_client_socket_accept_domain(
            acceptor_reciever_channel_arc.clone(), acceptor_transmitter_channel_arc.clone()).await;
        self.init_receiver_domain(
            acceptor_reciever_channel_arc.clone(), receiver_transmitter_channel_arc.clone()).await;
        self.init_transmitter_domain(
            acceptor_transmitter_channel_arc.clone(), receiver_transmitter_channel_arc.clone()).await;
    }
}

