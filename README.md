# cs2-haskell

Epitech Lambdananas Coding Style Helper

## Features

- Easily install [lambdananas](https://github.com/Epitech/lambnananas) [(see here)](#installingupdating-packages)
- Ignores all errors from files in your `.gitignore`

## Usage

```sh
cs2-haskell
```

#### Flags

Don't ignore `.gitignore` errors (`--no-ignore`)

```sh
cs2-haskell --no-ignore
```

CI mode with `--ci`

(Only GitHub is supported for now)

```sh
cs2-haskell --ci=github
```

## Install

Requirements:
- Rust
- Cargo

### Installing cs2-haskell

Use the `install.sh` script:

```sh
curl -s https://raw.githubusercontent.com/hugoschool/cs2-haskell/main/install.sh | sh
```

You can also clone the repo directly to `/usr/local/share/cs2-haskell` then run `compile.sh`:
```sh
git clone https://github.com/hugoschool/cs2-haskell.git /tmp/cs2-haskell-cs2
sudo mkdir -p /usr/local/share/cs2-haskell
/tmp/cs2-haskell-cs2/compile.sh
sudo mv /tmp/cs2-haskell-cs2 /usr/local/share/cs2-haskell/cs2
```

### Installing/Updating packages

After installing cs2-haskell, you can install `lambdananas` with:

```sh
cs2-haskell install
```

You can update the packages with:
```sh
cs2-haskell update
```

Only need to update a single package? Use `cs2-haskell install/update --package`:
```sh
cs2-haskell install --package lambdananas
cs2-haskell update --package lambdananas
```

Force rebuild/copy (force build even with if there is no update) (`cs2-haskell update` only):
```sh
cs2-haskell update --package lambdananas --force
```
