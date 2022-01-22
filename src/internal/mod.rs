pub mod exec;
pub mod files;
pub mod install;
pub mod strings;

pub fn install(pkgs: Vec<&str>) {
    install::install(pkgs);
}

pub fn info(a: String) {
    strings::info(a);
}

pub fn crash(a: String, b: i32) {
    strings::crash(a, b);
}

pub fn log(a: String) {
    strings::log(a);
}

#[macro_export]
macro_rules! uwu {
    ($x:expr) => {{
        let uwu: String = String::from_str($x).unwrap();
        let uwu = uwu.replace("l", "w");
        let uwu = uwu.replace("L", "W");
        let uwu = uwu.replace("r", "w");
        let uwu = uwu.replace("R", "W");
        let uwu = uwu.replace("na", "nya");
        let uwu = uwu.replace("Na", "Nya");
        let uwu = uwu.replace("NA", "NYA");
        uwu
    }};
}
