# Roblox Catalog Avatar Creator Exporter

This Rust project helps streamline the process of exporting item IDs from the Roblox Catalog Avatar Creator. It allows users to easily input catalog URLs, extract the item ID, and manage their outfits.

## Features

- Parse catalog item URLs and extract the item ID.
- Input multiple item URLs and store them as an outfit.
- Option to:
  - Set the list of item IDs to the clipboard.
  - Write the list to a text file.
  - Exit the program.

## Prerequisites

Make sure you have the following installed:
- [Rust](https://www.rust-lang.org/) (latest stable version)
- [Cargo](https://doc.rust-lang.org/cargo/) (comes bundled with Rust)

You also need the following dependencies in your `Cargo.toml`:

[dependencies]  
cli_clipboard = "0.7.1"  
colored = "2.0"  
text_io = "0.1"  
url = "2.2"  

## Usage

1. **Clone the repository:**

```bash
git clone <repository_url>  
cd <repository_name>  
```

2. **Run the program:**

```bash
cargo run  
```

3. **Enter Catalog URLs:**

- Paste a catalog URL into the program to extract the item ID.
- Input `-1` when you’re done adding items to your outfit.

4. **Exit Menu:**

The program will show an exit menu with the following options:
```bash
- `1`: Create a new outfit.
- `2`: Set the item list to the clipboard.
- `3`: Write the item list to a file (`outfits.txt`).
- `4`: Exit the program.
```

## Error Handling

If you input an invalid URL, the program will provide an error message indicating the issue, such as:
- **Not a catalog link**: The URL provided does not lead to a Roblox catalog page.
- **Failed to parse the link**: There was an issue parsing the URL.
- **No item ID**: The URL does not contain an item ID.

## Example Output

```bash
[Outfit Menu]  
First input:  
https://www.roblox.com/catalog/123456789/Some-Catalog-Item  
next input:  
https://www.roblox.com/catalog/987654321/Another-Item  
next input:   
-1  

[EXIT MENU]  
1. Create a new outfit  
2. Set To Clipboard  
3. Write to file
```
