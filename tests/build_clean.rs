use soma::docker;
use soma::docker::{image_exists, image_from_repo_exists};
use soma::ops::{add, build, clean};

pub use self::common::*;

mod common;

#[test]
fn test_build_clean() {
    let (_, mut data_dir) = temp_data_dir();
    let mut env = test_env(&mut data_dir);

    let repo_name = "test-simple-bof";
    let image_name = docker::image_name(repo_name);
    let mut runtime = default_runtime();

    assert!(add(
        &mut env,
        "https://github.com/PLUS-POSTECH/simple-bof.git",
        Some(repo_name),
    )
    .is_ok());

    assert!(build(&env, repo_name, &mut runtime).is_ok());
    let images = runtime.block_on(docker::list_images(&env)).unwrap();
    assert!(image_exists(&images, &image_name));
    assert!(image_from_repo_exists(&images, repo_name));

    assert!(clean(&env, repo_name, &mut runtime).is_ok());
    let images = runtime.block_on(docker::list_images(&env)).unwrap();
    assert!(!image_exists(&images, &image_name));
    assert!(!image_from_repo_exists(&images, repo_name));
}
