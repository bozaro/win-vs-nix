set BIN=%VS120COMNTOOLS%\..\..\VC\bin\amd64
"%BIN%\ml64.exe" minimal.asm /link /subsystem:console /entry:main
