use std::path::Path;

use git2::Repository;

pub(crate) fn git_repo_with_single_commit(path: &Path) {
    if !path.exists() {
        std::fs::create_dir_all(path).unwrap();
    }
    std::fs::write(path.join("file.txt"), "").unwrap();
    let repo = git2::Repository::init(path).unwrap();
    // let signature = Signature::now("Name", "name@domain").unwrap();
    // repo.commit(Some("HEAD"), &signature, &signature, "Commit message", repo.)
    //     .unwrap();
    create_initial_commit(&repo).unwrap();
}

fn create_initial_commit(repo: &Repository) -> Result<(), git2::Error> {
    // First use the config to initialize a commit signature for the user.
    let sig = repo.signature()?;

    // Now let's create an empty tree for this commit
    let tree_id = {
        let mut index = repo.index()?;

        // Outside of this example, you could call index.add_path()
        // here to put actual files into the index. For our purposes, we'll
        // leave it empty for now.

        index.write_tree()?
    };

    let tree = repo.find_tree(tree_id)?;

    // Ready to create the initial commit.
    //
    // Normally creating a commit would involve looking up the current HEAD
    // commit and making that be the parent of the initial commit, but here this
    // is the first commit so there will be no parent.
    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;

    Ok(())
}
