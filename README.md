<p align="center">
    <img src=".github/assets/header.png" alt="Xinux'es {E-IMZO}">
</p>

<p align="center">
    <h3 align="center">Various project templates for faster bootstrapping with Nix.</h3>
</p>

<p align="center">
    <img align="center" src="https://img.shields.io/github/languages/top/xinux-org/templates?style=flat&logo=nixos&logoColor=5277C3&labelColor=ffffff&color=ffffff" alt="Top Used Language">
    <a href="https://t.me/xinux"><img align="center" src="https://img.shields.io/badge/Chat-grey?style=flat&logo=telegram&logoColor=5277C3&labelColor=ffffff&color=ffffff" alt="Telegram Community"></a>
</p>

## About

This is Uzbek Xinux community (nix users mostly) member's effort on packaging E-IMZO & providing ready to use modules for NixOS users.

> [!CAUTION]
> Think thrice before using this software. We had to let the service have access to memory segments to make it work. It refuses to work in sandboxed environments. Apart from that, it also consumes huge amount of memory heaps to run the service continuously. **We don't take any responsibility** for whatever this software does to you and your computer. We just provide packaging support for this piece of garbage as community demands it.

> [!NOTE]
> Due to E-IMZO's malicious behavior, we won't be adding this software to [nixpkgs](https://github.com/NixOS/nixpkgs) nor support if someone attempts to.

## Guides & Use

This project effort provides you both E-IMZO as a package and ready to use nix modules. In order to get started, you need to add this flake to your own config:

### Package

If you want to use the package one time, you can easily call the package via `nix run`:

```shell
nix run github:xinux-org/e-imzo
```

If you're going to add this package to your own configuration, we provide `e-imzo` binary for every arch at:

```
inputs.e-imzo.packages.x86-64-linux.default
inputs.e-imzo.packages.aarch64-linux.default
inputs.e-imzo.packages.x86-64-darwin.default
inputs.e-imzo.packages.aarch64-darwin.default
```

Yes, technically you can run this software in your **MacOS** too~!

### Service Module (configuration use)

```nix
# In your configuration repo flake.nix
{
  inputs.e-imzo.url = "github:xinux-org/e-imzo";
}
```

And now,

## Thanks

To whoever participated in packaging a closed source piece of shit.

- [Orzklv](https://github.com/orzklv) - Maintainer
- [Shakhzod Kudratov](https://github.com/shakhzodkudratov) - Active tester & debugger

## Wall of shame

- [Yt.uz developers](https://yt.uz) - for developing malware that consumes gigs of operating memory and shamelessly monopolising identity management & database of uzbek tax-payers!

## License

This project is licensed under the CC4 license due to stricted use of [Soliq.uz](https://soliq.uz)'es policy - see the [LICENSE](license) file for details.

<p align="center">
    <img src=".github/assets/footer.png" alt="Xinux'es {E-IMZO}">
</p>

## Imperative distro retards

Read e-imzo devs provided doc: [README.txt](.github/guides/README.txt)

## Options

```nix
# In your flake.nix
{
  inputs.e-imzo.url = "github:xinux-org/e-imzo";
}

# Somewhere in your nix configs
{
  imports = [inputs.e-imzo.nixosModules.e-imzo];

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
