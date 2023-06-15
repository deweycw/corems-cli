use bollard::container::{
    AttachContainerOptions, AttachContainerResults, Config, CreateContainerOptions,
    DownloadFromContainerOptions, InspectContainerOptions, KillContainerOptions,
    ListContainersOptions, LogsOptions, PruneContainersOptions, RemoveContainerOptions,
    RenameContainerOptions, ResizeContainerTtyOptions, RestartContainerOptions, StatsOptions,
    TopOptions, UpdateContainerOptions, UploadToContainerOptions, WaitContainerOptions,StopContainerOptions,
};
use bollard::{API_DEFAULT_VERSION,Docker};
use bollard::auth::DockerCredentials;

use bollard::exec::{CreateExecOptions, ResizeExecOptions, StartExecResults};
use bollard::image::{CreateImageOptions, PushImageOptions, TagImageOptions};
use bollard::network::{ConnectNetworkOptions, ListNetworksOptions, CreateNetworkOptions, InspectNetworkOptions};
use bollard::volume::{CreateVolumeOptions};
use bollard::models::*;

use futures_util::{StreamExt, TryStreamExt};
use std::io::{stdout, Read, Write};
use std::time::Duration;
use std::collections::HashMap;
use std::env::{set_current_dir, current_dir, set_var};

use tokio::io::AsyncWriteExt;
use tokio::task::spawn;
use tokio::time::sleep;

use clap::{Parser};

#[macro_use]
pub mod common;
use crate::common::*;

mod assign;
use crate::assign::find_cards::*;

const COREMS_IMAGE: &str = "deweycw/corems-cli";
const DB_IMAGE: &str = "postgres";


#[derive(Parser,Default,Debug,)]
struct Arguments {
    module: String,
    #[clap(default_value="corems.in",short, long)]
    input_file: Option<String>,
    #[clap(default_value="NO_REMOTE_HOST",short, long)]
    remote_host: Option<String>,
    #[clap(default_value="NO_PYTHON_SCRIPT",short, long)]
    script: Option<String>,
    #[clap(default_value="corems-cli",short, long)]
    container: Option<String>,
    #[clap(default_value="corems-cli-molformdb-1",short, long)]
    database: Option<String>,
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {


    let WORKING_DIR: std::path::PathBuf = current_dir().unwrap();
    set_current_dir(&WORKING_DIR).expect("Couldn't change into current directory.");
    let CWD: String = WORKING_DIR.into_os_string().into_string().unwrap();

    let args = Arguments::parse();

    let module_arg: String = args.module;
    let module = module_arg;

    let path_arg: &Option<String> = &args.input_file;
    let path_arg_deref = path_arg.as_deref().unwrap();
    let input_file: std::path::PathBuf = std::path::PathBuf::from(path_arg_deref);

    let remote_host_arg: &Option<String> = &args.remote_host;
    let remote_host = remote_host_arg.as_deref().unwrap();

    let script_arg: &Option<String> = &args.script;
    let script = script_arg.as_deref().unwrap();

    let mut exec_script = String::from("/CoreMS/usrdata/corems_input.py");

    if module == String::from("assign") && script == String::from("NO_PYTHON_SCRIPT"){
        let content = std::fs::read_to_string(input_file).expect("could not read input (.in) file");
        find_cards(&content);
    } else if module == String::from("assign") && script != String::from("NO_PYTHON_SCRIPT"){
        exec_script = String::from("/CoreMS/usrdata/");
        exec_script.push_str(script);
        println!("{exec_script}");
    } else {
        println!("{module} is not a recognized coresms-cli module");
    }
    
    let mut docker = Docker::connect_with_socket_defaults().unwrap();

    //if remote_host != String::from("NO_REMOTE_HOST") {

    //    docker = Docker::connect_with_socket(remote_host,120,API_DEFAULT_VERSION).unwrap();

  
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
                from_image: COREMS_IMAGE,
                ..Default::default()
            }),
            None,
            None, 
        )
        .try_collect::<Vec<_>>()
        .await?;
        
    let container_name_arg: &Option<String> = &args.container;
    let CONTAINER_NAME = container_name_arg.as_deref().unwrap();


    let corems_id = &docker
        .create_container(
            Some(CreateContainerOptions {
                name: CONTAINER_NAME,
                platform: Some("linux/amd64"),
            }),
            Config {
                image:Some(COREMS_IMAGE),
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







    // non interactive
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

