@echo off
setlocal

REM Download release.zip
echo Downloading release.zip...
powershell -Command "Invoke-WebRequest -Uri 'https://raw.githubusercontent.com/u8array/zebro/main/release.zip' -OutFile 'release.zip'"

REM Extract release.zip
echo Extracting release.zip...
powershell -Command "Expand-Archive -Path 'release.zip' -DestinationPath 'zebro'"

REM Move zebro.exe to Program Files
echo Moving zebro.exe to Program Files...
move "zebro\zebro.exe" "%ProgramFiles%\zebro\zebro.exe"

REM Add zebro to PATH
echo Adding zebro to PATH...
set "PATH=%PATH%;%ProgramFiles%\zebro"
setx PATH "%PATH%"

echo Installation complete. You can now use the 'zebro' command.

endlocal
pause