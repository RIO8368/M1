use util::{
    artifact::Artifact,
    util::util::patterns::constructor::ConstructorOperations,
    util::util::version
};
use super::{
    testnet_id,
    subnet
};

#[derive(Debug, Clone)]
pub struct Config;

#[derive(Debug, Clone)]
pub struct Constructor;

impl ConstructorOperations for Constructor {

    type Artifact = Artifact;
    type Config = Config;

    fn default() -> Self::Artifact {

        Self::default_with_version(&version::Version::Latest)

    }

    fn default_with_version(version : &util::util::util::Version) -> Self::Artifact {
        // source should have the same version
        let subnet = subnet::Constructor::default_with_version(version);
        let testnet_id = testnet_id::Constructor::default_with_version(version);

        Artifact::self_contained_script(
            "subnet".to_string(),
            r#"
            echo $MOVEMENT_DIR
            cp $MOVEMENT_DIR/bin/subnet $MOVEMENT_DIR/bin/$(cat $MOVEMENT_DIR/rsc/testnet-id)
            "#.to_string(),
        ).with_dependencies(vec![
            testnet_id.into(),
            subnet.into(),
        ].into_iter().collect())

    }

    fn from_config(_ : &Self::Config) -> Self::Artifact {
        Self::default()
    }

}


#[cfg(test)]
pub mod test {

    use super::*;
    use util::movement_dir::MovementDir;

    #[cfg(target_os = "macos")]
    #[tokio::test]
    async fn test_curl_macos() -> Result<(), anyhow::Error> {
        
        let temp_home = tempfile::tempdir()?;   
    
        let dir = temp_home.path().to_path_buf();
        let movement_dir = MovementDir::new(&dir);
        let artifact = Constructor::default();

        artifact.install(&movement_dir).await?;

        let exists = match std::process::Command::new("curl").arg("--version").output() {
            Ok(output) => output.status.success(),
            Err(_) => false,
        };

        assert!(exists);

        Ok(())

    }

}