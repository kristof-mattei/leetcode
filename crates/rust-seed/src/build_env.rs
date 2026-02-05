pub const COMPILE_TIME_HOST: &str = env!("COMPILE_TIME_HOST");
pub const COMPILE_TIME_TARGET: &str = env!("COMPILE_TIME_TARGET");
pub const COMPILE_TIME_TARGET_CPU: Option<&str> = const {
    let cttc = env!("COMPILE_TIME_TARGET_CPU");

    if cttc.is_empty() { None } else { Some(cttc) }
};

pub struct BuildEnv {
    host: &'static str,
    target: &'static str,
    target_cpu: Option<&'static str>,
}

impl BuildEnv {
    #[expect(unused, reason = "Library code")]
    pub fn get_host(&self) -> &'static str {
        self.host
    }

    pub fn get_target(&self) -> &'static str {
        self.target
    }

    pub fn get_target_cpu(&self) -> Option<&str> {
        self.target_cpu
    }
}

pub fn get_build_env() -> BuildEnv {
    BuildEnv {
        host: COMPILE_TIME_HOST,
        target: COMPILE_TIME_TARGET,
        target_cpu: COMPILE_TIME_TARGET_CPU,
    }
}

impl std::fmt::Display for BuildEnv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Built on `{}`, targeting `{}`", self.host, self.target)?;

        if let Some(target_cpu) = self.target_cpu {
            write!(f, " (target-cpu `{}`)", target_cpu)?;
        }

        Ok(())
    }
}
