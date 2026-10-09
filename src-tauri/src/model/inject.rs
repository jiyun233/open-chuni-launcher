use std::process::Command;

pub struct InjectSpec {
    pub injector: &'static str,
    pub exe: String,
    pub dlls: Vec<String>,
    pub target_args: Vec<String>,
}

pub struct BuiltInjection {
    pub command: Command,
    pub missing_dlls: Vec<String>,
}
