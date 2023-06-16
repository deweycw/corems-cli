use bollard::container::{RemoveContainerOptions};
use bollard::{Docker};
use bollard::image::{ RemoveImageOptions};
use bollard::volume::{RemoveVolumeOptions};

use std::env::{set_current_dir, current_dir};
use std::path::PathBuf;

use clap::{Parser, Subcommand, Args, ValueEnum};

mod assign;
use crate::assign::find_cards::*;
mod container_env;
use crate::container_env::Load_Container_Env::*; 

use futures::executor::block_on;

#[derive(Parser)]
#[command(name = "corems-cli")]
#[command(author = "Christian Dewey <dewey.christian@gmail.com>")]
#[command(version = "beta")]
#[command(about = "command line tool for formula assignments with CoreMS",long_about = "This tool leverages CoreMS (https://github.com/EMSL-Computing/CoreMS; Yuri E. Corilo, William R. Kew, Lee Ann McCue (2021, March 27). EMSL-Computing/CoreMS: CoreMS 1.0.0 (Version v1.0.0), as developed on Github. Zenodo. http://doi.org/10.5281/zenodo.4641553), a comprehensive Python framework for analysis of high resolution ESI mass spectrometry data. The tool creates a containerized deployment of CoreMS and facilitates communication between the user's local system and the CoreMS container. Assignment parameters are defined in a text-based input file or a user-provided Python script. The user runs the tool within a directory containing the raw MS data files (.RAW), the input file or Python script, and a peak list for calibration. A .csv file with assignment results is generated and saved within the directory containing the raw files.")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    ///remote host of container environment; not currently implemented
    #[clap(default_value="None", long)]
    remote_host: Option<String>,
    /// CoreMS image for building CoreMS container
    #[arg(default_value="deweycw/corems-cli", long)]
    corems_image: Option<String>,
    /// CoreMS container to use for assignments
    #[arg(default_value="corems-cli", long)]
    corems_container: Option<String>,
    /// Database container to pair with container
    #[arg(default_value="corems-cli-molformdb-1", long)]
    db_container: Option<String>,
    /// Database volume to pair with container
    #[arg(default_value="corems-cli_db-volume", long)]
    db_volume: Option<String>,

}

#[derive(Subcommand)]
enum Commands {
    /// Runs formula assignment, per parameters defined in input file, on raw data files within the working directory
    Assign(AssignArgs),
    /// Deletes, pulls, and rebuilds container environment; extent of reset can be specified
    Reload(ReloadArgs),
}

#[derive(Args)]
#[group(required = false, multiple = false)]
struct AssignArgs {
    /// text-based input file defining assignment parameters
    #[arg(default_value="corems.in",short, long)]
    input_file: Option<PathBuf>,
    /// user-generated Python script defining assignment parameters
    #[arg(default_value="None",short, long)]
    script: Option<String>,
}

#[derive(Args)]
struct ReloadArgs {
    #[arg(value_enum,required = true)]
    extent: Reload,
    /// Docker image to use in rebuild
    #[arg(default_value="deweycw/corems-cli", short, long)]
    pull_image: Option<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Reload {
    /// Deletes Postgres image, database volume, and CoreMS image; pulls new images from DockerHub.
    All,
    /// Deletes and pulls CoreMS image only; new image can be specified with --pull-image option.
    Corems,
    /// Deletes database volume. 
    Database,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {

    let working_dir: std::path::PathBuf = current_dir().unwrap();
    set_current_dir(&working_dir).expect("Couldn't change into current directory.");
    let cwd: String = working_dir.into_os_string().into_string().unwrap();

    let cli = Cli::parse();

    let corems_container_arg: &Option<String> = &cli.corems_container;
    let db_container_arg: &Option<String> = &cli.db_container;
    let corems_image_arg: &Option<String> = &cli.corems_image;
    let db_volume_arg: &Option<String> = &cli.db_volume;

    let corems_container = corems_container_arg.as_ref().unwrap();
    let db_container = db_container_arg.as_ref().unwrap();
    let corems_image = corems_image_arg.as_ref().unwrap();
    let db_volume = db_volume_arg.as_ref().unwrap();

    let mut content = String::from("empty");
    let mut exec_script = String::from("/CoreMS/usrdata/corems_input.py");

    let docker = Docker::connect_with_socket_defaults().unwrap();

    

    match &cli.command {

        Commands::Assign(input_file) => {
            let input_file_arg: &Option<PathBuf> = &input_file.input_file;
            content = std::fs::read_to_string(input_file_arg.as_deref().unwrap()).expect("could not read input (.in) file");
            find_cards(&content);
            load_container(&docker,  &corems_container, &db_container, &corems_image, &exec_script, &cwd).await?;
           
        }

        Commands::Assign(script) => {
            let script_arg: &Option<String> = &script.script;
            if script_arg.as_deref().unwrap() != String::from("None") {
                exec_script = String::from("/CoreMS/usrdata/");
                exec_script.push_str(script_arg.as_deref().unwrap());
                println!("{exec_script}");
                load_container(&docker,  &corems_container, &db_container, &corems_image, &exec_script, &cwd).await?;
            } 
        }


        Commands::Reload(all) => {
            let options = Some(RemoveContainerOptions{
                force: true,
                v: true,
                ..Default::default()
            });
    
            let _ = docker.remove_container(&corems_container,options).await?;
            let _ = docker.remove_container(&db_container,options).await?;
    
            let remove_options = Some(RemoveImageOptions {
                force: true,
                ..Default::default()
            });
            
            let _ = docker.remove_image("postgres", remove_options, None).await;
            let _ = docker.remove_image(&corems_image, remove_options, None).await;

            let remove_options = Some(RemoveVolumeOptions {
                force: true,
            });
            
            let _ = docker.remove_volume(&db_volume, remove_options).await?;
        }

        Commands::Reload(corems) => {
            let options = Some(RemoveContainerOptions{
                force: true,
                v: true,
                ..Default::default()
            });
    
            let _ = docker.remove_container(&corems_container,options).await?;
        
            let remove_options = Some(RemoveImageOptions {
                force: true,
                ..Default::default()
            });
            
            let _ = docker.remove_image(&corems_image, remove_options, None).await?;
        }

        Commands::Reload(database) => {
            let remove_options = Some(RemoveVolumeOptions {
                force: true,
            });
            
            let _ = docker.remove_volume(&db_volume, remove_options).await?;
        }
    }

Ok(())
}

