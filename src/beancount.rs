use std::fs;
use zed_extension_api::{
    self as zed, settings::LspSettings, CodeLabel, CodeLabelSpan, LanguageServerId, Result,
};

struct BeancountExtension {
    cached_binary_path: Option<String>,
}

impl Default for BeancountExtension {
    fn default() -> Self {
        Self {
            cached_binary_path: None,
        }
    }
}

#[derive(Clone)]
struct BlsBinary {
    path: String,
    args: Option<Vec<String>>,
    environment: Option<Vec<(String, String)>>,
}

impl BeancountExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<BlsBinary> {
        println!("Finding beancount language server binary");
        let mut args: Option<Vec<String>> = None;

        let (platform, arch) = zed::current_platform();
        println!("Platform: {:?}, Arch: {:?}", platform, arch);
        let environment = match platform {
            zed::Os::Mac | zed::Os::Linux => Some(worktree.shell_env()),
            zed::Os::Windows => None,
        };

        if let Ok(lsp_settings) = LspSettings::for_worktree("beancount-language-server", worktree) {
            println!("Found LSP settings for beancount-language-server");
            if let Some(binary) = lsp_settings.binary {
                println!("Using custom binary path from settings");
                args = binary.arguments;
                if let Some(path) = binary.path {
                    println!("Custom binary path: {}", path);
                    return Ok(BlsBinary {
                        path: path.clone(),
                        args,
                        environment,
                    });
                }
            }
        } else {
            println!("No LSP settings found for beancount-language-server");
        }

        if let Some(path) = worktree.which("beancount-language-server") {
            println!("Found beancount-language-server in PATH: {}", path);
            return Ok(BlsBinary {
                path,
                args,
                environment,
            });
        } else {
            println!("beancount-language-server not found in PATH");
        }

        // if let Some(path) = &self.cached_binary_path {
        //     println!("Checking cached binary path: {}", path);
        //     if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
        //         println!("Using cached binary path: {}", path);
        //         return Ok(BlsBinary {
        //             path: path.clone(),
        //             args,
        //             environment,
        //         });
        //     } else {
        //         warn!("Cached binary path is not valid: {}", path);
        //     }
        // } else {
        //     println!("No cached binary path available");
        // }
        //
        println!("Starting download process for beancount-language-server");
        // zed::set_language_server_installation_status(
        //     language_server_id,
        //     &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        // );

        // Note that in github releases and on zlstools.org the tar.gz asset is not shown
        // but is available at https://builds.zigtools.org/zls-{os}-{arch}-{version}.tar.gz
        let release = zed::latest_github_release(
            "polarmutex/beancount-language-server",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;
        println!("Found release: {}", release.version);

        // let arch: &str = match arch {
        //     zed::Architecture::Aarch64 => "aarch64",
        //     zed::Architecture::X86 => "x86",
        //     zed::Architecture::X8664 => "x86_64",
        // };

        // let os: &str = match platform {
        //     zed::Os::Mac => "macos",
        //     zed::Os::Linux => "linux",
        //     zed::Os::Windows => "windows",
        // };

        // let extension: &str = match platform {
        //     zed::Os::Mac | zed::Os::Linux => "tar.gz",
        //     zed::Os::Windows => "zip",
        // };

        // let asset_name: String = format!("zls-{}-{}-{}.{}", os, arch, release.version, extension);
        // let download_url = format!("https://fixme.org/{}", asset_name);

        let version_dir = format!("beancount-language-server-{}", release.version);
        let binary_path = match platform {
            zed::Os::Mac | zed::Os::Linux => format!("{version_dir}/beanount-language-server"),
            zed::Os::Windows => format!("{version_dir}/beancount-language-server.exe"),
        };

        // if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
        //     zed::set_language_server_installation_status(
        //         language_server_id,
        //         &zed::LanguageServerInstallationStatus::Downloading,
        //     );
        //
        //     zed::download_file(
        //         &download_url,
        //         &version_dir,
        //         match platform {
        //             zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
        //             zed::Os::Windows => zed::DownloadedFileType::Zip,
        //         },
        //     )
        //     .map_err(|e| format!("failed to download file: {e}"))?;
        //
        //     zed::make_file_executable(&binary_path)?;
        //
        //     let entries =
        //         fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
        //     for entry in entries {
        //         let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
        //         if entry.file_name().to_str() != Some(&version_dir) {
        //             fs::remove_dir_all(entry.path()).ok();
        //         }
        //     }
        // }

        // self.cached_binary_path = Some(binary_path.clone());
        Ok(BlsBinary {
            path: binary_path,
            args,
            environment,
        })
    }
}

impl zed::Extension for BeancountExtension {
    fn new() -> Self {
        Self::default()
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        println!("Starting language server command for Beancount extension");
        let zls_binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: zls_binary.path,
            args: zls_binary.args.unwrap_or_default(),
            env: zls_binary.environment.unwrap_or_default(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        println!("init options lang server id: {}", _language_server_id);
        let settings = LspSettings::for_worktree(_language_server_id.as_ref(), _worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
            .unwrap_or_default();
        println!("init options settings: {}", settings);
        Ok(Some(settings))
    }

    // fn label_for_completion(
    //     &self,
    //     _language_server_id: &LanguageServerId,
    //     completion: zed::lsp::Completion,
    // ) -> Option<CodeLabel> {
    //     match completion.kind? {
    //         zed::lsp::CompletionKind::Variable => {
    //             // Account completions with hierarchical styling
    //             if completion.label.contains(':') {
    //                 let parts: Vec<&str> = completion.label.split(':').collect();
    //                 let mut spans = Vec::new();
    //
    //                 for (i, part) in parts.iter().enumerate() {
    //                     if i > 0 {
    //                         spans.push(CodeLabelSpan::literal(
    //                             ":",
    //                             Some("punctuation.separator".into()),
    //                         ));
    //                     }
    //                     spans.push(CodeLabelSpan::literal(*part, Some("type.account".into())));
    //                 }
    //
    //                 Some(CodeLabel {
    //                     spans,
    //                     filter_range: (0..completion.label.len()).into(),
    //                     code: completion.label,
    //                 })
    //             } else {
    //                 None
    //             }
    //         }
    //         zed::lsp::CompletionKind::Constant => {
    //             // Currency and commodity completions
    //             Some(CodeLabel {
    //                 spans: vec![CodeLabelSpan::literal(
    //                     &completion.label,
    //                     Some("constant.builtin.currency".into()),
    //                 )],
    //                 filter_range: (0..completion.label.len()).into(),
    //                 code: completion.label,
    //             })
    //         }
    //         _ => None,
    //     }
    // }
}

zed::register_extension!(BeancountExtension);
