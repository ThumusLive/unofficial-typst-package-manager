use owo_colors::OwoColorize;
use std::fs;
use typst_project::manifest::Manifest;

use crate::utils::{
    copy_dir_all,
    paths::{c_packages, check_path_dir, d_packages, get_current_dir},
    specs::Extra,
    state::{Error, ErrorKind, Result},
    symlink_all,
};

use super::LinkArgs;

pub fn run(cmd: &LinkArgs, path: Option<String>, pt: bool) -> Result<bool> {
    let curr = path.unwrap_or(get_current_dir()?);

    let config = match Manifest::try_find(&curr)? {
        Some(val) => Ok(val),
        None => Err(Error::empty(ErrorKind::Manifest)),
    }?;
    let namespace = if let Some(value) = config.tool {
        value
            .get_section("utpm")?
            .unwrap_or(Extra::default())
            .namespace
            .unwrap_or("local".into())
    } else {
        "local".into()
    };

    let name = config.package.name;
    let version = config.package.version;
    let path = if namespace != "preview" {
        format!("{}/{}/{}/{}", d_packages(), namespace, name, version)
    } else {
        format!("{}/{}/{}/{}", c_packages()?, namespace, name, version)
    };
    let binding = "Info:".yellow();
    let info = binding.bold();
    if check_path_dir(&path) && !cmd.force {
        return Err(Error::empty(ErrorKind::AlreadyExist(
            name.into(),
            version,
            format!("{}", info),
        )));
    }

    fs::create_dir_all(&path)?;

    if cmd.force {
        fs::remove_dir_all(&path)?
    }

    if cmd.no_copy {
        symlink_all(&curr, &path)?;
        if pt {
            println!(
                "Project linked to: {} \nTry importing with:\n #import \"@{}/{}:{}\": *",
                path, namespace, name, version
            );
        }
    } else {
        copy_dir_all(&curr, &path)?;
        if pt {
            println!(
                "Project copied to: {} \nTry importing with:\n #import \"@{}/{}:{}\": *",
                path, namespace, name, version
            );
        }
    }
    Ok(true)
}
