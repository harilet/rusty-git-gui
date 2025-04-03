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
    branches: Branches,
}

#[derive(Clone, serde::Serialize)]
struct Branches {
    local: Vec<String>,
    remote: Vec<String>,
}

#[derive(Clone, serde::Serialize, Debug)]
struct RemoteDetails {
    name: String,
    url: String,
}

#[derive(Clone, serde::Serialize, Debug)]
struct ChangeLine {
    from_no: String,
    to_no: String,
    content: String,
    change_type: String,
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
            Err(_e) => {
                app.emit("clone-complete", "failure").unwrap();
            }
        }
    });
}

#[tauri::command]
fn open_repo_window(app: AppHandle, repo_location: String) {
    save_project(repo_location.clone());
    let mut url = "/repo?location=".to_string();
    url.push_str(&repo_location);
    let webview_url = tauri::WebviewUrl::App(url.into());
    thread::spawn(move || {
        tauri::WebviewWindowBuilder::new(&app, "second", webview_url)
            .title("Second")
            .decorations(false)
            .transparent(true)
            .inner_size(1092.0, 790.0)
            .effects(EffectsBuilder::new().effect(Effect::Mica).build())
            .build()
            .unwrap();
    });
}

#[tauri::command]
fn get_branches(repo_location: String) -> BranchesPayload {
    let mut branch_list: Branches = Branches {
        local: Vec::new(),
        remote: Vec::new(),
    };
    let repo = Repository::open(repo_location).unwrap();

    let deafult_branch = get_current_branch_name(&repo);

    let branches = repo.branches(None).unwrap();
    branches.for_each(|branch| {
        for bran in branch.iter() {
            if bran.1 == BranchType::Local {
                branch_list
                    .local
                    .push(bran.0.name().unwrap().unwrap().to_string());
            } else if bran.1 == BranchType::Remote {
                branch_list
                    .remote
                    .push(bran.0.name().unwrap().unwrap().to_string());
            }
        }
    });
    BranchesPayload {
        default: deafult_branch,
        branches: branch_list,
    }
}

#[tauri::command]
fn fetch_all(repo_location: String) {
    let repo = Repository::open(repo_location).unwrap();
    let mut origin = repo.find_remote("origin").unwrap();
    origin
        .fetch(&["refs/heads/*:refs/remotes/origin/*"], None, None)
        .unwrap();
}

