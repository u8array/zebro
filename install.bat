@echo off
setlocal

echo Downloading zebro-v0.1.0-windows.zip...
powershell -Command "Invoke-WebRequest -Uri 'https://github.com/u8array/zebro/releases/download/v0.1.0/zebro-v0.1.0-windows.zip' -OutFile 'zebro-v0.1.0-windows.zip'"

echo Extracting zebro-v0.1.0-windows.zip...
powershell -Command "Expand-Archive -Path 'zebro-v0.1.0-windows.zip' -DestinationPath 'zebro'"

echo Moving zebro.exe to Program Files...
move "zebro\zebro.exe" "%ProgramFiles%\zebro\zebro.exe"

echo Adding zebro to PATH...
set "PATH=%PATH%;%ProgramFiles%\zebro"
setx PATH "%PATH%"

echo Installation complete. You can now use the 'zebro' command.

endlocal
pause