mod context;

use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use std::process::{self, Command};

use context::{InitContext, CreateAction};

use crate::pkgs::pkg_json::PackageJsonStandard;
use crate::utils::config::{check_config_file_if_exist, read_hhh_config, HHHConfig};
use crate::utils::monorepo;

pub fn run(app_dir: &Path) {
    let mut context = InitContext::new();
    println!("run init command");
    let (is_pnpm_monorepo, monorepo_root_path) = monorepo::is_pnpm_monorepo(app_dir);
    context.set_is_pnpm(is_pnpm_monorepo);
    if is_pnpm_monorepo {
        context.set_pnpm_root_path(&monorepo_root_path);

        let output = Command::new("git")
            .arg("status")
            .current_dir(&monorepo_root_path)
            .output()
            .expect("fail to execute git status");
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if !stdout.contains("working tree clean") {
                println!("请先提交或者存储 Git 工作区内容");
                process::exit(0);
            }
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Error: {stderr}");
        }
    }

    let project_type = check_project_type_in_cwd(app_dir, is_pnpm_monorepo);

    match project_type {
        ProjectType::OtherNpmProject => {
            println!("当前项目不是 ModernJS 项目，请在 ModernJS 项目执行命令");
        },
        ProjectType::EmptyDir => {
            println!("当前目录不能为空目录，请先通过 `npx @modern-js/create [目录]` 命令创建 ModernJS 项目，然后在项目下执行该命令!");
        },
        ProjectType::NotNpmProject => {
            println!("当前目录不是一个 NPM 项目，请先通过 `npx @modern-js/create [目录]` 命令创建 ModernJS 项目，然后在项目下执行该命令!");
        },
        ProjectType::ModernJSSubProject => {
            context.set_create_action(CreateAction::SubModernJSProject);
        }
        ProjectType::ModernJSProject => {
            context.set_create_action(CreateAction::ModernJSProject);
        },
        ProjectType::NotSubProject => {
            println!("请在 ModernJS 子项目下执行该命令");
        }
    }

    match check_config_file_if_exist(app_dir) {
        Ok(result) => {
            context.set_config_exist(result);
            if result == true {
                context.set_config_exist(result);
                if let Ok(hhh_config) = read_hhh_config(app_dir) {
                    context.set_hhh_config(hhh_config);
                } else {
                    eprintln!("读取本地 hhh 配置文件失败");
                }
            } else {
                context.set_hhh_config(HHHConfig::default());
            }
        },
        Err(_) => {
            context.set_config_exist(false);
            context.set_hhh_config(HHHConfig::default());
        }
    }


    println!("{context}");
}

pub enum ProjectType {
    NotNpmProject,
    NotSubProject,
    ModernJSProject,
    ModernJSSubProject,
    EmptyDir,
    OtherNpmProject,
}

fn is_empty_dir(p: &Path) -> bool {
    let err = format!("读取 {} 目录失败", p.to_str().unwrap());
    let mut entries = fs::read_dir(p).expect(err.as_str());
    entries.next().is_none()
}

fn check_project_type_in_cwd(cwd: &Path, is_mono: bool) -> ProjectType {
    let pkg_json_abs_path = Path::new(cwd).join("package.json");
    let exist_pkg = pkg_json_abs_path.exists();

    if is_empty_dir(cwd) {
        return ProjectType::EmptyDir;
    }

    let pkg_json_option = match exist_pkg {
        true => {
            let file = File::open(pkg_json_abs_path).expect("Open file fail");
            let reader: BufReader<File> = BufReader::new(file);
            match serde_json::from_reader::<_, PackageJsonStandard>(reader) {
                Ok(pkg_json) => Some(pkg_json),
                Err(_) => {
                    eprintln!("读取项目 package.json 失败");
                    panic!();
                }
            }
        },
        false => None
    };

    let no_pkg_json = pkg_json_option.is_none();

    if no_pkg_json {
        if is_mono {
            return ProjectType::NotSubProject;
        } else {
            return ProjectType::NotNpmProject;
        }
    }

    let pkg_json = pkg_json_option.unwrap();
    if pkg_json.dev_dependencies.contains_key("@modern-js/app-tools") {
        if is_mono {
            return ProjectType::ModernJSSubProject;
        } else {
            return ProjectType::ModernJSProject;
        }
    }

    ProjectType::OtherNpmProject
}