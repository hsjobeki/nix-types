<!-- markdownlint-disable MD013 -->
# nix-types RFC (draft)

This Draft of an RFC could be the first step to improve how nix is used as a language.

__:construction: :construction: Any help welcome! :construction: :construction:__

<div align="center">
  <br/>
  Start problem solving for nix with types
  <br/>
 Contribute to the | <a href="https://typednix.dev/">Specification</a>
 <br/>
  discussion on | <a href="https://matrix.to/#/#nix-types:matrix.org">matrix<a>
  <br/>
  <br/>
</div>

## The Idea

![type-system](./Types.drawio.svg)

### Standard for current doc-strings

A very large set of doc-strings is already available in `nixpkgs`.

But they don't follow `strict` conventions and thus it is very hard to write a parser for them. There is highly valuable information inside them:

Example:

#### stdenv.mkDerivation

| Required  | Attribute | Type  | Default | Description |
| ---       | ---       |---    |---      | --- |
| ✅ | name | String | | The name of the `Derivation` |
| ❓ | builder | String | "setup.sh" | Uses the predefined `setup.sh` script. Usually you want to define scripts for the different `phases`. |
| ❓ | buildInputs | List[Derivation] | | List of Packages needed inside the build-sandbox of this Derivation. Will be built beforehand and made available in $PATH of that build. e.g `python` is needed for python command executed during build time. |
| ❓ | buildPhase | String | | String is interpreted and executed as bash script. Use magic quotes '' {script} '' for multiline scripts|

Defined in: __pkgs/stdenv/generic/make-derivation.nix__

The full content is available as mdbook [here](https://typednix.dev).
