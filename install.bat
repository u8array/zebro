@echo off
setlocal

echo Downloading release.zip...
powershell -Command "Invoke-WebRequest -Uri 'https://raw.githubusercontent.com/u8array/zebro/master/release.zip' -OutFile 'release.zip'"

echo Extracting release.zip...
powershell -Command "Expand-Archive -Path 'release.zip' -DestinationPath 'zebro'"

echo Moving zebro.exe to Program Files...
move "zebro\zebro.exe" "%ProgramFiles%\zebro\zebro.exe"

echo Adding zebro to PATH...
set "PATH=%PATH%;%ProgramFiles%\zebro"
setx PATH "%PATH%"

echo Installation complete. You can now use the 'zebro' command.

endlocal
pause
