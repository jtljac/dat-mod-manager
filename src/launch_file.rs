use std::process::Command;

fn run_exe() {
    let mut child = match Command::new("/bin/unshare")
        .current_dir("/media/games/SteamLibrary/steamapps/common/Fallout 4/")
        .env("STEAM_COMPAT_DATA_PATH","/media/games/SteamLibrary/steamapps/compatdata/377160")
        .env("STEAM_COMPAT_CLIENT_INSTALL_PATH", "/home/jacob/.steam/steam")
        .env("PROTON_DUMP_DEBUG_COMMANDS","1")
        .arg("-rm")
        .arg("/bin/sh")
        .arg("-c")
        .arg("/home/jacob/.local/share/Steam/ubuntu12_32/reaper SteamLaunch AppId=377160 -- /home/jacob/.local/share/Steam/compatibilitytools.d/GE-Proton7-49/proton waitforexitandrun \"/media/games/SteamLibrary/steamapps/common/Fallout 4/f4se_loader.exe\"")
        .spawn() {
        Ok(child) => child,
        Err(error) => {
            eprint!("an error occured: {error}");
            return;
        }
    };

    let ecode = child.wait()
        .expect("failed to wait on child");

    assert!(ecode.success());
}