@echo off
title NexoPDF - Workbench Documental
echo ========================================================
echo               Iniciando NexoPDF...
echo ========================================================
echo.

cd /d "%~dp0"

if exist "target\debug\nexopdf.exe" (
    start "" "target\debug\nexopdf.exe" %*
    exit /b 0
)

cargo run -p nexopdf

if errorlevel 1 (
    echo.
    echo Ocorreu um erro ao executar o NexoPDF.
    pause
)
