[Setup]
AppId={{E3F8A2B1-6C4D-4A9E-9F0B-1D5C7E2A8B34}
AppName=Cad Helper 3d
AppVersion=1.0.0
DefaultDirName={autopf}\CadHelper3d
DefaultGroupName=Cad Helper 3d
UninstallDisplayIcon={app}\cad-helper-3d.exe
Compression=lzma2
SolidCompression=yes
OutputDir=.
OutputBaseFilename=cad-helper-3d-setup
SetupIconFile=ui\images\ico.ico

[Files]
; Copy the main executable
Source: "cad-helper-3d.exe"; DestDir: "{app}"; Flags: ignoreversion
; Copy the UI images asset folder so icons render inside the app
Source: "ui\images\*"; DestDir: "{app}\ui\images"; Flags: ignoreversion recursesubdirs createallsubdirs

[Icons]
Name: "{group}\Cad Helper 3d"; Filename: "{app}\cad-helper-3d.exe"
Name: "{autodesktop}\Cad Helper 3d"; Filename: "{app}\cad-helper-3d.exe"