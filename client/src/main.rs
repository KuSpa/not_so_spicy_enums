use serde::*;
use std::{net::SocketAddr};

use bevy::{log::LogPlugin, prelude::*};
use bevy_spicy_networking::{
    AppNetworkClientMessage, ClientMessage, ClientPlugin, NetworkClient, NetworkData,
    NetworkMessage, NetworkSettings,
};

#[allow(dead_code)]
fn main() {
    let mut app = App::build();
    app.add_plugins(MinimalPlugins)
        .add_startup_system(setup_networking.system())
        .add_plugin(ClientPlugin)
        .add_plugin(LogPlugin)
        .add_system(handle_pings.system());
    app.listen_for_client_message::<MyEnum>();
    app.run();
}

fn setup_networking(mut net: ResMut<NetworkClient>) {
    let ip_address = "127.0.0.1".parse().unwrap();
    let socket_address = SocketAddr::new(ip_address, 9999);
    net.connect(
        socket_address,
        NetworkSettings {
            max_packet_length: 10 * 1024 * 1024,
        },
    );
}

fn handle_pings(mut network_events: EventReader<NetworkData<MyEnum>>) {
    for event in network_events.iter() {
        info!("{:?}", event as &MyEnum);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub enum MyEnum{
    A,
}

#[typetag::serde]
impl NetworkMessage for MyEnum {}

impl ClientMessage for MyEnum {
    const NAME: &'static str = "Enum";
}
