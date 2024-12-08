use core::str;
use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{BranchType, DiffFormat, DiffOptions, ObjectType, Repository};
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
fn get_commit(repo_location: String, branch_name: String) -> Vec<[std::string::String; 5]> {
    let repo = Repository::open(repo_location).unwrap();
    let branch = repo.find_branch(&branch_name, BranchType::Local).unwrap();

    let mut walk = repo.revwalk().unwrap();
    walk.push(branch.get().peel(ObjectType::Commit).unwrap().id())
        .unwrap();
    walk.set_sorting(git2::Sort::TIME).unwrap();

    let mut commit_list = vec![];
    for rev in walk.take(30) {
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

#[tauri::command]
fn get_file_changes(app: AppHandle, repo_location: String, commit_id: String) {
    let repo = Repository::open(repo_location).unwrap();

    let mut opts = git2::DiffOptions::new();
    opts.force_text(true)
        .ignore_whitespace_eol(false)
        .ignore_whitespace_change(false)
        .ignore_whitespace(false)
        .include_ignored(false)
        .include_untracked(false)
        .patience(true)
        .minimal(true);

    let commit = repo
        .find_commit(git2::Oid::from_str(&commit_id).unwrap())
        .unwrap();
    let tree = commit.tree().unwrap();
    let parent_tree = repo
        .find_commit(commit.parent(0).unwrap().id())
        .unwrap()
        .tree()
        .unwrap();

    let mut diff_data: String = "".to_string();
    repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), Some(&mut opts))
        .unwrap()
        .print(DiffFormat::Patch, |_d, _h, l| {
            match l.origin() {
                '+' | '-' | ' ' => {
                    diff_data.push(l.origin());
                }
                _ => {}
            }
            diff_data.push_str(str::from_utf8(l.content()).unwrap());
            true
        })
        .unwrap();
    app.emit("changes", diff_data.clone()).unwrap();
}

#[tauri::command]
fn checkout_branch(repo_location: String, branch_name: String) {
    let repo = Repository::open(repo_location).unwrap();
    let (obje, refr) = repo.revparse_ext(&branch_name).unwrap();
    repo.checkout_tree(&obje, None).unwrap();
    match refr {
        Some(gref) => {
            repo.set_head(gref.name().unwrap()).unwrap();
        }
        None => {}
    }
}

#[tauri::command]
fn get_unstaged_files(repo_location: String) -> Vec<String> {
    let repo = Repository::open(repo_location).unwrap();
    let mut path_list = vec![];
    let mut diff_opts = DiffOptions::new();
    diff_opts
        .patience(true)
        .minimal(true)
        .include_ignored(false)
        .include_untracked(true)
        .ignore_whitespace_eol(false);
    let unstaged_diff = repo
        .diff_index_to_workdir(Some(&repo.index().unwrap()), Some(&mut diff_opts))
        .unwrap();
    for diff in unstaged_diff.deltas().into_iter() {
        path_list.push(
            diff.new_file()
                .path()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );
    }
    path_list
}

#[tauri::command]
fn get_staged_files(repo_location: String) -> Vec<String> {
    let repo = Repository::open(repo_location).unwrap();
    let mut path_list = vec![];
    let mut diff_opts = DiffOptions::new();
    let old_tree = repo.head().unwrap().peel_to_tree().unwrap();

    let staged_diff = repo
        .diff_tree_to_index(
            Some(&old_tree),
            Some(&repo.index().unwrap()),
            Some(&mut diff_opts),
        )
        .unwrap();

    for diff in staged_diff.deltas().into_iter() {
        path_list.push(
            diff.new_file()
                .path()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );
    }
    path_list
}

