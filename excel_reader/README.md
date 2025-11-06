# Excel Reader - Czytnik plików Excel w Rust

Prosty program w Rust do odczytu plików Excel (.xlsx i .xls).

A simple Rust program for reading Excel files (.xlsx and .xls).

## Funkcje / Features

- ✅ Odczyt plików .xlsx (Excel 2007+)
- ✅ Odczyt plików .xls (Excel 97-2003)
- ✅ Wyświetlanie wszystkich arkuszy w skoroszycie
- ✅ Obsługa różnych typów danych (tekst, liczby, daty, bool)
- ✅ Czytelne formatowanie wyjścia

---

- ✅ Read .xlsx files (Excel 2007+)
- ✅ Read .xls files (Excel 97-2003)
- ✅ Display all sheets in workbook
- ✅ Support for different data types (text, numbers, dates, bool)
- ✅ Clean output formatting

## Instalacja / Installation

### Wymagania / Requirements

- Rust 1.70+ (zalecane najnowsza wersja / latest recommended)
- Cargo

### Kompilacja / Build

```bash
cd excel_reader
cargo build --release
```

## Użycie / Usage

### Uruchomienie / Run

```bash
cargo run -- <ścieżka_do_pliku_excel>
```

Lub skompilowany plik binarny:

Or using the compiled binary:

```bash
./target/release/excel_reader <ścieżka_do_pliku_excel>
```

### Przykłady / Examples

```bash
# Odczyt pliku XLSX
cargo run -- dane.xlsx

# Odczyt pliku XLS
cargo run -- raport.xls

# Pełna ścieżka
cargo run -- /home/user/dokumenty/budżet.xlsx
```

## Format wyjścia / Output Format

Program wyświetla:
The program displays:

1. Nazwę pliku / File name
2. Dla każdego arkusza / For each sheet:
   - Nazwę arkusza / Sheet name
   - Rozmiar (liczba wierszy i kolumn) / Size (number of rows and columns)
   - Wszystkie dane w formacie tabelarycznym / All data in tabular format

### Przykład wyjścia / Example Output

```
=== Odczyt pliku Excel: dane.xlsx ===
=== Reading Excel file: dane.xlsx ===

📊 Arkusz / Sheet: Arkusz1
============================================================
Rozmiar / Size: 3 wierszy × 3 kolumn
           3 rows × 3 columns

Wiersz/Row   1 | Imię | Nazwisko | Wiek
Wiersz/Row   2 | Jan | Kowalski | 30
Wiersz/Row   3 | Anna | Nowak | 25
```

## Zależności / Dependencies

- [calamine](https://crates.io/crates/calamine) - biblioteka do odczytu plików Excel / Excel file reading library

## Obsługiwane typy danych / Supported Data Types

- Tekst / Text (String)
- Liczby całkowite / Integers (Int)
- Liczby zmiennoprzecinkowe / Floating point (Float)
- Wartości logiczne / Boolean (Bool)
- Daty i czas / Dates and times (DateTime)
- Czas trwania / Duration
- Puste komórki / Empty cells

## Licencja / License

MIT
