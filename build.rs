use std::{
    fs::{
        File
    },
    io::{
        BufWriter,
        Read,
        Write
    },
    path::{
        PathBuf,
        Path
    }
};

use git2::{
    Repository,
    DescribeOptions
};

fn main() -> Result<(), Error> {
    // Get the build directory from the environment arguments
    let build_path: PathBuf = std::env::var_os("OUT_DIR")
        .ok_or(Error::InvalidBuildPath)?.into();

    // Make sure the build directory exists
    std::fs::create_dir_all(&build_path)
        .or(Err(Error::CreateDirectory))?;

    // Write the date to the date file
    write_to_file(build_path.join("date.txt"), chrono::Utc::now().format("%Y-%m-%d"))?;

    // Get the repository object for the current folder
    let repository = Repository::discover(".")
        .or(Err(Error::MissingRepository))?;

    // Get the current description of the repository
    let description = repository.describe(DescribeOptions::new()
            .describe_tags()
            .show_commit_oid_as_fallback(true))
        .or(Err(Error::RepositoryDescription))?;

    // Format the description to a string
    let commit = description.format(None).or(Err(Error::CommitFormatError))?;

    // Write the commit hash to the commit hash file
    write_to_file(build_path.join("commit.txt"), commit)?;

    Ok(())
}

fn write_to_file<P: AsRef<Path>, S: ToString>(path: P, content: S) -> Result<(), Error> {

    let content: String = content.to_string();

    let current = if path.as_ref().exists() {
        // Open the file
        let mut file = File::open(&path)
            .or(Err(Error::InvalidFile))?;

        let mut current_content = String::new();
        // Read it's contents
        file.read_to_string(&mut current_content)
            .or(Err(Error::FileRead))?;

        // And set the variable equal to whether or not
        // the file already has the right information
        current_content == content

    } else {
        false
    };

    // If the current content in the file is not up to date
    if !current {
        // Overwrite the file
        let mut file = BufWriter::new(
            File::create(&path)
                .or(Err(Error::InvalidFile))?
        );

        // With the correct information
        file.write_all(content.as_bytes())
            .or(Err(Error::FileWrite))?
    }

    Ok(())
}

#[derive(Debug)]
enum Error {
    InvalidBuildPath,
    CreateDirectory,
    InvalidFile,
    FileRead,
    FileWrite,
    MissingRepository,
    RepositoryDescription,
    CommitFormatError
}