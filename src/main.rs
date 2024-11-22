use clap::{Parser, Subcommand};
use colored::Colorize;
use std::io::{self, Write};
use std::process::Command;

#[cfg(windows)]
use windows::core::PCWSTR;
#[cfg(windows)]
use windows::Win32::Storage::FileSystem::{GetDriveTypeW, GetLogicalDrives};

#[cfg(unix)]
use std::path::Path;
#[cfg(unix)]
use nix::mount;
#[cfg(unix)]
use sysinfo::{DiskExt, System, SystemExt};
#[cfg(unix)]
use std::fs;

#[derive(Parser)]
#[command(
    name = "fixr",
    about = "Çapraz Platform Disk Yönetim Aracı",
    version = "1.0",
    author = "Your Name"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

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
    path: String,
    is_removable: bool,
    total_space: u64,
    free_space: u64,
}

#[cfg(windows)]
const DRIVE_REMOVABLE: u32 = 2;

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

// Platform-specific implementations
#[cfg(windows)]
fn validate_drive(s: &str) -> Result<String, String> {
    let s = s.to_uppercase();
    if s.len() == 2 && s.ends_with(':') && s.chars().next().unwrap().is_ascii_uppercase() {
        Ok(s)
    } else {
        Err("Disk harfi geçersiz format. Örnek: 'F:'".to_string())
    }
}

#[cfg(unix)]
fn validate_drive(s: &str) -> Result<String, String> {
    let path = Path::new(s);
    if path.exists() && path.to_string_lossy().starts_with("/dev/") {
        Ok(s.to_string())
    } else {
        Err("Geçersiz disk yolu. Örnek: '/dev/sdb1'".to_string())
    }
}

#[cfg(windows)]
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

#[cfg(unix)]
fn list_drives(verbose: bool) -> io::Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let removable_disks: Vec<_> = sys.disks().iter()
        .filter(|disk| is_removable(disk.mount_point().to_str().unwrap_or("")))
        .collect();

    if removable_disks.is_empty() {
        println!("{}", "Hiçbir taşınabilir disk bulunamadı.".yellow());
        return Ok(());
    }

    println!("{}", "\nTaşınabilir Diskler:".green().bold());
    println!("{}", "=================".green());

    for disk in removable_disks {
        let mount_point = disk.mount_point().to_str().unwrap_or("");
        if verbose {
            let total_gb = disk.total_space() as f64 / 1_073_741_824.0;
            let free_gb = disk.available_space() as f64 / 1_073_741_824.0;
            println!(
                "{} ({}) - Toplam: {:.2} GB, Boş: {:.2} GB",
                disk.name().to_str().unwrap_or("").blue().bold(),
                mount_point,
                total_gb,
                free_gb
            );
        } else {
            println!("{} ({})",
                     disk.name().to_str().unwrap_or("").blue().bold(),
                     mount_point
            );
        }
    }
    println!();
    Ok(())
}

#[cfg(windows)]
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

    execute_repair_command(command)
}

#[cfg(unix)]
fn fix_drive(disk: &str, force: bool) -> io::Result<()> {
    if !is_removable(disk) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Belirtilen aygıt taşınabilir disk değil!",
        ));
    }

    if force {
        // Linux'ta diski unmount et
        let _ = unmount_drive(disk);
    }

    print!("Disk onarımı başlatılıyor... ");
    io::stdout().flush()?;

    let mut command = Command::new("fsck");
    command.arg("-y").arg(disk);

    if force {
        command.arg("-f");
    }

    execute_repair_command(command)
}

fn execute_repair_command(mut command: Command) -> io::Result<()> {
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

#[cfg(windows)]
fn show_drive_info(disk: &str) -> io::Result<()> {
    if !is_removable(disk) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Belirtilen sürücü taşınabilir disk değil!",
        ));
    }

    let info = get_drive_info(disk)?;
    display_drive_info(disk, info.total_space, info.free_space)
}

#[cfg(unix)]
fn show_drive_info(disk: &str) -> io::Result<()> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let disk_info = sys.disks().iter()
        .find(|d| d.name().to_str().unwrap_or("") == disk)
        .ok_or_else(|| io::Error::new(
            io::ErrorKind::NotFound,
            "Disk bulunamadı!"
        ))?;

    display_drive_info(
        disk,
        disk_info.total_space(),
        disk_info.available_space()
    )
}

fn display_drive_info(disk: &str, total_space: u64, free_space: u64) -> io::Result<()> {
    let total_gb = total_space as f64 / 1_073_741_824.0;
    let free_gb = free_space as f64 / 1_073_741_824.0;
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

// Platform-specific helper functions
#[cfg(windows)]
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

#[cfg(windows)]
fn is_removable(drive: &str) -> bool {
    let drive_path = format!("{}\\", drive);
    let mut wide_path: Vec<u16> = drive_path.encode_utf16().collect();
    wide_path.push(0);
    unsafe {
        GetDriveTypeW(PCWSTR(wide_path.as_ptr())) == DRIVE_REMOVABLE
    }
}

#[cfg(unix)]
fn is_removable(path: &str) -> bool {
    if let Some(device_name) = Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .and_then(|n| n.strip_prefix("sd"))
    {
        let removable_path = format!("/sys/block/sd{}/removable", device_name);
        if let Ok(content) = fs::read_to_string(removable_path) {
            return content.trim() == "1";
        }
    }
    false
}

#[cfg(unix)]
fn unmount_drive(path: &str) -> io::Result<()> {
    match mount::umount(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, e.to_string()))
    }
}

#[cfg(windows)]
fn get_drive_info(drive: &str) -> io::Result<DriveInfo> {
    Ok(DriveInfo {
        path: drive.to_string(),
        is_removable: is_removable(drive),
        total_space: 1000000000,  // Windows API'den gerçek değerleri almalısınız
        free_space: 500000000,    // Windows API'den gerçek değerleri almalısınız
    })
}