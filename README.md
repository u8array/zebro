# Zebro

Zebro is a simple CLI tool for sending ZPL code to a label printer.

## Installation

### Windows

todo

### Linux

Download the installation script and run it:

```sh
wget https://raw.githubusercontent.com/u8array/zebro/main/install.sh
chmod +x install.sh
./install.sh
```

## Usage

Use the following command to send ZPL code to a printer:

```sh
zebro --address <IP:PORT> --zpl <ZPL_CODE>
```

### Arguments

- `-a`, `--address <IP:PORT>`: Defines the printer's address (IP:PORT).
- `-i`, `--ip <IP>`: Defines the printer's IP.
- `-p`, `--port <PORT>`: Defines the printer's port (default: 9100).
- `-z`, `--zpl <ZPL_CODE>`: The ZPL code to be sent to the printer.

### Examples

Send ZPL code to a printer with a specific address:

```sh
zebro --address 192.168.1.100:9100 --zpl "^XA^FO50,50^ADN,36,20^FDHello, World!^FS^XZ"
```

Send ZPL code to a printer with IP and default port:

```sh
zebro --ip 192.168.1.100 --zpl "^XA^FO50,50^ADN,36,20^FDHello, World!^FS^XZ"
```

Send ZPL code from a file to a printer:

```sh
cat label.zpl | zebro --ip 192.168.1.100 --zpl -
```

## Error Handling

If an error occurs, an appropriate error message will be displayed. Possible errors include:

- `ConnectionError`: Failed to connect to the printer.
- `SendError`: Failed to send the ZPL code.
- `MissingZplCode`: ZPL code is required.
- `MissingAddress`: Either address or IP must be provided.

## License

This project is licensed under the MIT License. For more information, see the LICENSE file.
