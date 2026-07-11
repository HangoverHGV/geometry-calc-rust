[Setup]
AppName=Inventor Calculator Helper
AppVersion=1.0.0
DefaultDirName={autopf}\InventorCalculatorHelper
DefaultGroupName=Inventor Calculator Helper
UninstallDisplayIcon={app}\inventor-math-calculator.exe
Compression=lzma2
SolidCompression=yes
OutputDir=.
OutputBaseFilename=inventor-calculator-helper-setup
SetupIconFile=ui\images\ico.ico

[Files]
; Copy the main executable
Source: "inventor-math-calculator.exe"; DestDir: "{app}"; Flags: ignoreversion
; Copy the UI images asset folder so icons render inside the app
Source: "ui\images\*"; DestDir: "{app}\ui\images"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\Inventor Calculator Helper"; Filename: "{app}\inventor-math-calculator.exe"
Name: "{autodesktop}\Inventor Calculator Helper"; Filename: "{app}\inventor-math-calculator.exe"