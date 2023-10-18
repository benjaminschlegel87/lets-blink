# Intro
Was ist macht man zuerst in einer neuen Programmiersprache???

Natürlich ein "Hello World" Beispiel. Was ist das "Hello World" der embedded Welt?

Eine LED Blinken... Daher sind hier 5 verschiedene Wege in Rust eine LED zu blinken

# Requirements
- STM32F3 Discovery Board + Mini USB Cable
- Rust Compiler mit "thumb7em-none-eabihf" Target installiert
- probe-rs installiert
- ST-Link Treiber installiert

# Installation

Dies ist keine Schritt für Schritt vollständige Anleitung! Es zeigt die groben Schritte, im Detail kann aber mehr notwendig sein. Kann aber alles mit google gelöst werden!

1) **Installation rustup (Tool zum Installieren/Updaten des Rust Compilers)**


https://www.rust-lang.org/learn/get-started 

Rustup-init.exe 64 Bit runterladen und ausführen (dabei muss unter Umständen wie beschrieben "Visual Studio C++ Build tools" installiert werden)

2)  **MCU Target hinzufügen**

In CMD folgendes ausführen:

```
rustup target add thumbv7em-none-eabihf 
```

Erlaubt das Cross compilen für dieses Target

thumbv7em-none-eabihf => Triplet für den Cortex-M4 mit FPU

thumbv7em => Instruction Set ("Kleines Embedded Instruction Set" von Arm)

Fun Fact:

Arm Instruction Set => Arm wie der Arm

Thumb Instruction Set => Wie der Daumen daher kleiner als Hand

none => kein hersteller

eabi(hf) => "Embedded Application Binary Interface" ... HF keine Ahnung warum steht aber für FPU support

3) **probe-rs Installieren**

In CMD folgendes ausführen:
```
cargo install probe-rs
```

Installiert das Programm probe-rs => Erkennt und verwendet Flash/Debug Probes wie JLink, ST-Link etc.
Damit kann direkt auf das Board geflasht werden

# Example

Nachdem alles installiert wurde kann im Root des Repos foglender Befehl ausgeführt werden:

## Minimal Beispiel - Nur für Raw Pointer Freunde

Für die echten harten Kerle die Variante mit nur Raw Pointern. Zeigt das Rust eine Low Level Variante ohne (fast ohne) Runtime ist. 
```
cargo run --bin minimal
```
## Beispiel mit ein wenig mehr Abstraktion - PAC Crate
PAC = Peripheral Access Crate
Diese Crate wird aus dem SVD File des MCU automatisch mit dem SVD2Rust tool generiert. Gibt einem rudimentäre Access Funktionen für die Register. Hier muss schon nicht mehr mit Addressen, Pointern oder Masken gearbeitet werden
```
cargo run --bin pac
```
## Noch mehr Abstraktion mit HAL und embedded HAL
Eine HAL Crate setzt auf die entsprechen PAC Crate auf und stellt damit typische Funktionen direkt zur Verfügung ohne das man sich noch mit Registern beschäftigen muss. Eine HAL Crate implementiert typischerweise die Traits der Crate (embedded-hal)[https://docs.rs/embedded-hal/latest/embedded_hal/].
```
cargo run --bin hal
```
## Darf es noch mehr Abstraktion sein? BSP!
Das Board Support Package setzt auf der HAL auf und definiert Funktionen die das konkrete Board zur Verfügung stellt. Hier das STM32F3DISCOVERY.
```
cargo run --bin bsp
```
## Now it gets crazy - Blinking async
Wer kein Bock auf händisches pollen mit Statemachines hat und blockierende Funktionen Kacke findet kommt hier auf seine kosten.
```
cargo run --bin async
```

Alle Befehle können mit dem zusatz --release als maximal optimierte Variante gebaut werden (besonders interessant für das minimal Beispiel)

Alle Beispiele sollten auf dem angeschlossene Board die orangene LED mit ca. 1Hz blinken (Überraschung in Abhängigkeit von --release inbegriffen)
c

## Binutils

Die ganz Neugierigen können sich noch die cargo-binutils installieren!
Was ist cargo-binutils
> Cargo subcommands to invoke the LLVM tools shipped with the Rust toolchain

Installieren mit:
```
cargo install cargo-binutils
```

Im root des repos kann dann mit folgendem Befehl die Größe des Binär Files ausgegeben werden:
```
cargo size --bin minimal
```

oder mit dem zusatz --release

```
cargo size --bin minimal --release
```
