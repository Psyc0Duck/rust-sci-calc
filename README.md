# rust-sci-calc

Secure scientific calculator with a GUI, written in safe Rust for Windows, Linux and macOS.

Ein wissenschaftlicher Taschenrechner mit grafischer Oberfläche, geschrieben in Rust.
Er läuft unter Windows, Linux und macOS.

![Screenshot](docs/screenshot.png)

## Funktionen

- Grundrechenarten, Klammern, Potenzen (`^`), Modulo (`mod` bzw. `%`) und Fakultät (`!`)
- sin, cos, tan, asin, acos, atan, sinh, cosh, tanh
- Wurzel (`√` bzw. `sqrt`), Kubikwurzel (`cbrt`), ln, log (Basis 10), log2, exp
- abs, round, floor, ceil
- Konstanten π (`pi`) und e, das letzte Ergebnis mit `ans`
- Umschaltung zwischen Grad (DEG) und Bogenmaß (RAD)
- Speicher (M+, MR, MC) und ein Verlauf; ein Klick auf einen Eintrag fügt das Ergebnis ein
- Eingabe auch direkt über die Tastatur: Enter rechnet, Esc löscht
- Dezimalkomma und Dezimalpunkt funktionieren beide; `2pi` oder `3(4+1)` gelten als Multiplikation

## Herunterladen und installieren

