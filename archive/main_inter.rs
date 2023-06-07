//! This example will run a interactive command inside the container using `docker exec`,
//! passing trough input and output into the tty running inside the container

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

    #[cfg(not(windows))]
    let tty_size = terminal_size()?;

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
                name: "integration_test_mount_volume_container",
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

    let exec = docker
        .create_exec(
            &id,
            CreateExecOptions {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                attach_stdin: Some(true),
                tty: Some(true),
                cmd: Some(vec!["sh"]),
                ..Default::default()
            },
        )
        .await?
        .id;
    #[cfg(not(windows))]
    if let StartExecResults::Attached {
        mut output,
        mut input,
    } = docker.start_exec(&exec, None).await?
    {
        // pipe stdin into the docker exec stream input
        spawn(async move {
            let mut stdin = async_stdin().bytes();
            loop {
                if let Some(Ok(byte)) = stdin.next() {
                    input.write(&[byte]).await.ok();
                } else {
                    sleep(Duration::from_nanos(10)).await;
                }
            }
        });

        docker
            .resize_exec(
                &exec,
                ResizeExecOptions {
                    height: tty_size.1,
                    width: tty_size.0,
                },
            )
            .await?;

        // set stdout in raw mode so we can do tty stuff
        let stdout = stdout();
        let mut stdout = stdout.lock().into_raw_mode()?;

        // pipe docker exec output into stdout
        while let Some(Ok(output)) = output.next().await {
            stdout.write_all(output.into_bytes().as_ref())?;
            stdout.flush()?;
        }
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
