use std::{fmt::Display, path::PathBuf};

use crate::utils::config::HHHConfig;

pub enum CreateAction {
    ModernJSProject,
    SubModernJSProject,
}

pub struct InitContext {
    pub is_pnpm: bool,
    pub monorepo_abs_path: PathBuf,
    pub create_action: Option<CreateAction>,
    pub config_exist: Option<bool>,
    pub hhh: Option<HHHConfig>,
    pub links: (Option<String>, Option<String>, Option<String>),
}

impl InitContext {
    pub fn new() -> Self {
        InitContext {
            is_pnpm: false,
            monorepo_abs_path: PathBuf::new(),
            create_action: None,
            config_exist: None,
            hhh: None,
            links: (None, None, None)
        }
    }

    pub fn set_is_pnpm(&mut self, is_pnpm: bool) {
        self.is_pnpm = is_pnpm;
    }

    pub fn set_pnpm_root_path(&mut self, monorepo_abs_path: &PathBuf) {
        self.monorepo_abs_path = monorepo_abs_path.to_path_buf();
    }

    pub fn set_create_action(&mut self, create_action: CreateAction) {
        self.create_action = Some(create_action);
    }

    pub fn set_config_exist(&mut self, exist: bool) {
        self.config_exist = Some(exist);
    }

    pub fn set_hhh_config(&mut self, hhh: HHHConfig) {
        self.hhh = Some(hhh);
    }
}

impl Display for InitContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let is_pnpm = self.is_pnpm;
        let mono_abs_path_str = self.monorepo_abs_path.to_str().unwrap();
        let create_action_value = match self.create_action {
            Some(CreateAction::ModernJSProject) => "modern-js-project",
            Some(CreateAction::SubModernJSProject) => "sub-modern-js-project",
            None => ""
        };
        let config_exist = match self.config_exist {
            Some(true) => true,
            Some(false) => false,
            None => false,
        };
        write!(f, r#"
          {{
            "isPnpm": {is_pnpm},
            "monorepoRoot": "{mono_abs_path_str}",
            "createAction": "{create_action_value}",
            "configExist": "{config_exist}",
          }}
        "#)
    }
}