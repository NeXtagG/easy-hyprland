{
  lib,
  rustPlatform,
  makeWrapper,
  libxkbcommon,
  wayland,
  libGL,
  xorg,
  # Top-level names in current nixpkgs, xorg.* in older releases.
  libx11 ? xorg.libX11,
  libxcursor ? xorg.libXcursor,
  libxi ? xorg.libXi,
  libxrandr ? xorg.libXrandr,
}:

let
  cargoToml = lib.importTOML ../Cargo.toml;
  # egui/winit load these with dlopen at runtime, so they must be on LD_LIBRARY_PATH.
  runtimeLibs = [
    libxkbcommon
    wayland
    libGL
    libx11
    libxcursor
    libxi
    libxrandr
  ];
in
rustPlatform.buildRustPackage {
  pname = "easy-hyprland";
  inherit (cargoToml.package) version;

  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.unions [
      ../Cargo.toml
      ../Cargo.lock
      ../src
      ../packaging/easy-hyprland.desktop
    ];
  };

  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [ makeWrapper ];

  postInstall = ''
    install -Dm644 packaging/easy-hyprland.desktop $out/share/applications/easy-hyprland.desktop
    wrapProgram $out/bin/easy-hyprland \
      --prefix LD_LIBRARY_PATH : ${lib.makeLibraryPath runtimeLibs}
  '';

  passthru = { inherit runtimeLibs; };

  meta = {
    description = cargoToml.package.description;
    homepage = "https://github.com/NeXtagG/easy-hyprland";
    license = lib.licenses.gpl3Plus;
    mainProgram = "easy-hyprland";
    platforms = lib.platforms.linux;
  };
}