#[tauri::command]
fn get_unstaged_changes(app: AppHandle, repo_location: String, path: String) {
    let repo = Repository::open(repo_location).unwrap();

    let mut diff_opts = DiffOptions::new();
    diff_opts
        .patience(false)
        .minimal(false)
        .include_ignored(false)
        .include_untracked(false)
        .ignore_whitespace_eol(false)
        .pathspec(path.clone());

    let mut diff_data: String = "".to_string();

    repo.diff_index_to_workdir(Some(&repo.index().unwrap()), Some(&mut diff_opts))
        .unwrap()
        .print(DiffFormat::Patch, |_d, _h, l| {
            match l.origin() {
                '+' | '-' | ' ' => {
                    diff_data.push(l.origin());
                }
                _ => {}
            }

            match l.old_lineno() {
                Some(num) => {
                    diff_data.push_str(&format!(" {}", num));
                }
                None => {
                    diff_data.push_str(&format!(" _"));
                }
            }

            match l.new_lineno() {
                Some(num) => {
                    diff_data.push_str(&format!(" {}", num));
                }
                None => {
                    diff_data.push_str(&format!(" _"));
                }
            }
            diff_data.push_str(str::from_utf8(l.content()).unwrap());
            true
        })
        .unwrap();
    app.emit("changes", diff_data.clone()).unwrap();
}

#[tauri::command]
fn get_staged_changes(app: AppHandle, repo_location: String, path: String) {
    let repo = Repository::open(repo_location).unwrap();

    let mut diff_opts = DiffOptions::new();
    diff_opts
        .patience(true)
        .minimal(true)
        .include_ignored(false)
        .include_untracked(false)
        .ignore_whitespace_eol(false)
        .pathspec(path.clone());
    let old_tree = repo.head().unwrap().peel_to_tree().unwrap();

    let mut diff_data: String = "".to_string();
    repo.diff_tree_to_index(
        Some(&old_tree),
        Some(&repo.index().unwrap()),
        Some(&mut diff_opts),
    )
    .unwrap()
    .print(DiffFormat::Patch, |_d, _h, l| {
        match l.origin() {
            '+' | '-' | ' ' => {
                diff_data.push(l.origin());
            }
            _ => {}
        }

        match l.old_lineno() {
            Some(num) => {
                diff_data.push_str(&format!(" {}", num));
            }
            None => {
                diff_data.push_str(&format!(" _"));
            }
        }

        match l.new_lineno() {
            Some(num) => {
                diff_data.push_str(&format!(" {} ", num));
            }
            None => {
                diff_data.push_str(&format!(" _ "));
            }
        }
        diff_data.push_str(str::from_utf8(l.content()).unwrap());
        true
    })
    .unwrap();
    println!("{}",diff_data);
    app.emit("changes", diff_data.clone()).unwrap();
}

#[tauri::command]
fn add_file_index(repo_location: String, path: String) {
    let repo = Repository::open(repo_location).unwrap();
    let mut index = repo.index().unwrap();
    index.add_path(Path::new(&path)).unwrap();
    index.write().unwrap();
    return;
}

#[tauri::command]
fn remove_file_index(repo_location: String, path: String) {
    let repo = Repository::open(repo_location).unwrap();
    let head = repo.head().unwrap();
    let commit = head.peel_to_commit().unwrap();
    repo.reset_default(Some(commit.as_object()), &[path])
        .unwrap();
    return;
}

#[tauri::command]
fn make_commit(repo_location: String, message: String) {
    let repo = Repository::open(repo_location).unwrap();
    let mut index = repo.index().unwrap();
    let oid = index.write_tree().unwrap();
    let signature = repo.signature().unwrap();
    let parent_commit = repo.head().unwrap().peel_to_commit().unwrap();
    let tree = repo.find_tree(oid).unwrap();
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &message,
        &tree,
        &[&parent_commit],
    )
    .unwrap();
    return;
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
            get_commit,
            get_file_changes,
            checkout_branch,
            get_unstaged_files,
            get_staged_files,
            get_unstaged_changes,
            get_staged_changes,
            add_file_index,
            remove_file_index,
            make_commit
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
