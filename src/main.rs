use std::ffi::{OsStr, OsString};
use clap::{Parser, Subcommand};
use colored::Colorize;
use std::io::{self, Write};
use std::process::Command;
use windows::core::PCWSTR;
use windows::Win32::Storage::FileSystem::{GetDriveTypeA, GetDriveTypeW, GetLogicalDrives};
#[derive(Parser)]
#[command(
    name = "fixr",
    about = "Taşınabilir disk yönetim aracı",
    version = "1.0",
    author = "Your Name"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

const DRIVE_REMOVABLE: u32 = 2;

#[derive(Subcommand)]
enum Commands {
    List {
        #[arg(short, long, help = "Ayrıntılı bilgi göster")]
        verbose: bool,
    },
    Fix {
        #[arg(value_parser = validate_drive)]
        disk: String,

        #[arg(short, long, help = "Zorla onarım gerçekleştir")]
        force: bool,
    },
    Info {
        #[arg(value_parser = validate_drive)]
        disk: String,
    },
}

struct DriveInfo {
    letter: String,
    is_removable: bool,
    total_space: u64,
    free_space: u64,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { verbose } => {
            list_drives(verbose)?;
        }
        Commands::Fix { disk, force } => {
            fix_drive(&disk, force)?;
        }
        Commands::Info { disk } => {
            show_drive_info(&disk)?;
        }
    }

    Ok(())
}

fn validate_drive(s: &str) -> Result<String, String> {
    let s = s.to_uppercase();
    if s.len() == 2 && s.ends_with(':') && s.chars().next().unwrap().is_ascii_uppercase() {
        Ok(s)
    } else {
        Err("Disk harfi geçersiz format. Örnek: 'F:'".to_string())
    }
}

fn list_drives(verbose: bool) -> io::Result<()> {
    let drives = get_removable_drives()?;

    if drives.is_empty() {
        println!("{}", "Hiçbir taşınabilir disk bulunamadı.".yellow());
        return Ok(());
    }

    println!("{}", "\nTaşınabilir Diskler:".green().bold());
    println!("{}", "=================".green());

    for drive in drives {
        if verbose {
            let info = get_drive_info(&drive)?;
            let total_gb = info.total_space as f64 / 1_073_741_824.0;
            let free_gb = info.free_space as f64 / 1_073_741_824.0;
            println!(
                "{} - Toplam: {:.2} GB, Boş: {:.2} GB",
                drive.blue().bold(),
                total_gb,
                free_gb
            );
        } else {
            println!("{}", drive.blue().bold());
        }
    }
    println!();
    Ok(())
}

fn fix_drive(disk: &str, force: bool) -> io::Result<()> {
    if !is_removable(disk) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Belirtilen sürücü taşınabilir disk değil!",
        ));
    }

    print!("Disk onarımı başlatılıyor... ");
    io::stdout().flush()?;

    let mut command = Command::new("chkdsk");
    command.arg(disk);

    if force {
        command.arg("/F").arg("/X");
    } else {
        command.arg("/F");
    }

    let output = command.output()?;

    if output.status.success() {
        println!("{}", "Başarılı!".green());
        println!("\nOnarım çıktısı:");
        println!("{}", "=============".green());
        println!("{}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    } else {
        println!("{}", "Başarısız!".red());
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "Disk onarımı başarısız oldu: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ))
    }
}

fn show_drive_info(disk: &str) -> io::Result<()> {
    if !is_removable(disk) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Belirtilen sürücü taşınabilir disk değil!",
        ));
    }

    let info = get_drive_info(disk)?;
    let total_gb = info.total_space as f64 / 1_073_741_824.0;
    let free_gb = info.free_space as f64 / 1_073_741_824.0;
    let used_gb = total_gb - free_gb;
    let usage_percent = (used_gb / total_gb) * 100.0;

    println!("\n{} Disk Bilgileri:", disk.blue().bold());
    println!("{}", "===============".blue());
    println!("Toplam Alan: {:.2} GB", total_gb);
    println!("Kullanılan Alan: {:.2} GB ({:.1}%)", used_gb, usage_percent);
    println!("Boş Alan: {:.2} GB", free_gb);
    println!("Tip: {}", "Taşınabilir Disk".green());

    Ok(())
}

fn get_removable_drives() -> io::Result<Vec<String>> {
    let mut drives = Vec::new();
    let drive_mask = unsafe { GetLogicalDrives() };

    if drive_mask == 0 {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Sürücü maskesi alınamadı",
        ));
    }

    for i in 0..26 {
        if (drive_mask >> i) & 1 != 0 {
            let drive_letter = format!("{}:", (b'A' + i) as char);
            if is_removable(&drive_letter) {
                drives.push(drive_letter);
            }
        }
    }
    Ok(drives)
}



fn is_removable(drive: &str) -> bool {
    let drive_path = format!("{}\\", drive);
    let mut wide_path: Vec<u16> = drive_path.encode_utf16().collect();
    wide_path.push(0); // Null terminate
    unsafe {
        GetDriveTypeW(PCWSTR(wide_path.as_ptr())) == DRIVE_REMOVABLE
    }
}



fn get_drive_info(drive: &str) -> io::Result<DriveInfo> {
    Ok(DriveInfo {
        letter: drive.to_string(),
        is_removable: is_removable(drive),
        total_space: 1000000000,
        free_space: 500000000,
    })
}