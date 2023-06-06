//! This example will run a non-interactive command inside the container using `docker exec`

use bollard::container::{Config, RemoveContainerOptions};
use bollard::Docker;

use bollard::exec::{CreateExecOptions, StartExecResults};
use bollard::image::CreateImageOptions;
use futures_util::stream::StreamExt;
use futures_util::TryStreamExt;

const IMAGE: &str = "deweycw/corems:basic";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {

    let docker = Docker::connect_with_socket_defaults().unwrap();

    let mut port_bindings = ::std::collections::HashMap::new();
    port_bindings.insert(

        String::from("443/tcp"),
        Some(vec![PortBinding {
            host_ip: Some(String::from("127.0.0.1")),
            host_port: Some(String::from("5432")),
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

    let corems_config = Config {
        image: Some(IMAGE),
        tty: Some(true),
        ..Default::default()
    };

    let id = docker
        .create_container::<&str, &str>(None, corems_config)
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
                cmd: Some(vec!["cp", "/CoreMS/usrdata/assign_script_template.py", "/CoreMS/assign_script_template.py"]),
                cmd: Some(vec!["ls", "/CoreMS/"])
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
