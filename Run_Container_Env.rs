use bollard::container::{
    Config, CreateContainerOptions,KillContainerOptions,
    ListContainersOptions, PruneContainersOptions, RemoveContainerOptions,
    ResizeContainerTtyOptions, RestartContainerOptions,
    StopContainerOptions,
};
use bollard::{API_DEFAULT_VERSION,Docker};
use bollard::auth::DockerCredentials;

use bollard::exec::{CreateExecOptions, ResizeExecOptions, StartExecResults};
use bollard::image::{CreateImageOptions, PushImageOptions, TagImageOptions, RemoveImageOptions};
use bollard::network::{ConnectNetworkOptions, ListNetworksOptions, CreateNetworkOptions, InspectNetworkOptions};
use bollard::volume::{CreateVolumeOptions, RemoveVolumeOptions};
use bollard::models::*;

use std::collections::HashMap;
use futures_util::StreamExt;
  
pub async fn run_corems(docker: &Docker, exec_script: &String, corems_container: &String, corems_image: &String, cwd: &String) -> Result<(), Box<dyn std::error::Error + 'static>> {
    
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
            target: Some(String::from("/CoreMS/usrdata")),
            source: Some(String::from(cwd)),
            typ: Some(MountTypeEnum::BIND),
            consistency: Some(String::from("default")),
            ..Default::default()
        }]),
        port_bindings: Some(port_bindings),
        ..Default::default()
    };

    let corems_id = &docker
        .create_container(
            Some(CreateContainerOptions {
                name: corems_container,
                platform: Some("linux/amd64"),
            }),
            Config {
                image:Some(corems_image),
                tty: Some(true),
                host_config: Some(host_config),
                ..Default::default()
            },
        )
        .await?
        .id;


    let exec = docker
        .create_exec(
            &corems_id,
            CreateExecOptions {
                attach_stdout: Some(true),
                attach_stderr: Some(true),
                cmd: Some(vec!["python3", &exec_script]),
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
            &corems_id,
            Some(RemoveContainerOptions {
                force: true,
                ..Default::default()
            }),
            )
            .await?;

Ok(())
} 