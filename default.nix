{ lib, naersk, targetPlatform, pkg-config, cargo, rustc, openssl }:

let cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));

in naersk.lib."${targetPlatform.system}".buildPackage rec {
  src = ./.;

  buildInputs = [ pkg-config cargo rustc openssl ];
  checkInputs = [ cargo rustc ];

  doCheck = true;
  copyLibs = true;

  name = cargoToml.package.name;
  version = cargoToml.package.version;

  meta = with lib; {
    description = cargoToml.package.description;
    homepage = cargoToml.package.homepage;
    license = with licenses; [ mit ];
    maintainers = with maintainers; [ maintainers.zbrox ];
  };
}
