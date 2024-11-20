use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{BranchType, Repository};
use std::path::Path;
use std::thread::{self};
use std::{env, fs};
use std::{fs::File, path::PathBuf};
use tauri::{
    window::{Effect, EffectsBuilder},
    Manager,
};
use tauri::{AppHandle, Emitter};

#[derive(Clone, serde::Serialize)]
struct BranchesPayload {
    default: String,
    branches: Vec<[String; 2]>,
}

#[tauri::command]
fn clone_repo(app: AppHandle, repo_url: String, repo_location: String) {
    thread::spawn(move || {
        let mut co = CheckoutBuilder::new();
        co.progress(|_, cur, total| {
            if cur == total {
                app.emit("clone-complete", "success").unwrap();
            }
        });
        let result = RepoBuilder::new()
            .with_checkout(co)
            .clone(&repo_url, Path::new(&repo_location));
        match result {
            Ok(_) => {}
            Err(_) => {
                app.emit("clone-complete", "failure").unwrap();
            }
        }
    });
}

#[tauri::command]
fn open_repo_window(app: AppHandle, repo_location: String) {
    let mut url = "/repo?location=".to_string();
    url.push_str(&repo_location);
    let webview_url = tauri::WebviewUrl::App(url.into());
    thread::spawn(move || {
        tauri::WebviewWindowBuilder::new(&app, "second", webview_url)
            .title("Second")
            .decorations(false)
            .transparent(true)
            .effects(EffectsBuilder::new().effect(Effect::Mica).build())
            .build()
            .unwrap();
    });
}

#[tauri::command]
fn get_branches(repo_location: String) -> BranchesPayload {
    let repo = Repository::open(repo_location).unwrap();
    let deafult_branch = get_current_branch_name(&repo);

    let branches = repo.branches(None).unwrap();
    let mut branch_list = vec![];
    branches.for_each(|branch| {
        for bran in branch.iter() {
            if bran.1 == BranchType::Local {
                branch_list.push([
                    bran.0.name().unwrap().unwrap().to_string(),
                    "Local".to_string(),
                ]);
            } else if bran.1 == BranchType::Remote {
                branch_list.push([
                    bran.0.name().unwrap().unwrap().to_string(),
                    "Remote".to_string(),
                ]);
            }
        }
    });
    BranchesPayload {
        default: deafult_branch,
        branches: branch_list,
    }
}

#[tauri::command]
fn get_commit(repo_location: String) -> Vec<[std::string::String; 5]> {
    let repo = Repository::open(repo_location).unwrap();
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();

    revwalk.set_sorting(git2::Sort::TIME).unwrap();

    let mut commit_list = vec![];

    for rev in revwalk {
        let commit = repo.find_commit(rev.unwrap()).unwrap();
        commit_list.push([
            commit.id().to_string(),
            commit.committer().name().unwrap().to_string(),
            commit.committer().email().unwrap().to_string(),
            commit.message().unwrap().to_string(),
            commit.summary().unwrap().to_string(),
        ]);
    }

    commit_list
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let exe_path = get_exe_dir();
    let project_list_path = exe_path.join("config\\project-list.json");
    if !(project_list_path.exists()) {
        let _ = fs::create_dir(exe_path.join("config"));
        let _ = File::create(project_list_path);
    }
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            clone_repo,
            open_repo_window,
            get_branches,
            get_commit
        ])
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window
                .set_effects(EffectsBuilder::new().effect(Effect::Mica).build())
                .unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn get_exe_dir() -> PathBuf {
    let exe_path = env::current_exe().expect("Failed to get current executable path");
    return exe_path
        .parent()
        .expect("Failed to get executable directory")
        .to_path_buf();
}

fn get_current_branch_name(repo: &Repository) -> String {
    let head = repo.head().unwrap();
    head.shorthand().unwrap().to_string()
}
