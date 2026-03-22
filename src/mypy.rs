use std::{env, fs};

use zed_extension_api::{self as zed, serde_json, settings::LspSettings};

struct MypyLSP {
    mypy_path: String,
    mypy_args: Vec<String>,
}

struct MypyExtension {}

impl MypyExtension {
    fn get_mypy_lsp(
        &mut self,
        language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> MypyLSP {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|settings| settings.settings);

        let mypy_path = settings
            .as_ref()
            .and_then(|s| s.get("path"))
            .and_then(|v| v.as_str())
            .map(String::from)
            .or_else(|| worktree.which("mypy"));
        let mypy_args: Vec<String> = settings
            .as_ref()
            .and_then(|s| s.get("args"))
            .and_then(|a| a.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        if mypy_path.is_none() {
            panic!(
                "mypy executable was not found, did you set the path of the mypy executable in your settings.json?"
            )
        }

        MypyLSP {
            mypy_path: mypy_path.unwrap(),
            mypy_args,
        }
    }
}

impl zed::Extension for MypyExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        let mypy_lsp = self.get_mypy_lsp(language_server_id, worktree);

        let server_code = include_str!("../lsp/server.py");
        fs::write("server.py", server_code)
            .map_err(|e| format!("Cannot write server.py: {}", e))?;
        let requirements = include_str!("../lsp/requirements.txt");
        fs::write("requirements.txt", requirements)
            .map_err(|e| format!("Cannot write requirements.txt: {}", e))?;
        let init = include_str!("../lsp/init.sh");
        fs::write("init.sh", init).map_err(|e| format!("Cannot write init.sh: {}", e))?;
        let make_exec_result = zed::make_file_executable("init.sh");
        if make_exec_result.is_err() {
            panic!("Cannot make init.sh executable, do you have permission to do that?")
        }

        let current_dir = env::current_dir();
        if current_dir.is_err() {
            panic!("Cannot get current directory")
        }
        let project_root = worktree.root_path();
        Ok(zed::Command {
            command: "init.sh".to_string(),
            args: vec![
                current_dir.unwrap().to_string_lossy().to_string(),
                project_root,
                mypy_lsp.mypy_path,
                serde_json::to_string(&mypy_lsp.mypy_args).unwrap(),
            ],
            env: vec![],
        })
    }
}

zed::register_extension!(MypyExtension);