Fertige Programme gibt es unter [Releases](https://github.com/Psyc0Duck/rust-sci-calc/releases).
Rust ist dafür nicht nötig.

### Windows

- **`Taschenrechner-Setup.exe`** installiert den Taschenrechner nur für deinen Benutzer, ohne
  Administratorrechte. Er erscheint im Startmenü und lässt sich über
  *Einstellungen → Apps* wieder entfernen.
- **`taschenrechner.exe`** läuft auch ohne Installation, einfach doppelklicken.

Die Dateien sind nicht digital signiert. Beim ersten Start meldet Windows deshalb
„Der Computer wurde durch Windows geschützt“. Klicke auf **Weitere Informationen** und dann auf
**Trotzdem ausführen**.

### Linux

```bash
tar -xzf taschenrechner-linux-x86_64.tar.gz
./taschenrechner
```

Benötigt werden `libxkbcommon-x11` und OpenGL, die auf normalen Desktop-Systemen schon vorhanden
sind. Falls nicht (Ubuntu/Debian): `sudo apt install libxkbcommon-x11-0 libgl1`

### macOS

`taschenrechner-macos.zip` läuft auf Macs mit Apple-Chip (M1 und neuer) und mit Intel-Chip.
Da das Programm nicht von Apple beglaubigt ist, blockiert macOS es nach dem Herunterladen.
Am einfachsten startest du es im Terminal:

```bash
cd ~/Downloads
unzip taschenrechner-macos.zip
xattr -d com.apple.quarantine taschenrechner
./taschenrechner
```

### Echtheit prüfen

Jedes Release enthält `SHA256SUMS.txt` mit den Prüfsummen aller Dateien. So prüfst du eine Datei:

- Windows (PowerShell): `Get-FileHash .\Taschenrechner-Setup.exe`
- Linux: `sha256sum taschenrechner-linux-x86_64.tar.gz`
- macOS: `shasum -a 256 taschenrechner-macos.zip`

Der angezeigte Wert muss mit dem in `SHA256SUMS.txt` übereinstimmen.

## Selbst bauen

### 1. Rust installieren

**Windows**

1. Auf <https://rustup.rs> `rustup-init.exe` herunterladen und starten.
2. Wenn der Installer danach fragt, die **Visual Studio Build Tools** (Workload „Desktopentwicklung
   mit C++“) mitinstallieren lassen.
3. Ein neues Terminal (PowerShell) öffnen und prüfen: `cargo --version`

**Linux (Ubuntu/Debian)**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install build-essential libxkbcommon-x11-0 libgl1
```

Unter Fedora heißen die Pakete `gcc libxkbcommon-x11 mesa-libGL` (`sudo dnf install …`).

**macOS**

```bash
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Danach ein neues Terminal öffnen und mit `cargo --version` prüfen.

### 2. Bauen und starten

Im Projektordner, also dort, wo `Cargo.toml` liegt:

```bash
cargo run --release
```

Der erste Build lädt alle Abhängigkeiten herunter und dauert einige Minuten, danach geht es schnell.
Das fertige Programm liegt anschließend unter

- Windows: `target\release\taschenrechner.exe`
- Linux und macOS: `target/release/taschenrechner`

und lässt sich auch ohne Rust starten. Selbst gebaute Programme blockiert macOS nicht.

### 3. Tests

```bash
cargo test
```

## Änderungen beitragen

Der Branch `main` ist geschützt: Änderungen kommen nur über einen Pull Request hinein, und dieser
lässt sich erst mergen, wenn alle automatischen Prüfungen grün sind:

| Prüfung | Was sie macht |
|---|---|
| `test` | Formatierung (`cargo fmt`), Lint (`cargo clippy`) und Tests (`cargo test`) |
| `windows`, `linux`, `macos` | baut das Programm für jede Plattform, unter Windows samt Setup |
| `audit` | prüft die Abhängigkeiten auf bekannte Sicherheitslücken (bei Änderungen an `Cargo.toml`/`Cargo.lock` und jeden Montag) |

Die fertig gebauten Dateien jedes Laufs findest du unter
[Actions](https://github.com/Psyc0Duck/rust-sci-calc/actions) beim jeweiligen Lauf unter **Artifacts**.

Vor einem Pull Request lokal prüfen:

```bash
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

Dependabot schlägt jede Woche Updates für Abhängigkeiten als eigene Pull Requests vor. Sind deren
Prüfungen grün, kannst du sie mergen.

## Neue Version veröffentlichen

1. **Versionsnummer erhöhen.** In `Cargo.toml` die Zeile `version = "0.1.0"` anpassen, zum Beispiel
   auf `0.2.0`. Danach einmal `cargo build` ausführen, damit `Cargo.lock` die neue Nummer übernimmt.
   Beide Dateien per Pull Request auf `main` bringen und mergen.
2. **Release anlegen.** Auf GitHub
   [Releases → Draft a new release](https://github.com/Psyc0Duck/rust-sci-calc/releases/new) öffnen.
   Bei **Choose a tag** die neue Version mit `v` davor eintippen, zum Beispiel `v0.2.0`, und
   **Create new tag** wählen. Titel eintragen, **Generate release notes** und dann
   **Publish release** klicken.
3. **Warten.** GitHub baut jetzt alle Programme und hängt sie nach etwa 5 bis 10 Minuten an das
   Release an: Setup und exe für Windows, Linux- und macOS-Datei sowie `SHA256SUMS.txt`.
   Den Fortschritt siehst du unter [Actions](https://github.com/Psyc0Duck/rust-sci-calc/actions).

Statt Schritt 2 geht auch `git tag v0.2.0 && git push origin v0.2.0`; GitHub legt das Release
dann selbst an.

## Sicherheit

- Im eigenen Code ist `unsafe` verboten (`#![forbid(unsafe_code)]`); der Compiler verweigert es.
- Ein eigener Parser (`src/parser.rs`) liest Eingaben nur als Rechenausdruck. Es gibt kein `eval`,
  nichts davon wird als Code ausgeführt.
- Grenzen gegen bösartige oder riesige Eingaben: höchstens 1000 Zeichen und 100 Ebenen
  Verschachtelung, Fakultät nur bis 170. Damit kann keine Eingabe den Stack sprengen oder
  das Programm lange blockieren.
- Rechenfehler wie Division durch null erscheinen als Meldung, das Programm läuft weiter.
- Nur eine direkte Abhängigkeit (`eframe`/`egui`, ein verbreitetes GUI-Framework in reinem Rust).
  `Cargo.lock` legt alle Versionen fest.
- Der Release-Build prüft Ganzzahl-Überläufe auch im fertigen Programm (`overflow-checks = true`).
- Kein Netzwerkzugriff, keine Dateizugriffe, keine gespeicherten Daten.

Sicherheitslücken bitte vertraulich melden, siehe [SECURITY.md](SECURITY.md).

## Projektaufbau

| Pfad | Inhalt |
|---|---|
| `src/parser.rs` | zerlegt und berechnet Ausdrücke, mit Tests |
| `src/main.rs` | die Oberfläche: Anzeige, Tastenfeld, Verlauf |
| `installer/windows.nsi` | Skript für das Windows-Setup ([NSIS](https://nsis.sourceforge.io)) |
| `.github/workflows/build.yml` | Prüfungen, Builds und Releases |
| `.github/workflows/audit.yml` | Sicherheitsprüfung der Abhängigkeiten |
| `.github/dependabot.yml` | automatische Update-Vorschläge |

## Lizenz

Apache-2.0, siehe [LICENSE](LICENSE).
