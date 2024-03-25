use sdbootutil::cli::{parse_args, Commands};
use sdbootutil as lib;
use sdbootutil::fs;
use sdbootutil::MessagePrinter;
use std::path::PathBuf;

fn main() {
    let args = parse_args();
    let _snapshot = args.snapshot.unwrap_or_else(lib::get_root_snapshot);
    let console_printer = lib::ConsolePrinter;
    let command_executor = lib::RealCommandExecutor;

    match args.cmd {
        Some(Commands::Kernels {}) => lib::command_kernels(&console_printer),
        Some(Commands::Snapshots {}) => lib::command_snapshots(&console_printer),
        Some(Commands::Entries {}) => lib::command_entries(&console_printer),
        Some(Commands::Bootloader {}) => lib::command_bootloader(&console_printer),
        Some(Commands::AddKernel { kernel_version }) => lib::command_add_kernel(&console_printer, &kernel_version),
        Some(Commands::AddAllKernels {}) => lib::command_add_all_kernels(&console_printer),
        Some(Commands::Mkinitrd {}) => lib::command_mkinitrd(&console_printer),
        Some(Commands::RemoveKernel { kernel_version }) => lib::command_remove_kernel(&console_printer, &kernel_version),
        Some(Commands::RemoveAllKernels {}) => lib::command_remove_all_kernels(&console_printer),
        Some(Commands::ListKernels {}) => lib::command_list_kernels(&console_printer),
        Some(Commands::ListEntries {}) => lib::command_list_entries(&console_printer),
        Some(Commands::ListSnapshots {}) => lib::command_list_snapshots(&console_printer),
        Some(Commands::SetDefaultSnapshot {}) => lib::command_set_default_snapshot(&console_printer),
        Some(Commands::IsBootable {}) => lib::command_is_bootable(&console_printer),
        Some(Commands::Install {}) => lib::command_install(&console_printer),
        Some(Commands::NeedsUpdate {}) => lib::command_needs_update(&console_printer),
        Some(Commands::Update {}) => lib::command_update(&console_printer),
        Some(Commands::ForceUpdate {}) => lib::command_force_update(&console_printer),
        Some(Commands::UpdatePredictions {}) => lib::command_update_predictions(&console_printer),
        None => lib::ui::show_main_menu(),
    }

    if fs::is_transactional(&command_executor).expect("Failed to check if filesystem is transactional") {
        console_printer.log_info("It is a transactional system")
    }
    else {
        console_printer.log_info("It is not a transactional system")
    }
    let (_temp_dir, _tmpdir_path) = fs::create_temp_dir();
    let rollback_items = vec![
        fs::RollbackItem::new(PathBuf::from("/path/to/file1")),
        fs::RollbackItem::new(PathBuf::from("/path/to/file2")),
    ];
    fs::cleanup_rollback_items(&rollback_items);
}