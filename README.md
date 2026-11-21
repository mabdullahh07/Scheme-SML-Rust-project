# COSC 455 – Project 2: Functional Programming

Programming Languages: Design and Implementation, Fall 2026

Small functional programs written in **SML**, **Rust**, and **Scheme (Racket)**, comparing how each language handles recursion, pattern matching, lists, and higher-order functions.

## Contents

| Part | Topic | Files |
|---|---|---|
| I | SML & Rust warmup: `hitchhiker`, `everyother`, `goldbach` | `warmup.sml`, `warmup.rs` |
| II | Binary addition and subtraction (two's complement) | `binary.sml`, `binary.rs` |
| III | Pinyin/English number translator in Rust | `translate.rs` |
| IV | Scheme prep: `third`–`fifth`, `truecount`, `squarelist`, `hundreds?`, `collatz` | `prep.rkt` |
| V | Pinyin/English number translator in Scheme | `translate.rkt` |
| VI | Functional programming reflection | `COSC455_Project2_2026.docx` |
| VII | Extra credit: Drunk Homer logic puzzle (Rust) | `homer.rs` |

The full write-up (code, code understanding answers, screenshots, and reflection) is in `COSC455_Project2_2026.docx`.

## How to run

**SML:** paste the `.sml` file into [SOSML](https://sosml.org) and run it.

**Rust:** paste the `.rs` file into the [Rust Playground](https://play.rust-lang.org) and click Run, or locally:

```
rustc translate.rs && ./translate
```

**Scheme:** open the `.rkt` file in [DrRacket](https://racket-lang.org) (language set to "Determine language from source") and click Run.

## Playground links

| Part | SOSML | Rust Playground |
|---|---|---|
| I – Warmup | [link](https://sosml.org/share/888c9b55a816e1154491a582cf3d75287def31e0d49a3e73ac1f9538192ff0be) | [link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=cd0a062c42561932da81e272d7f47bba) |
| II – Binary | [link](https://sosml.org/share/c0959ef40cfa4854cd9179bf37fd27a8464ce2a1812691d1e175d402f96a2cb5) | [link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=01900d5d8e6f3cdcfe58c4ec259e4f94) |
| III – Translator | – | [link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=4350941b25b04891c6a89d7dddf37878) |
| VII – Homer | – | [link](https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=12915234ec9bafea333cf09880f2cb19) |

## Example output (translator)

```
Translation: 1 9 6 8
Addition: 1 + 9 + 6 + 8 = 24
Multiplication: 1 * 9 * 6 * 8 = 432
```

## AI use

AI tools were used to help with design and development, as allowed by the project rules. Transcripts are linked in the project document.
