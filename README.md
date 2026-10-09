# rust-sci-calc

Secure scientific calculator with a GUI, written in safe Rust for Windows, Linux and macOS.

Ein wissenschaftlicher Taschenrechner mit grafischer Oberfläche, geschrieben in Rust.

Läuft unter Windows, Linux und macOS.

![Screenshot](docs/screenshot.png)

## Funktionen

- Grundrechenarten, Klammern, Potenzen (`^`), Modulo (`mod` / `%`), Fakultät (`!`)
- sin, cos, tan, asin, acos, atan, sinh, cosh, tanh
- Wurzel (`√` / `sqrt`), Kubikwurzel (`cbrt`), ln, log (Basis 10), log2, exp
- abs, round, floor, ceil
- Konstanten π (`pi`) und e, letztes Ergebnis mit `ans`
- Umschaltung Grad (DEG) / Bogenmaß (RAD)
- Speicher (M+, MR, MC) und ein Verlauf (anklicken fügt das Ergebnis ein)
- Direkt per Tastatur tippen: Enter rechnet, Esc löscht
- Dezimalkomma und Dezimalpunkt funktionieren beide, `2pi` oder `3(4+1)` werden als Multiplikation verstanden

## Sicherheit

- `#![forbid(unsafe_code)]`: im eigenen Code ist kein `unsafe` erlaubt, der Compiler verweigert es.
- Eigener Parser (`src/parser.rs`) statt einer `eval`-Bibliothek: Eingaben werden nur als
  Rechenausdruck gelesen, niemals als Code ausgeführt.
- Begrenzungen gegen bösartige oder riesige Eingaben: max. 1000 Zeichen, max. 100 Ebenen
  Verschachtelung (kein Stack-Überlauf), Fakultät nur bis 170.
- Fehler (Division durch null, Wurzel aus negativer Zahl, …) werden als Meldung angezeigt,
  das Programm stürzt nicht ab.
- Nur eine direkte Abhängigkeit: `eframe`/`egui`, ein weit verbreitetes, reines Rust-GUI-Framework.
  `Cargo.lock` hält alle Versionen fest.
- Release-Build mit Überlaufprüfung (`overflow-checks = true`).
- Keine Netzwerkzugriffe, keine Dateien, keine gespeicherten Daten.

## Fertige Programme herunterladen

Unter [Releases](https://github.com/Psyc0Duck/rust-sci-calc/releases) gibt es fertige Dateien, Rust ist dafür nicht nötig:

- **Windows:** `Taschenrechner-Setup.exe` installiert das Programm für deinen Benutzer (ohne Administratorrechte)
  mit Startmenü-Eintrag und Deinstallation über die Windows-Einstellungen. Alternativ läuft
  `taschenrechner.exe` auch direkt ohne Installation.
- **Linux:** `taschenrechner-linux-x86_64.tar.gz`
- **macOS:** `taschenrechner-macos.zip` (Apple Silicon und Intel)

Die Dateien sind nicht signiert. Windows SmartScreen zeigt deshalb beim ersten Start
„Der Computer wurde durch Windows geschützt“: über „Weitere Informationen“ und „Trotzdem ausführen“
lässt es sich starten. macOS blockiert heruntergeladene, unsignierte Programme ebenfalls; dort
im Finder per Rechtsklick „Öffnen“ wählen. Die Prüfsummen stehen in `SHA256SUMS.txt`.

Eine neue Version veröffentlichen: Version in `Cargo.toml` erhöhen, dann einen Tag pushen, z. B.
`git tag v0.2.0 && git push origin v0.2.0`. GitHub baut daraufhin alles automatisch.

## Voraussetzung: Rust installieren

### Windows

1. <https://rustup.rs> öffnen und `rustup-init.exe` herunterladen und starten.
2. Wenn der Installer fragt, die **Visual Studio Build Tools** (C++-Workload) mitinstallieren lassen.
3. Neues Terminal (PowerShell) öffnen und prüfen: `cargo --version`

### Linux (Ubuntu/Debian)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt install build-essential libxkbcommon-x11-0 libgl1
```

Bei Fedora entsprechend `sudo dnf install gcc libxkbcommon-x11 mesa-libGL`.

### macOS

1. Im Terminal die Apple-Entwicklerwerkzeuge installieren: `xcode-select --install`
2. Rust installieren:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
3. Neues Terminal öffnen und prüfen: `cargo --version`

Funktioniert auf Apple-Silicon- (M1 und neuer) und Intel-Macs.

## Bauen und starten

Im Projektordner (dort, wo `Cargo.toml` liegt):

```bash
cargo run --release
```

Der erste Build lädt die Abhängigkeiten und dauert einige Minuten, danach geht es schnell.
Das fertige Programm liegt anschließend hier und kann ohne Rust gestartet werden:

- Windows: `target\release\taschenrechner.exe`
- Linux und macOS: `target/release/taschenrechner`

Unter macOS startet das Programm am einfachsten aus dem Terminal. Da du es selbst gebaut hast,
blockiert Gatekeeper es nicht.

## Tests

```bash
cargo test
```

Optional die Abhängigkeiten auf bekannte Sicherheitslücken prüfen:

```bash
cargo install cargo-audit
cargo audit
```

## Aufbau

- `src/parser.rs`: Zerlegt und berechnet Ausdrücke, inklusive Tests.
- `src/main.rs`: Die Oberfläche (Anzeige, Tastenfeld, Verlauf).

## Lizenz

Apache-2.0, siehe [LICENSE](LICENSE).
