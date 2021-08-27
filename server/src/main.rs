use std::{net::SocketAddr, ops::RangeFrom};

use bevy::{input::mouse::MouseButtonInput, prelude::*};
use bevy_spicy_networking::{NetworkServer, ServerPlugin};
use client::MyEnum;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_networking.system())
        .add_plugin(ServerPlugin)
        .add_system(click.system())
        .run();
}

fn setup_networking(mut net: ResMut<NetworkServer>) {
    let ip_address = "127.0.0.1".parse().expect("Could not parse ip address");
    let socket_address = SocketAddr::new(ip_address, 9999);
    match net.listen(socket_address) {
        Ok(_) => (),
        Err(err) => {
            error!("Could not start listening: {}", err);
            panic!();
        }
    }
    info!("Started listening for new connections!");
}

struct Counter(RangeFrom<usize>);
impl Default for Counter {
    fn default() -> Self {
        Self(0..)
    }
}

fn click(mut counter: Local<Counter>, net:Res<NetworkServer>,mut clicks: EventReader<MouseButtonInput>,) {
    for event in clicks.iter(){
        if event.button == MouseButton::Left && event.state.is_pressed(){
            let num = counter.0.next().unwrap();
            info!("Ping {:?}", num);
            net.broadcast(MyEnum::A);
        }
    }
}
