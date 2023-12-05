use std::path::Path;

use miette::{IntoDiagnostic, Result, WrapErr};

pub fn fetch_models(path: &Path) -> Result<()> {
    let repo = git2::Repository::init_opts(
        ".aws-sdk.git",
        git2::RepositoryInitOptions::new().bare(true),
    )
    .into_diagnostic()
    .wrap_err("creating repository")?;

    repo.remote_set_url("origin", "https://github.com/aws/aws-sdk-js-v3")
        .into_diagnostic()
        .wrap_err("setting origin url")?;

    let mut fetch_callbacks = git2::RemoteCallbacks::new();
    fetch_callbacks.sideband_progress(|data| {
        eprintln!("{}", String::from_utf8_lossy(data));
        true
    });
    fetch_callbacks.transfer_progress({
        let mut last_progress_time: Option<std::time::Instant> = None;
        move |stats| {
            if let Some(time) = last_progress_time {
                if time.elapsed().as_millis() < 100 {
                    return true;
                }
            }
            last_progress_time = Some(std::time::Instant::now());

            const ANSI_MOVE_CURSOR_LINE_START: &str = "\x1b[1G";
            const ANSI_HIDE_CURSOR: &str = "\x1b[?25l";
            const ANSI_CLEAR_TO_END_OF_LINE: &str = "\x1b[K\r";
            const ANSI_SHOW_CURSOR: &str = "\x1b[?25h";
            print!(
                "{}{}fetching: {}/{} objects, {}/{} deltas, {} bytes{}{}",
                ANSI_MOVE_CURSOR_LINE_START,
                ANSI_HIDE_CURSOR,
                stats.received_objects(),
                stats.total_objects(),
                stats.indexed_deltas(),
                stats.total_deltas(),
                stats.received_bytes(),
                ANSI_SHOW_CURSOR,
                ANSI_CLEAR_TO_END_OF_LINE,
            );
            true
        }
    });

    repo.find_remote("origin")
        .into_diagnostic()
        .wrap_err("finding origin remote")?
        .fetch(
            &["main"],
            Some(
                &mut git2::FetchOptions::new()
                    .depth(0)
                    .remote_callbacks(fetch_callbacks),
            ),
            None,
        )
        .into_diagnostic()
        .wrap_err("fetching remote")?;

    let origin_main = repo
        .find_reference("refs/remotes/origin/main")
        .into_diagnostic()
        .wrap_err("finding origin/main reference")?;
    let root_tree = origin_main
        .peel_to_tree()
        .into_diagnostic()
        .wrap_err("peeling origin/main reference to tree")?;
    let models_tree = root_tree
        .get_path(Path::new("codegen/sdk-codegen/aws-models"))
        .into_diagnostic()
        .wrap_err("finding codegen models tree in origin/main root tree")?;

    repo.checkout_tree(
        &models_tree
            .to_object(&repo)
            .into_diagnostic()
            .wrap_err("converting models tree to object")?,
        Some(git2::build::CheckoutBuilder::new().force().target_dir(path)),
    )
    .into_diagnostic()
    .wrap_err("checking out model tree")?;
    Ok(())
}
