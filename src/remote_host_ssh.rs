use openssh::{Session, KnownHosts};
use std::process::{Command, Stdio};
use std::io::{self, Write};

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

    let mut remote_dir = String::from("\\Users\\");
    let str_split: Vec<&str> = remote_host.split("@").collect();
    let user_home = str_split[0];
    remote_dir.push_str(user_home);
    remote_dir.push_str("\\.corems_cli_transfer\\docker-compose.yml\\");

    let session = Session::connect_mux(remote_host, KnownHosts::Accept).await?;
    println!("\n...Checking running containers...\n") ;
    let mut powershell = session.command("powershell.exe")
        .arg("docker")
        .arg("container")
        .arg("ls").output().await?;
    io::stdout().write_all(&powershell.stdout).unwrap();
    io::stderr().write_all(&powershell.stderr).unwrap();

    println!("\n...Composing containers...\n");
    let mut powershell = session.command("powershell.exe")
        .arg("docker")
        .arg("compose")
        .arg("-f")
        .arg(remote_dir)
        .arg("up")
        .arg("-d").output().await?;
    io::stdout().write_all(&powershell.stdout).unwrap();
    io::stderr().write_all(&powershell.stderr).unwrap();

    Ok(())
}