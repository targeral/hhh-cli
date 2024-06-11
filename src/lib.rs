mod pkgs;
mod utils;
mod init;


use std::path::PathBuf;

use utils::update;

pub struct CommandContext {
    pub app_dir: PathBuf,
    pub run_in_global_env: bool,
}

impl CommandContext {
    pub fn new(cwd: PathBuf) -> Self {
    
        CommandContext {
            app_dir: cwd,
            run_in_global_env: true,
        }
    }
}

pub fn commands_init(context: CommandContext) {
    println!("appDir: {:?}, runInGlobalEnv: {}", context.app_dir, context.run_in_global_env);
    update::check_latest_version();
    init::run();
}

pub fn init_command_context() -> CommandContext {
    let path: std::path::PathBuf = std::env::current_dir().expect("Failed to get current directory");
    CommandContext::new(path)
}