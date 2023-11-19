use clap::{command, Parser};

#[derive(Parser)]
#[command(author = "lI15SO0", version, about = "Init nvim workspace dir.", long_about = None)]
struct Args {
    path: Option<String>,

    #[arg(short, long)]
    force: bool,
}

fn create_dir(path: &str) {
    std::fs::create_dir_all(path).is_err().then(||{
        println!("Error: Failed to create dir: {}", path);
        std::process::exit(1);
    });
}

fn create_dirs(root: &str, dirs: Vec<String>) {
    dirs.iter()
        .for_each(|dir| create_dir(&(format!("{}{}", root, dir))));
}

fn write_file(path: &str, contents: &str) {
    std::fs::write(path, contents).is_err().then(||{
        println!("Error: Failed to write file: {}", path);
        std::process::exit(1);
    });
    println!("Write: {}", path);
}

fn write_files(path: &str) {
    [
        (".nvim.lua", include_str!("./assets/nvim.lua")),
        (".nvim/snip/package.json", include_str!("./assets/package.json"),),
        (".nvim/snip/global.json", include_str!("./assets/global.json"),),
        (".nvim/lua/ws/core/dap.lua", include_str!("./assets/dap.lua"),),
        (".nvim/lua/ws/core/options.lua", ""),
        (".nvim/lua/ws/core/keymaps.lua", ""),
    ]
    .iter()
    .for_each(|(file, contents)| {
        write_file(&format!("{}{}", path, file), contents);
    });
}

fn already_is_ws(path: &str, force: bool) {
    if !force {
        let exist = std::fs::metadata(format!("{}{}", path, ".nvim.lua"))
            .and_then(|_| Ok(true))
            .or_else(|_| Ok::<bool, String>(false))
            .unwrap();

        if exist {
            match path {
                "./" => {
                    println!("This path already was nvim workspace.(run with \"-f\" to force init workspace.)");
                }
                _ => {
                    println!("{} already exist.(run with \"-f\" to force init workspace.)", path);
                }
            }
            std::process::exit(0);
        }
    }

    println!("Warning: Run at force mode.");
}

fn main() {
    let args = Args::parse();

    let dirs: Vec<String> = include_str!("./assets/dirs.txt")
        .lines()
        .map(|f| f.to_string())
        .collect();

    let path = match args.path {
        Some(path) => {
            let p = path.replace("\\", "/");
            p.ends_with("/")
                .then(|| p.clone())
                .or_else(|| Some(format!("{}{}", p, '/')))
                .unwrap()
        }
        None => "./".to_string(),
    };

    already_is_ws(&path, args.force);
    create_dirs(&path, dirs);
    write_files(&path);

    println!("Generate done at \"{}\" .", path);
}
