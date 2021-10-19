use clap::{crate_authors, crate_description, crate_name, crate_version, load_yaml, App};

fn main() {
    let yml = load_yaml!("main.yml");
    let app = App::from_yaml(yml)
        .about(crate_description!())
        .author(crate_authors!())
        .name(crate_name!())
        .version(crate_version!());
    let _m = app.get_matches();
}
