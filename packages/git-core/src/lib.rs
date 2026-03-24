use git2::Repository;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// Custom error type — thiserror generates the boilerplate Display impl for us
#[derive(Debug, Error)]
pub enum GitError {
    #[error("git2 error: {0}")]
    Git2(#[from] git2::Error),
    #[error("not a git repository: {0}")]
    NotARepo(String),
}

// This struct will be serialized to JSON and sent to the frontend
#[derive(Debug, Serialize, Deserialize)]
pub struct FileStatus {
    pub path: String,
    pub status: String,
}

/// Returns the working tree status of a repository.
/// `repo_path` is the absolute path to the git repo on disk.
pub fn get_status(repo_path: &str) -> Result<Vec<FileStatus>, GitError> {
    // Open the repo — returns GitError::NotARepo if the path isn't a git repo
    let repo = Repository::open(repo_path)
        .map_err(|_| GitError::NotARepo(repo_path.to_string()))?;

    let statuses = repo.statuses(None)?;

    let files = statuses
        .iter()
        .filter_map(|entry| {
            let path = entry.path()?.to_string();
            let label = match entry.status() {
                s if s.contains(git2::Status::WT_NEW) => "untracked",
                s if s.contains(git2::Status::INDEX_NEW) => "added",
                s if s.contains(git2::Status::INDEX_MODIFIED) | s.contains(git2::Status::WT_MODIFIED) => "modified",
                s if s.contains(git2::Status::INDEX_DELETED) | s.contains(git2::Status::WT_DELETED) => "deleted",
                _ => "other",
            };
            Some(FileStatus { path, status: label.to_string() })
        })
        .collect();

    Ok(files)
}