#[tauri::command]
fn get_commit(repo_location: String, branch_name: String) -> Vec<[std::string::String; 5]> {
    let repo = Repository::open(repo_location).unwrap();
    let branch_name_split: Vec<_> = branch_name.split('/').collect();
    if branch_name_split[0] == "origin" {
        [].to_vec()
    } else {
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

    let mut diff_data: Vec<ChangeLine> = vec![];

    repo.diff_tree_to_tree(Some(&parent_tree), Some(&tree), Some(&mut opts))
        .unwrap()
        .print(DiffFormat::Patch, |_d, _h, l| {
            diff_data.push(format_change_line_item(l, _d));
            true
        })
        .unwrap();
    app.emit("changes", diff_data.clone()).unwrap();
}

#[tauri::command]
fn checkout_branch(repo_location: String, branch_name: String) {
    let repo = Repository::open(repo_location).unwrap();
    let branch_name_split: Vec<_> = branch_name.split('/').collect();
    if branch_name_split[0] == "origin" {
        checkout_remote_branch(&repo, "origin", &branch_name_split[1]).unwrap();
    } else {
        let (obje, refr) = repo.revparse_ext(&branch_name).unwrap();

        repo.checkout_tree(&obje, None).unwrap();
        match refr {
            Some(gref) => {
                repo.set_head(gref.name().unwrap()).unwrap();
            }
            None => {}
        }
    }
}

fn checkout_remote_branch(
    repo: &Repository,
    remote: &str,
    branch: &str,
) -> Result<(), git2::Error> {
    // Build the remote branch reference name, e.g., "refs/remotes/origin/mybranch"
    let remote_ref = format!("refs/remotes/{}/{}", remote, branch);
    // Find the remote branch reference
    let reference = repo.find_reference(&remote_ref)?;
    // Get the commit that the remote branch points to
    let commit = repo.find_commit(reference.target().expect("No target found"))?;
    // Create a new local branch named `branch` pointing at that commit.
    // The `false` means do not force creation if it already exists.
    repo.branch(branch, &commit, false)?;
    // Set HEAD to the new local branch and update the working directory.
    let local_ref = format!("refs/heads/{}", branch);
    repo.set_head(&local_ref)?;
    repo.checkout_head(None)?;
    Ok(())
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

    let mut diff_data: Vec<ChangeLine> = vec![];

    repo.diff_index_to_workdir(Some(&repo.index().unwrap()), Some(&mut diff_opts))
        .unwrap()
        .print(DiffFormat::Patch, |_d, _h, l| {
            diff_data.push(format_change_line_item(l, _d));
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

    let mut diff_data: Vec<ChangeLine> = vec![];
    repo.diff_tree_to_index(
        Some(&old_tree),
        Some(&repo.index().unwrap()),
        Some(&mut diff_opts),
    )
    .unwrap()
    .print(DiffFormat::Patch, |_d, _h, l| {
        diff_data.push(format_change_line_item(l, _d));
        true
    })
    .unwrap();
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
fn remove_file(repo_location: String, path: String) {
    let repo = Repository::open(repo_location).unwrap();
    let mut index = repo.index().unwrap();
    index.remove_path(Path::new(&path)).unwrap();
    index.write().unwrap();
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
#[tauri::command]
fn new_branch(
    repo_location: String,
    from_branch_name: String,
    new_branch_name: String,
    force: bool,
) {
    let repo = Repository::open(repo_location).unwrap();
    let current_branch = repo
        .find_branch(&from_branch_name, BranchType::Local)
        .unwrap();

    repo.branch(
        &new_branch_name,
        &current_branch.get().peel_to_commit().unwrap(),
        force,
    )
    .unwrap();
}

#[tauri::command]
fn delete_branch(repo_location: String, branch_name: String) {
    let repo = Repository::open(repo_location).unwrap();
    let mut branch = repo.find_branch(&branch_name, BranchType::Local).unwrap();
    branch.delete().unwrap();
}

#[tauri::command]
fn rename_branch(repo_location: String, branch_name: String, new_branch_name: String, force: bool) {
    let repo = Repository::open(repo_location).unwrap();
    let mut branch = repo.find_branch(&branch_name, BranchType::Local).unwrap();
    branch.rename(&new_branch_name, force).unwrap();
}

#[tauri::command]
fn get_config(repo_location: String, key_name: String) -> String {
    let repo = Repository::open(repo_location).unwrap();
    match repo.config().unwrap().get_string(&key_name) {
        Ok(data) => return data,
        Err(_) => return "".to_string(),
    }
}

#[tauri::command]
fn get_remotes(repo_location: String) -> Vec<RemoteDetails> {
    let repo = Repository::open(repo_location).unwrap();

    let mut remotes_details: Vec<RemoteDetails> = vec![];

    repo.remotes().unwrap().iter().for_each(|remote_name| {
        let name = remote_name.unwrap();
        if let Some(remote) = repo.find_remote(name).ok() {
            let remote_name = remote.name().unwrap();
            let remote_url = remote.url().unwrap();
            remotes_details.push(RemoteDetails {
                name: remote_name.to_string(),
                url: remote_url.to_string(),
            });
        }
    });

    remotes_details
}

#[tauri::command]
fn add_remote(repo_location: String, remote_name: String, remote_url: String) {
    let repo = Repository::open(repo_location).unwrap();
    let _remote = repo.remote(&remote_name, &remote_url).unwrap();
}

#[tauri::command]
fn get_remote_local_drift(repo_location: String, branch_name: String) -> (usize, usize) {
    let repo = Repository::open(repo_location).unwrap();

    let head = repo.revparse_single(&branch_name).unwrap().id();

    let temp = repo.revparse_ext("@{u}");
    return match temp {
        Ok(data) => {
            let (upstream, _) = data;
            return match repo.graph_ahead_behind(head, upstream.id()) {
                Ok((commits_ahead, commits_behind)) => {
                    println!("{:?}", commits_behind);
                    (commits_ahead, commits_behind)
                }
                Err(e) => {
                    println!("{:#?}", e);
                    return (0, 0);
                }
            };
        }
        Err(e) => {
            println!("{:#?}", e);
            (0, 0)
        }
    };
}

#[tauri::command]
fn push_to_remote(app: AppHandle, repo_location: String, branch_name: String, remote: String) {

    error_emitter(app, "push-failed to push to remote: {}".to_string());
    return;
    let repo = Repository::open(repo_location).unwrap();
    let mut origin = repo.find_remote(&remote).unwrap();

    let branch = repo.find_branch(&branch_name, BranchType::Local).unwrap();

    let branch_ref = branch.into_reference();
    let branch_ref_name = branch_ref.name().unwrap();
    repo.set_head(branch_ref_name).unwrap();

    let mut remote_callbacks = git2::RemoteCallbacks::new();
    remote_callbacks.credentials(|_url, _username_from_url, _allowed_types| {
        git2::Cred::ssh_key_from_agent(_username_from_url.unwrap())
    });

    remote_callbacks.certificate_check(|_str1, str2| {
        println!("certificate check");
        println!("{:#?}", str2);
        Ok(git2::CertificateCheckStatus::CertificateOk)
    });

    let mut push_options = git2::PushOptions::new();
    let mut_push_options = push_options.remote_callbacks(remote_callbacks);

    match origin.push(&[branch_ref_name], Some(mut_push_options)) {
        Ok(_) => {
            println!("push-pushed to remote")
        }
        Err(e) => {
            error_emitter(app, format!("push-failed to push to remote: {}", e))
        }
    };
}

#[tauri::command]
fn pull_from_remote(repo_location: String, branch_name: String, remote: String) {
    let repo = Repository::open(repo_location).unwrap();
    let mut origin = repo.find_remote(&remote).unwrap();
    let branch = repo.find_branch(&branch_name, BranchType::Local).unwrap();

    let mut remote_callbacks = git2::RemoteCallbacks::new();
    remote_callbacks.credentials(|_url, _username_from_url, _allowed_types| {
        println!(
            "url: {:#?}\nusername: {:#?}:\ntype: {:#?}",
            _url,
            _username_from_url.unwrap(),
            _allowed_types
        );

        git2::Cred::ssh_key_from_agent(_username_from_url.unwrap())
    });

    let mut fetch_options = git2::FetchOptions::new();
    let mut_fetch_options = fetch_options.remote_callbacks(remote_callbacks);

    let branch_ref = branch.into_reference();
    let branch_ref_name = branch_ref.name().unwrap();
    origin
        .fetch(&[branch_ref_name], Some(mut_fetch_options), None)
        .unwrap();
    repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
        .unwrap();
}

#[tauri::command]
fn create_repo_window(repo_location: String) {
    Repository::init(std::path::PathBuf::from(repo_location)).unwrap();
    return;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let exe_path = get_exe_dir();
    let project_list_path = exe_path.join("config\\project-list.json");
    if !(project_list_path.exists()) {
        fs::create_dir(exe_path.join("config")).unwrap();
        File::create(project_list_path).unwrap();
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
            make_commit,
            new_branch,
            delete_branch,
            rename_branch,
            remove_file,
            get_projects_list,
            fetch_all,
            get_config,
            get_remotes,
            add_remote,
            push_to_remote,
            create_repo_window,
            get_remote_local_drift,
            pull_from_remote,
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

fn save_project(path: String) {
    let exe_path: PathBuf = get_exe_dir();
    let project_list_path = exe_path.join("config\\project-list.json");

    let file_data = fs::read(project_list_path.clone()).unwrap();
    let mut data: Vec<String> = serde_json::from_slice(&file_data[..]).unwrap();
    data = data
        .into_iter()
        .filter(|x| x.as_str() != path.as_str())
        .collect();
    data.push(path);
    fs::write(project_list_path, serde_json::to_string(&data).unwrap()).unwrap();
}

#[tauri::command]
fn get_projects_list() -> Vec<String> {
    let exe_path = get_exe_dir();
    let project_list_path = exe_path.join("config\\project-list.json");
    let file_data = fs::read(project_list_path).unwrap();
    let data: Vec<String> = serde_json::from_slice(&file_data[..]).unwrap();
    data
}

fn format_change_line_item(l: git2::DiffLine, d: git2::DiffDelta) -> ChangeLine {
    let mut temp_data = ChangeLine {
        content: "".to_string(),
        to_no: "".to_string(),
        from_no: "".to_string(),
        change_type: "".to_string(),
    };

    let mut content = str::from_utf8(l.content()).unwrap().to_string();

    temp_data.change_type = l.origin().to_string();

    match l.old_lineno() {
        Some(num) => {
            temp_data.from_no = format!("{}", num);
        }
        None => {
            temp_data.from_no = format!("");
        }
    }

    match l.new_lineno() {
        Some(num) => {
            temp_data.to_no = format!("{}", num);
        }
        None => {
            temp_data.to_no = format!("",);
        }
    }

    if temp_data.change_type == "F" {
        match d.status() {
            git2::Delta::Modified => {
                temp_data.change_type = "M".to_owned();
            }
            git2::Delta::Added => {
                temp_data.change_type = "A".to_owned();
            }
            git2::Delta::Deleted => {
                temp_data.change_type = "D".to_owned();
            }
            git2::Delta::Renamed => {
                temp_data.change_type = "R".to_owned();
            }
            _ => {}
        };
        content = d.new_file().path().unwrap().to_str().unwrap().to_string();
    }

    temp_data.content = content;
    temp_data
}

fn error_emitter(app: AppHandle, message:String){
    app.emit("error", message).unwrap();
}