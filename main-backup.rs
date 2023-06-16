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

const IMAGE: &str = "deweycw/corems-cli";
const DB_IMAGE: &str = "postgres";


#[derive(Parser)]
#[command(name = "corems-cli")]
#[command(author = "Christian Dewey <dewey.christian@gmail.com>")]
#[command(version = "beta")]
#[command(about = "command line tool for formula assignments with CoreMS",long_about = "This tool leverages CoreMS (https://github.com/EMSL-Computing/CoreMS), a comprehensive Python framework for analysis of high resolution ESI mass spectrometry data. The tool creates a containerized deployment of CoreMS and facilitates communication between the user's local system and the CoreMS container. Assignment parameters are defined in a text-based input file or a user-provided Python script. The user runs the tool within a directory containing the raw MS data files (.RAW), the input file or Python script, and a peak list for calibration. A .csv file with assignment results is generated and saved within the directory containing the raw files.")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(default_value="corems.in",short, long)]
    input_file: Option<String>,
    #[clap(default_value="None",short, long)]
    script: Option<String>,
    ///remote host of container environment; not currently implemented
    #[clap(default_value="None", long)]
    remote_host: Option<String>,
    #[clap(default_value="corems-cli", long)]
    container: Option<String>,
    #[clap(default_value="corems-cli-molformdb-1", long)]
    database: Option<String>,

}

#[derive(Subcommand)]
enum Commands {
    /// Runs formula assignment, per parameters defined in input file, on raw data files within the working directory
    Assign(AssignArgs),
    /// Deletes, pulls, and rebuilds container environment; extent of reset can be specified
    Reset(ResetArgs),
}

#[derive(Args)]
struct AssignArgs {
    /// text-based input file defining assignment parameters
    #[arg(default_value="corems.in",short, long)]
    input_file: Option<String>,
    /// user-generated Python script defining assignment parameters
    #[arg(default_value="None",short, long)]
    script: Option<String>,
    /// CoreMS container in which to run assignments
    #[arg(default_value="corems-cli", long)]
    container: Option<String>,
    /// Database to pair with container
    #[arg(default_value="corems-cli-molformdb-1", long)]
    database: Option<String>,
}

#[derive(Args)]
struct ResetArgs {
    #[arg(value_enum)]
    extent: Reset,
    /// Docker image to use in rebuild
    #[arg(default_value="deweycw/corems-cli", long)]
    pull_image: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Reset {
    /// Deletes Postgres image, database volume, and CoreMS image; pulls new images from DockerHub.
    All,
    /// Deletes and pulls CoreMS image only; new image can be specified with --pull-image option.
    Corems,
    /// Deletes database volume. 
    Database,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {


    let WORKING_DIR: std::path::PathBuf = current_dir().unwrap();
    set_current_dir(&WORKING_DIR).expect("Couldn't change into current directory.");
    let CWD: String = WORKING_DIR.into_os_string().into_string().unwrap();

    let args = Cli::parse();


    let container_name_arg: &Option<String> = &args.container;
    let CONTAINER_NAME = container_name_arg.as_deref().unwrap();

    //let module_arg: String = Commands::Module { module };
    //let module = module_arg;

    let path_arg: &Option<String> = &args.input_file;
    let path_arg_deref = path_arg.as_deref().unwrap();
    let input_file: std::path::PathBuf = std::path::PathBuf::from(path_arg_deref);

    let remote_host_arg: &Option<String> = &args.remote_host;
    let remote_host = remote_host_arg.as_deref().unwrap();

    let script_arg: &Option<String> = &args.script;
    let script = script_arg.as_deref().unwrap();

    //let reset_arg: &Option<String> = &args.reset;
    //let reset = reset_arg.as_deref().unwrap();

    let mut exec_script = String::from("/CoreMS/usrdata/corems_input.py");



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


    let mut filters = HashMap::new();
    filters.insert("name", vec![CONTAINER_NAME]);
    let options = Some(ListContainersOptions{
        all: true,
        filters,
        ..Default::default()
    });

    let running_containers = docker.list_containers(options).await?;
    let x = running_containers.len();
    
    if x > 0 {

        let options = Some(RemoveContainerOptions{
            force: true,
            v: true,
            ..Default::default()
        });

        docker.remove_container(CONTAINER_NAME,options).await?;

    }

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

