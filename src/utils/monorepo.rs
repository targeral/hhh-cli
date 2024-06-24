use std::{fs, path::{Path, PathBuf}};
use super::constants::PNPM_MONO_CONFIG_JSON;



pub fn is_pnpm_monorepo(cwd: &Path) -> (bool, PathBuf) {
    let mut find_path = cwd.to_path_buf();
    let have_pnpm_workspace_file = |mono_repo_root_path: &Path| -> bool {
        let pnpm_workspace_file_abs_path = mono_repo_root_path.join(PNPM_MONO_CONFIG_JSON);
        match fs::metadata(&pnpm_workspace_file_abs_path).map(|metadata| metadata.is_file()) {
            Ok(result) => result,
            Err(_) => false,
        }
        // match pnpm_workspace_file_abs_path.as_path().try_exists() {
        //     Ok(_) => true,
        //     Err(e) => false,
        // }
    };

    let repo_is_mono = loop {
        if have_pnpm_workspace_file(&find_path) {
            break true;
        }

        match find_path.parent() {
            Some(dirname) => {
                find_path = dirname.to_path_buf();
            },
            None => break false
        }
    };

    return (repo_is_mono, find_path)
}