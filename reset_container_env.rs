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

use futures_util::{StreamExt, TryStreamExt};
use std::io::{stdout, Read, Write};
use std::time::Duration;
use std::collections::HashMap;
use std::env::{set_current_dir, current_dir, set_var};

use tokio::io::AsyncWriteExt;
use tokio::task::spawn;
use tokio::time::sleep;

use clap::{Parser, Subcommand, Args, ValueEnum};

#[macro_use]
pub mod common;
use crate::common::*;

mod assign;
use crate::assign::find_cards::*;

const DB_IMAGE: &str = "postgres";




#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {


    let args = Cli::parse();

    let mut docker = Docker::connect_with_socket_defaults().unwrap();

    match &args.command {

        Commands::Assign(assign) => {
            if script == String::from("None"){
                let content = std::fs::read_to_string(input_file).expect("could not read input (.in) file");
                find_cards(&content);
            } else if script != String::from("None"){
                exec_script = String::from("/CoreMS/usrdata/");
                exec_script.push_str(script);
                println!("{exec_script}");
            } 
        }

        Commands::Reset(All) => {
            let options = Some(RemoveContainerOptions{
                force: true,
                v: true,
                ..Default::default()
            });
    
            docker.remove_container(CONTAINER_NAME,options).await?;
            docker.remove_container("corems-cli-molformdb-1",options).await?;
    
            let remove_options = Some(RemoveImageOptions {
                force: true,
                ..Default::default()
            });
            
            docker.remove_image("postgres", remove_options, None);
            docker.remove_image(IMAGE, remove_options, None);

            let remove_options = Some(RemoveVolumeOptions {
                force: true,
            });
            
            docker.remove_volume("corems-cli_db-volume", remove_options).await?;
        }

        Commands::Reset(Corems) => {
            let options = Some(RemoveContainerOptions{
                force: true,
                v: true,
                ..Default::default()
            });
    
            docker.remove_container(CONTAINER_NAME,options).await?;
        
            let remove_options = Some(RemoveImageOptions {
                force: true,
                ..Default::default()
            });
            
            docker.remove_image(IMAGE, remove_options, None);
        }
        Commands::Reset(Database) => {
            let remove_options = Some(RemoveVolumeOptions {
                force: true,
            });
            
            docker.remove_volume("corems-cli_db-volume", remove_options).await?;
        }
    }


  
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
            source: Some(String::from(CWD)),
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
        


    let corems_id = &docker
        .create_container(
            Some(CreateContainerOptions {
                name: CONTAINER_NAME,
                platform: Some("linux/amd64"),
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

    docker.start_container::<String>(&corems_id, None).await?;



    let connect_network_options = ConnectNetworkOptions {
        container: CONTAINER_NAME,
        endpoint_config: EndpointSettings {
            ..Default::default()
        }
    };

    docker.connect_network("corems-cli_default", connect_network_options).await?;

    let database_arg: &Option<String> = &args.database;
    let DATABASE = database_arg.as_deref().unwrap();

    docker.start_container::<String>(DATABASE, None).await?;



    Ok(())
}

