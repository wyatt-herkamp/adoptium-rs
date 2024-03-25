use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SysConfig {
    pub install_method: InstallMethod,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "values")]
pub enum InstallMethod {
    UpdateAlternatives(UpdateAlternatives),
}

impl Default for InstallMethod {
    fn default() -> Self {
        InstallMethod::UpdateAlternatives(Default::default())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateAlternatives {
    pub jre_paths: Vec<UpdateAlternativePath>,
    pub jdk_paths: Vec<UpdateAlternativePath>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateAlternativePath {
    pub system_path: String,
    pub exec_name: String,
}

impl From<(&str, &str)> for UpdateAlternativePath {
    fn from((sys, internal): (&str, &str)) -> Self {
        Self {
            system_path: sys.to_string(),
            exec_name: internal.to_string(),
        }
    }
}

impl Default for UpdateAlternatives {
    fn default() -> Self {
        UpdateAlternatives {
            jre_paths: vec![("/usr/bin/java", "java").into()],
            jdk_paths: vec![
                ("/usr/bin/java", "java").into(),
                ("/usr/bin/javac", "javac").into(),
                ("/usr/bin/javadoc", "javadoc").into(),
                ("/usr/bin/javah", "javah").into(),
                ("/usr/bin/javap", "javap").into(),
                ("/usr/bin/javaws", "javaws").into(),
            ],
        }
    }
}
