@echo off
chcp 65001 > nul
echo ?? wid derleme sureci ba?lat?ld?...

if exist wid.exe (
    echo ?? Eski wid.exe kald?r?l?yor...
    del /F /Q wid.exe
)

cargo build --release
if %ERRORLEVEL% NEQ 0 (
    echo ? Derleme hatas?! Lutfen kodunuzu kontrol edin.
    pause
    exit /b %ERRORLEVEL%
)

echo ?? Derleme ba?ar?l?. Dosya ana dizine ta??n?yor...
copy /Y target\release\wid.exe wid.exe > nul

echo ?? ??lem tamam! Guncel wid.exe ana dizinde kullan?ma haz?r.
echo ?? Test etmek icin: wid.exe install 7zip