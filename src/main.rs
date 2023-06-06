use bollard::container::{
    AttachContainerOptions, AttachContainerResults, Config, CreateContainerOptions,
    DownloadFromContainerOptions, InspectContainerOptions, KillContainerOptions,
    ListContainersOptions, LogsOptions, PruneContainersOptions, RemoveContainerOptions,
    RenameContainerOptions, ResizeContainerTtyOptions, RestartContainerOptions, StatsOptions,
    TopOptions, UpdateContainerOptions, UploadToContainerOptions, WaitContainerOptions,
};
use bollard::Docker;

use bollard::exec::{CreateExecOptions, ResizeExecOptions, StartExecResults};
use bollard::image::{CreateImageOptions, PushImageOptions, TagImageOptions};
use bollard::models::*;
use futures_util::{StreamExt, TryStreamExt};
use std::io::{stdout, Read, Write};
use std::time::Duration;
#[cfg(not(windows))]
use termion::raw::IntoRawMode;
#[cfg(not(windows))]
use termion::{async_stdin, terminal_size};
use tokio::io::AsyncWriteExt;
use tokio::task::spawn;
use tokio::time::sleep;

#[macro_use]
pub mod common;
use crate::common::*;


const IMAGE: &str = "deweycw/corems:basic";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {

    let docker = Docker::connect_with_socket_defaults().unwrap();

    let mut port_bindings = ::std::collections::HashMap::new();
    port_bindings.insert(

        String::from("8080/tcp"),
        Some(vec![PortBinding {
            host_ip: Some(String::from("127.0.0.1")),
            host_port: Some(String::from("8080")),
        }]),
    );


    let host_config = HostConfig {
        mounts: Some(vec![Mount {
            target: Some(if cfg!(windows) {
                String::from("C:\\Windows\\Temp")
            } else {
                String::from("/CoreMS/usrdata")
            }),
            source: Some(if cfg!(windows) {
                String::from("C:\\Windows\\Temp")
            } else {
                String::from("/Users/christiandewey/corems-docker/corems/rawfiles")
            }),
            typ: Some(MountTypeEnum::BIND),
            consistency: Some(String::from("default")),
            ..Default::default()
        }]),
        port_bindings: Some(port_bindings),
        ..Default::default()
    };

    docker
        .create_image(
            Some(CreateImageOptions {
                from_image: IMAGE,
                ..Default::default()
            }),
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await?;
        

    let id = &docker
        .create_container(
            Some(CreateContainerOptions {
                name: "test2",
                platform: None,
            }),
            Config {
                image:Some(IMAGE),
                tty: Some(true),
                host_config: Some(host_config),
                ..Default::default()
            },
        )
        .await?
        .id;

    docker.start_container::<String>(&id, None).await?;

    // non interactive
    let exec = docker
        .create_exec(
            &id,
            CreateExecOptions {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                cmd: Some(vec!["python3", "/CoreMS/usrdata/corems_input.py"]),
                ..Default::default()
            },
        )
        .await?
        .id;
    if let StartExecResults::Attached { mut output, .. } = docker.start_exec(&exec, None).await? {
        while let Some(Ok(msg)) = output.next().await {
            print!("{}", msg);
        }
    } else {
        unreachable!();
    }

    docker
        .remove_container(
            &id,
            Some(RemoveContainerOptions {
                force: true,
                ..Default::default()
            }),
        )
        .await?;

    Ok(())
}
