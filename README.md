# Requirements
- STM32F3 Discovery Board + Mini USB Cable
- Rust Compiler mit "thumb7em-none-eabihf" Target installiert
- probe-rs installiert
- ST-Link Treiber installiert

# Installation

Dies ist keine Schritt für Schritt vollständige Anleitung! Es zeigt die groben Schritte, im Detail kann aber mehr notwendig sein. Kann aber alles mit google gelöst werden!

1) Installation rustup (Tool zum Installieren/Updaten des Rust Compilers)
https://www.rust-lang.org/learn/get-started 

Rustup-init.exe 64 Bit runterladen und ausführen (dabei muss unter Umständen wie beschrieben "Visual Studio C++ Build tools" installiert werden)

2) MCU Target hinzufügen
In CMD folgendes ausführen:

rustup target add thumbv7em-none-eabihf 

Erlaubt das Cross compilen für dieses Target

thumbv7em-none-eabihf => Triplet für den Cortex-M4 mit FPU

thumbv7em => Instruction Set ("Kleines Embedded Instruction Set" von Arm)

Fun Fact:

Arm Instruction Set => Arm wie der Arm

Thumb Instruction Set => Wie der Daumen daher kleiner als Hand

none => kein hersteller

eabi(hf) => "Embedded Application Binary Interface" ... HF keine Ahnung warum steht aber für FPU support

3) probe-rs Installieren
In CMD folgendes ausführen:
cargo install probe-rs

Installiert das Programm probe-rs => Erkennt und verwendet Flash/Debug Probes wie JLink, ST-Link etc.
Damit kann direkt auf das Board geflasht werden

# Example

Nachdem alles installiert wurde kann im Root des Repos foglender Befehl ausgeführt werden:

## Minimal Beispiel
cargo run --bin minimal
## Beispiel mit ein wenig mehr Abstraktion - PAC Crates
cargo run --bin pac
## Noch mehr Abstraktion mit HAL und embedded HAL
cargo run --bin hal

alle Befehle können mit dem zusatz --release als maximal optimierte Variante gebaut werden (besonders interessant für das minimal Beispiel)



Alle Beispiele sollten auf dem angeschlossene Board die orangene LED mit ca. 1Hz blinken (Überraschung bei minimal ohne Release inbegriffen)
