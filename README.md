<p align="center">
    <img src=".github/assets/header.png" alt="Xinux'es {E-IMZO}">
</p>

<p align="center">
    <h3 align="center">Nix effort for E-IMZO service.</h3>
</p>

<p align="center">
    <img align="center" src="https://img.shields.io/github/languages/top/xinux-org/e-imzo?style=flat&logo=nixos&logoColor=5277C3&labelColor=ffffff&color=ffffff" alt="Top Used Language">
    <a href="https://t.me/xinux"><img align="center" src="https://img.shields.io/badge/Chat-grey?style=flat&logo=telegram&logoColor=5277C3&labelColor=ffffff&color=ffffff" alt="Telegram Community"></a>
</p>

## About

This is Uzbek Xinux community (nix users mostly) member's effort on packaging E-IMZO & providing ready to use modules for NixOS users.

> [!CAUTION]
> Think thrice before using this software. We had to let the service have access to memory segments to make it work. It refuses to work in sandboxed environments. Apart from that, it also consumes huge amount of memory heaps to run the service continuously. **We don't take any responsibility** for whatever this software does to you and your computer. We just provide packaging support for this piece of garbage as community demands it.

> [!NOTE]
> We are currently working on upstreaming the NixOS module to nixpkgs: https://github.com/NixOS/nixpkgs/pull/393835

## Guides & Use

This project effort provides you both E-IMZO as a package and ready to use nix modules. Also, don't forget to read [browser](#browsers-must-read-in-any-case) section because, without it your browser will refuse to communicate with the service as E-IMZO uses unverified SSL certification (guess someone couldn't afford normal enterprise SSL certificate). In order to get started, you need to add this flake to your own config:

### Package

If you want to use the package one time, you can easily call the package via `nix run`:

```shell
# Bootstrap necessary directories/files before start
nix run github:xinux-org/e-imzo#e-helper

# Start the e-imzo server
nix run github:xinux-org/e-imzo
```

If you're going to add this package to your own configuration, we provide `e-imzo` binary for every arch at:

```
inputs.e-imzo.packages.x86-64-linux.default
inputs.e-imzo.packages.aarch64-linux.default
inputs.e-imzo.packages.x86-64-darwin.default
inputs.e-imzo.packages.aarch64-darwin.default
```

Yes, technically you can run this software in your **MacOS** too if you have `Nix package manager` installed in it. Nix darwin modules might be added in the future, stay tuned!

### Service Module (configuration use)

In order to make use of this project's modules, you **must have** your own nix configuration flake! Afterwards, you can get started by adding this project to your own configuration flake like this:

```nix
# In your configuration repo flake.nix
{
  inputs.e-imzo.url = "github:xinux-org/e-imzo";

  # Or

  inputs = {
    ...

    # E-IMZO project flake
    e-imzo.url = "github:xinux-org/e-imzo";

    ...
  };
}
```

Afterwards, you need to import the module and use it! You can import the module literally anywhere of your configuration as shown in example below:

```nix
# flake.nix -> nixosConfigurations as an example:
{
  outputs =
    { self
    , nixpkgs
    , nixpkgs-unstable
    , e-imzo # <-- don't forget
    , ...
    } @ inputs:
    {
      nixosConfigurations = {
        "Example" = nixpkgs.lib.nixosSystem {
          specialArgs = { inherit inputs outputs; };
          modules = [
            inputs.e-imzo.nixosModules.e-imzo
            ./nixos/example/configuration.nix
          ];
        };
      };
    };
}

# ./nixos/example/configuration.nix anywhere of your configuration
{
  services.e-imzo = {
    enable = true;
  };
}
```

or if you broke your configurations into parts (modules), you can write your own mini-module like this:

```nix
# ./anywhere/modules/nixos/e-imzo.nix
{ inputs, ... }: {
{
  imports = [inputs.e-imzo.nixosModules.e-imzo];

  services.e-imzo = {
    enable = true;
  };
}
```

You can refer to [available options](#available-options) section for more available features/options/settings~! Finally, please run this command to finish your setup and perform some initialization steps:

```shell
# Bootstrap necessary directories/files
nix run github:xinux-org/e-imzo#e-helper

# Restart service to include all files
sudo systemctl restart e-imzo
```

### Browsers (must read in any case)

Long story short, service runs websocket at `https://127.0.0.1:64443/` with untrusted SSL certificate. The problem is, whenever a website tries to approach the service, your browser will deny/ignore requests due to service's unverified SSL. You just need to open `https://127.0.0.1:64443/` once in your default browser and add its certificates to trusted, so other windows (website that use e-imzo) can connect to websockets. If you don't understand what I'm explaining, just [click this](https://letmegooglethat.com/?q=trust+website+certificate+in+browser).

### Available Options

Please, refer to the example nix showcase below for more information:

```nix
{
  # Here are available options
  services.e-imzo = {
    # Enable Toggle
    # => Mandatory
    enable = true;

    # ID Card support (experimental)
    # => Optional
    id-card = false;

    # User for launching service
    # => Optional
    user = "negir";

    # Group of user for launching service
    # => Optional
    group = "negirlar";

    # E-IMZO custom package
    # => Optional
    package = pkgs.<?>;
  };
}
```

## Thanks

To whoever participated in packaging a closed source piece of shit.

- [Orzklv](https://github.com/orzklv) - Maintainer
- [Shakhzod Kudratov](https://github.com/shakhzodkudratov) - Active tester & debugger

## Wall of shame

- [Yt.uz developers](https://yt.uz) - for developing malware that consumes gigs of operating memory and shamelessly monopolising identity management & database of uzbek tax-payers!

## License

This project is licensed under the CC-BY-4.0 license due to stricted use of [Soliq.uz](https://soliq.uz)'es policy - see the [LICENSE](LICENSE) file for details.

<p align="center">
    <img src=".github/assets/footer.png" alt="Xinux'es {E-IMZO}">
</p>
