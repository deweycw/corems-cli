use openssh::{Session, KnownHosts, Stdio};
use std::process::{Command};
use std::io::{self, Write};
use tokio::{task, time};
use std::time::Duration;


pub async fn sync_directory(remote_host: &str, cwd: &str) -> Result< (),Box<dyn std::error::Error + 'static>>{
    let mut remote_dir = String::from(remote_host);
    let remote_dir_root: &str = ":/cygdrive/c/users/";
    //let remote_dir_root: &str = ":/mnt/c/users/";
    remote_dir.push_str(remote_dir_root);
    let str_split: Vec<&str> = remote_host.split("@").collect();
    let user_home = str_split[0];
    remote_dir.push_str(user_home);
    remote_dir.push_str("/Desktop/corems/");

    println!("\n...starting data transfer to {}\n", remote_dir);

    let rsync_cmnd = Command::new("rsync")
        .arg("-aP")
        .arg("-e")
        .arg("ssh")
        .arg("/Users/christiandewey/test-sync/data-temp/")
        .arg(remote_dir)
        //.arg("--rsync-path='C:\\ProgramData\\chocolatey\\bin\\rsync.exe'")
        .output()
        .expect("rsync command failed to start");

    io::stdout().write_all(&rsync_cmnd.stdout).unwrap();
    io::stderr().write_all(&rsync_cmnd.stderr).unwrap();
    
    Ok(())
}


pub async fn run_remote_host(remote_host: &str) -> Result< (), Box<dyn std::error::Error + 'static>> {
    println!("\n...running assignment - this may take some time...")
    let str_split: Vec<&str> = remote_host.split("@").collect();
    let user_home = str_split[0];

    let mut docker_compose = String::from("/Users/");
    docker_compose.push_str(user_home);
    docker_compose.push_str("/.corems_cli_transfer/docker-compose.yml");

    let mut log_out = String::from("/Users/");
    log_out.push_str(user_home); 
    log_out.push_str("/.corems_cli_transfer/log.out");

    let session = Session::connect_mux(remote_host, KnownHosts::Accept).await?;

    //let mut move_data = &session.command("wsl.exe")
        //.arg("rsync")
//let mut move_data = &session.command("rsync")
 //       .arg("-aP")
 //       .arg(&data_c_path)
 //       .arg(&wsl_home)
 //       .output().await?;
 //   io::stdout().write_all(&move_data.stdout).unwrap();
 //   io::stderr().write_all(&move_data.stderr).unwrap();

    exec_docker(&session, &remote_host, &docker_compose).await?;

    //let session2 = Session::connect_mux(remote_host, KnownHosts::Accept).await?;

    //loop{
        //print_out(&session2,&remote_host).await?;
    //}



    async fn exec_docker(session: &Session, remote_host: &str, docker_compose: &str) -> Result< (), Box<dyn std::error::Error + 'static>>{ 

        //let mut docker_compose_cmd = session.command("docker")
          //  .args(["compose","-f",docker_compose, "up", "-d"])
          //  //.args(["sudo","docker","compose","-f",&docker_compose, "up","-d", "'>", &log_out_p])
          //  .spawn().await?;
        
        let mut docker_exec_cmd = session.command("docker")
            .args(["exec","corems-dewey-1","python3","-u","/CoreMS/usrdata/corems_input.py"])
            //.args(["sudo","docker","compose","-f",&docker_compose, "up","-d", "'>", &log_out_p])
            .output().await?;

        io::stdout().write_all(&docker_exec_cmd.stdout).unwrap();
        io::stderr().write_all(&docker_exec_cmd.stderr).unwrap();

        Ok(())
    }
    Ok(())
}


pub async fn print_out(remote_host: &str) -> Result< (), Box<dyn std::error::Error + 'static>>{

    let session = Session::connect_mux(remote_host, KnownHosts::Accept).await?;
    //let mut interval = time::interval(Duration::from_millis(10));

    //interval.tick().await;

    let mut print_docker_logs = session.command("docker")
        .args(["logs", "--tail","all","corems-dewey-1"])
        .output().await?;

    io::stdout().write_all(&print_docker_logs.stdout).unwrap();
    io::stderr().write_all(&print_docker_logs.stderr).unwrap();
    
   Ok(())     
   //-> Result< (), Box<dyn std::marker::Send + 'static>>
}



