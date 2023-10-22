use clap::{command, Parser};

#[derive(Debug, Parser)]
#[command(author = "lI15SO0", version, about = "Init nvim workspace dir.", long_about = None)]
struct Args {
    path: Option<String>,

    #[arg(short, long)]
    force: bool,
}

fn create_dirs(path: &str) {
    let _ = std::fs::create_dir_all(path);
}

fn create_exrc(path: &str) {
    let _ = std::fs::write(
        format!("{}{}", path, ".nvim.lua"),
        include_str!("./nvim.lua"),
    );
    println!("Write: {}{}", path, ".nvim.lua");
}

fn create_snip_package_file(path: &str) {
    let _ = std::fs::write(
        format!("{}{}", path, ".nvim/snip/package.json"),
        include_str!("./package.json"),
    );
    println!("Write: {}{}", path, ".nvim/snip/package.json");

    let _ = std::fs::write(
        format!("{}{}", path, ".nvim/snip/global.json"),
        include_str!("./global.json"),
    );
    println!("Write: {}{}", path, ".nvim/snip/global.json");
}

fn create_core_file(path: &str) {
    let _ = std::fs::write(format!("{}{}", path, ".nvim/lua/ws/core/options.lua"), "");
    println!("Write: {}{}", path, ".nvim/lua/ws/core/options.lua");
    let _ = std::fs::write(format!("{}{}", path, ".nvim/lua/ws/core/keymaps.lua"), "");
    println!("Write: {}{}", path, ".nvim/lua/ws/core/keymaps.lua");
    let _ = std::fs::write(format!("{}{}", path, ".nvim/lua/ws/core/dap.lua"), include_str!("./dap.lua"));
    println!("Write: {}{}", path, ".nvim/lua/ws/core/dap.lua");
}

fn already_is_ws(path: &str, force: bool) {
    if !force {
        let exist = std::fs::metadata(path)
            .and_then(|_| Ok(true))
            .or_else(|_| Ok::<bool, String>(false))
            .unwrap();

        if exist {
            match path {
                "./.nvim.lua" => {
                    println!("This path already was nvim workspace.(You can run with \"-f\" to force init this path as workspace after you backup file.)");
                }
                _ => {
                    println!("{} already exist.(You can run with \"-f\" to force init this path as workspace after you backup file.)", path);
                }
            }
            std::process::exit(0);
        }
    }
}

fn main() {
    let args = Args::parse();

    let dirs: Vec<String> = include_str!("./dirs.txt")
        .lines()
        .map(|f| f.to_string())
        .collect();

    match args.path {
        Some(path) => {
            let p = path
                .contains("\\")
                .then(|| path.clone().replace("\\", "/"))
                .or_else(|| (Some(path.clone())))
                .unwrap();
            let p = p
                .ends_with("/")
                .then(|| p.clone())
                .or_else(|| Some(format!("{}{}", p, '/')))
                .unwrap();

            already_is_ws(&path, args.force);

            for i in dirs {
                create_dirs(&(format!("{}{}", p, i)))
            }
            create_exrc(&p);
            create_snip_package_file(&p);
            create_core_file(&p);
            println!("Generate done at \"{}\" .", path);
        }
        None => {
            already_is_ws("./.nvim.lua", args.force);
            for i in dirs {
                create_dirs(&(format!("{}{}", "./", i)))
            }
            create_exrc("./");
            create_snip_package_file("./");
            create_core_file("./");
            println!("Generate done at \"./\" .");
        }
    }
}
