use ::prelude::*;

pub struct Platform;

pub trait PlatformInterface {
  /// Set the execute permission on the file pointed to by `path`
  fn set_execute_permission(path: &Path) -> Result<(), io::Error>;
}

#[cfg(unix)]
impl PlatformInterface for Platform {
  fn set_execute_permission(path: &Path) -> Result<(), io::Error> {
    use std::os::unix::fs::PermissionsExt;

    // get current permissions
    let mut permissions = fs::metadata(&path)?.permissions();

    // set the execute bit
    let current_mode = permissions.mode();
    permissions.set_mode(current_mode | 0o100);

    // set the new permissions
    fs::set_permissions(&path, permissions)
  }
}

#[cfg(windows)]
impl PlatformInterface for Platform {
  fn set_execute_permission(path: &Path) -> Result<(), io::Error> {
    // it is not necessary to set an execute permission on a script on windows,
    // so this is a nop
    Ok(())
  }
}
