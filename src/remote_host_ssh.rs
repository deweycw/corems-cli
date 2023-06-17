use openssh::{Session, KnownHosts, Stdio};
use std::process::{Command};
use std::io::{self, Write};
use std::{
    thread,
    time::{Duration, Instant},
};

pub async fn sync_directory(remote_host: &str, cwd: &str) -> Result< (),Box<dyn std::error::Error + 'static>>{
    let mut remote_dir = String::from(remote_host);
    let remote_dir_root: &str = ":/cygdrive/c/users/";
    remote_dir.push_str(remote_dir_root);
    let str_split: Vec<&str> = remote_host.split("@").collect();
    let user_home = str_split[0];
    remote_dir.push_str(user_home);
    remote_dir.push_str("/.corems_cli_transfer/");

    println!("\n...Starting data transfer to {}\n", remote_host);

    let rsync_cmnd = Command::new("rsync")
        .arg("-aP")
        .arg("/Users/christiandewey/test-sync/data-temp/")
        .arg(remote_dir)
        .output()
        .expect("rsync command failed to start");

    io::stdout().write_all(&rsync_cmnd.stdout).unwrap();
    io::stderr().write_all(&rsync_cmnd.stderr).unwrap();
    
    Ok(())
}


pub async fn run_remote_host(remote_host: &str) -> Result< (), Box<dyn std::error::Error + 'static>> {

    let mut docker_compose = String::from("\\Users\\");
    let str_split: Vec<&str> = remote_host.split("@").collect();
    let user_home = str_split[0];
    docker_compose.push_str(user_home);
    docker_compose.push_str("\\.corems_cli_transfer\\");
    docker_compose.push_str("docker-compose.yml\\");


    let mut log_file = String::from("\\Users\\");
    let str_split: Vec<&str> = remote_host.split("@").collect();
    let user_home = str_split[0];
    log_file.push_str(user_home);
    log_file.push_str("\\.corems_cli_transfer\\");
    log_file.push_str("log.txt");
    println!("{}",log_file);

    let session = Session::connect_mux("bioinfo-pc", KnownHosts::Accept).await?;
    
    exec_docker(&session).await?;

    loop{
        print_out(&session).await?;
    }

    async fn print_out(session: &Session) -> Result< (), Box<dyn std::error::Error + 'static>>{
        let mut cmdline = session.command("powershell.exe")
        .arg("-Command")
        .raw_arg("\"& Get-Content -Path \".logfile.txt\" -Tail 1\"")
        .output().await?;
        io::stdout().write_all(&cmdline.stdout).unwrap();
        io::stderr().write_all(&cmdline.stderr).unwrap();
        Ok(())

    }
    
    async fn exec_docker(session: &Session) -> Result< (), Box<dyn std::error::Error + 'static>>{
        println!("\n...Checking running containers...\n") ;
        let mut cmdline = session.command("powershell.exe")
            .arg("-Command")
            .raw_arg("\"& docker container ls > .logfile.txt | cat .logfile.txt\"")
            .output().await?;
        io::stdout().write_all(&cmdline.stdout).unwrap();
        io::stderr().write_all(&cmdline.stderr).unwrap();

        //println!("\n...Composing containers...\n");
        let mut powershell = session.command("powershell.exe")
            .arg("-Command")
            .raw_arg("\"& docker compose -f .docker-compose.yml up -d > .logfile.txt | cat .logfile.txt\"")
            .output()
            .await?;
        io::stdout().write_all(&powershell.stdout).unwrap();
        io::stderr().write_all(&powershell.stderr).unwrap();
        Ok(())
    }
    Ok(())
}