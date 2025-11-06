use calamine::{Reader, open_workbook, Xlsx, Xls, Data};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Użycie: {} <ścieżka_do_pliku_excel>", args[0]);
        eprintln!("Usage: {} <path_to_excel_file>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];

    // Określ typ pliku na podstawie rozszerzenia
    // Determine file type based on extension
    if file_path.ends_with(".xlsx") {
        if let Err(e) = read_xlsx(file_path) {
            eprintln!("Błąd odczytu pliku XLSX: {}", e);
            eprintln!("Error reading XLSX file: {}", e);
            process::exit(1);
        }
    } else if file_path.ends_with(".xls") {
        if let Err(e) = read_xls(file_path) {
            eprintln!("Błąd odczytu pliku XLS: {}", e);
            eprintln!("Error reading XLS file: {}", e);
            process::exit(1);
        }
    } else {
        eprintln!("Nieobsługiwany format pliku. Użyj .xlsx lub .xls");
        eprintln!("Unsupported file format. Use .xlsx or .xls");
        process::exit(1);
    }
}

fn read_xlsx(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;

    println!("\n=== Odczyt pliku Excel: {} ===\n", path);
    println!("=== Reading Excel file: {} ===\n", path);

    // Pobierz nazwy wszystkich arkuszy
    // Get all sheet names
    let sheet_names = workbook.sheet_names().to_owned();

    for sheet_name in sheet_names {
        println!("📊 Arkusz / Sheet: {}", sheet_name);
        println!("{}", "=".repeat(60));

        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            print_range(&range);
        }
        println!();
    }

    Ok(())
}

fn read_xls(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook: Xls<_> = open_workbook(path)?;

    println!("\n=== Odczyt pliku Excel: {} ===\n", path);
    println!("=== Reading Excel file: {} ===\n", path);

    // Pobierz nazwy wszystkich arkuszy
    // Get all sheet names
    let sheet_names = workbook.sheet_names().to_owned();

    for sheet_name in sheet_names {
        println!("📊 Arkusz / Sheet: {}", sheet_name);
        println!("{}", "=".repeat(60));

        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            print_range(&range);
        }
        println!();
    }

    Ok(())
}

fn print_range(range: &calamine::Range<Data>) {
    let (height, width) = range.get_size();

    if height == 0 || width == 0 {
        println!("(pusty arkusz / empty sheet)");
        return;
    }

    println!("Rozmiar / Size: {} wierszy × {} kolumn", height, width);
    println!("           {} rows × {} columns\n", height, width);

    // Wyświetl dane w formacie tabelarycznym
    // Display data in tabular format
    for (row_idx, row) in range.rows().enumerate() {
        print!("Wiersz/Row {:3} | ", row_idx + 1);

        for (col_idx, cell) in row.iter().enumerate() {
            let cell_str = match cell {
                Data::Empty => String::from(""),
                Data::String(s) => s.clone(),
                Data::Float(f) => f.to_string(),
                Data::Int(i) => i.to_string(),
                Data::Bool(b) => b.to_string(),
                Data::Error(e) => format!("ERR: {:?}", e),
                Data::DateTime(dt) => format!("DateTime: {}", dt),
                Data::DateTimeIso(dt) => dt.clone(),
                Data::DurationIso(d) => d.clone(),
            };

            if col_idx > 0 {
                print!(" | ");
            }
            print!("{}", cell_str);
        }
        println!();
    }
}
