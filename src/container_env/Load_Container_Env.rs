use bollard::container::{
    Config, CreateContainerOptions, 
    ListContainersOptions, RemoveContainerOptions,
};
use bollard::{Docker};
use bollard::image::{CreateImageOptions};
use bollard::network::{ConnectNetworkOptions};
use bollard::exec::{StartExecResults,CreateExecOptions};
use bollard::models::*;

use futures_util::{TryStreamExt, StreamExt};
use std::collections::HashMap;

pub async fn load_container(docker: &Docker, corems_container: &str, db_container: &str, corems_image: &str, exec_script: &str, cwd: &str) -> Result< (), Box<dyn std::error::Error + 'static>> {

    let mut filters = HashMap::new();
    filters.insert("name", vec![corems_container]);
    let options = Some(ListContainersOptions{
        all: true,
        filters,
        ..Default::default()
    });
    let running_containers = docker.list_containers(options).await?;
    let x = running_containers.len();
    if x > 2 {

        let options = Some(RemoveContainerOptions{
            force: true,
            v: true,
            ..Default::default()
        });

        docker.remove_container(corems_container,options).await?;

    }
 
    
    //if remote_host != String::from("NO_REMOTE_HOST") {

    //    docker = Docker::connect_with_socket(remote_host,120,API_DEFAULT_VERSION).unwrap();

  
    let mut port_bindings = ::std::collections::HashMap::new();
    port_bindings.insert(
        String::from("8080/tcp"),
        Some(vec![PortBinding {
            host_ip: Some(String::from("127.0.0.1")),
            host_port: Some(String::from("1986")),
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

    
    docker
        .create_image(
            Some(CreateImageOptions {
                from_image: corems_image,
                ..Default::default()
            }),
            None,
            None, 
        )
        .try_collect::<Vec<_>>()
        .await?;
        


    let corems_id = docker
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
    
    docker.start_container::<String>(&corems_id, None).await?;



    let connect_network_options = ConnectNetworkOptions {
        container: corems_container,
        endpoint_config: EndpointSettings {
            ..Default::default()
        }
    };

    docker.connect_network("corems-cli_default", connect_network_options).await?;

    docker.start_container::<String>(db_container, None).await?;

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