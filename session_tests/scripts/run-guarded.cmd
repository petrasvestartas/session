@echo off
node "%~dp0run-guarded.cjs" %*
exit /b %errorlevel%
