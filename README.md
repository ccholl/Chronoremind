# Chronoremind
Flusser’s Apparatus

_Version1_
# Chronoremind 

Anti-Reminder Tool | Experimental Suggestions | Cross-Platform

## Features

- **Cross-Platform** - Windows/macOS/Linux support
- **Time Parsing** - Natural language format (`+2h30m`/`tomorrow 10am`)
- **Suggestions** - Integrated DeepSeek API for experimental recommendations
- **Local Database** - SQLite storage for chronoreminders
- **Desktop Notifications** - System-level alerts
- **Lightweight** - Statically compiled binaries with zero dependencies

## Quick Start 

### Installation

#### Pre-built Binaries
Download from [GitHub Releases](https://github.com/ccholl/chronoremind /releases):

```bash
# Linux
curl -LO https://github.com/ccholl/chronoremind/releases/latest/download/chronoremind-x86_64-unknown-linux-gnu
chmod +x remindai-x86_64-unknown-chronoremind linux-gnu

# macOS
curl -LO https://github.com/ccholl/chronoremind/releases/latest/download/chronoremind-x86_64-apple-darwin
chmod +x remindai-x86_64-apple-darwin

# Windows
Invoke-WebRequest -Uri https://github.com/ccholl/chronoremind/releases/latest/download/chronoremind-x86_64-pc-windows-gnu.exe -OutFile chronoremind.exe
```

#### Build from Source
Requires [Rust toolchain](https://www.rust-lang.org/tools/install):

```bash
cargo install --git https://github.com/ccholl/chronoremind
```

### Basic Usage

```bash
# Create reminder (relative time)
chronoremind create "+2h30m" "Project status meeting"

# Create reminder (absolute time)
chronoremind create "2024-06-01T09:00:00Z" "Monthly review"

# List all reminders
chronoremind list

# Example Output
✓ chronoremind #1 created
══════════ AI Suggestion ══════════
Consider preparing materials 15 minutes early and checking network connection
═══════════════════════════════════
```

## Configuration ⚙️

### 1. Get API Key
1. Visit [DeepSeek Console](https://platform.deepseek.com/)
2. Create new application and get API key

### 2. Set Environment Variables
Create `.env` file in project root:

```env
DEEPSEEK_API_KEY=your_api_key_here
```

## Technical Architecture 

| Component         | Technology              |
|-------------------|-------------------------|
| Language          | Rust 2021 Edition       |
| Async Runtime     | Tokio                   |
| Database          | SQLite                  |
| CLI Parsing       | Clap 4.4                |
| Time Handling     | Chrono + humantime      |
| HTTP Client       | reqwest                 |

---
**Start play with your time!**  
For questions, please [open an issue](https://github.com/ccholl/chronoremind /issues) or contact ccholl716@gmail.com

---


_Version2_
# Chronoremind - Temporal Scribe

> "programmed machines about time"

## ∮ Archaeology of Apparatus

This project embodies Flusser's theory of "technical apparatus" through digital praxis. We construct not a mere reminder tool, but a **temporally-aware meta-apparatus**. The database becomes a "technological stomach" for memory storage, the async runtime forms a "cognitive intestine" processing temporality, while the AI suggestion system manifests what Flusser called "programmed imagination machines".

### Philosophical Dimensions in Technical Features:
- **UTC Standardization**: Technical compromise revealing digital time's colonization of biological duration (parseable strings vs lived temporality)
- **Irreversible Deletion**: Exposing digital storage's materiality through the paradox of "permanent memory" vs finite memory
- **AI Oracle System**: Demonstrating Flusser's "programmed serendipity" - algorithms as new prophets

```rust
// Flusserian reading: This async block represents the "black box" of technical apparatus, its timing mechanism reshaping human perception of "waiting"
tokio::spawn(async move {
    time::sleep(d).await; // Mechanical suspension of time
    Notification::new().body(&message).show() // Digital summoning of flesh
});
```

## λ Techno-Phenomenology

### Core Paradox Implementations:
1. **Self-Observation Paradox**  
   Through temporal differential calculations in `list_reminders`, the tool becomes a Heideggerian "present-at-hand", forcing confrontation with:
   ```rust
   let remain = trigger_utc - now; // Quantitative violence of digital time upon existence
   ```
2. **Invisible Interactivity**  
   Deliberately austere CLI interface responds to Flusser's critique of "interface tyranny". The hidden AI suggestion system in `create_reminder` constitutes a microcosm of his warned "technological subconscious"

3. **Modular Epistemic Isolation**  
   Each Rust module forms autonomous "technical black boxes" (Flusser's term):
   ```
   scheduler::schedule() // Temporal discipline apparatus
   deepseek::get_simple_advice() // Algorithmic unconscious
   db::delete_reminder() // Digital oblivion ritual
   ```

## ⟳ Technologies of the Self

The ultimate goal is Foucauldian "self-cultivation" through technical praxis:

### Traces of Asceticism in Code:
- **Ownership System** as technical discipline:  
  ```rust 
  .bind(message) // Semiotic incarceration of information
  ```
- **Error Handling Chain** as epistemic prism:
  ```rust
  .context("Failed to parse API response as JSON")?; // Breakdown as revelation
  ```
- **Async Runtime** as temporal zendo:
  ```tokio::main``` annotation constructs mechanized meditation space

## Initiation Ritual

```bash
DEEPSEEK_API_KEY=your_philosophical_password cargo run -- create "+1h" "Interrogate technology's ontological presuppositions"
```

_ORIGINAL NOTE_
- **Rust Technical Implementation — How to Balance "Ideas" and "Technology"**
    - **Viewing (Presentation)**
    - **Iteration — Primarily self-iteration. A feedback mechanism is crucial, but I’m unsure how to implement it technically.**
    - **Digital itself is a "device" — Searchability — Currently unachievable in my implementation.**
    - **"Logic"**
        - *Filter*
            - *Why abandoned: Found it too difficult in practice.*
        - *Summary*
            - *Should be included, but due to limited skills, it was omitted. Writing in Rust affects the entire system—adding one feature requires adjusting others. No monitoring/feedback functionality remains.*
        - *Edit*
            - *Why abandoned: Already added a database feature, so prioritized keeping input as simple as possible.*
            - *Manipulating this in the database is too challenging for my current skill level.*
        - *Tag*
            - *Abandoned: Felt that "message" fully covers my needs. No classification system required.*
        - *Message*
            - *A notify: **Raise "self"-related issues***
        - *Time parsing simplified to UTC format to reduce complexity.*
        - *Delete?*
            - *Ideally, deletion is unwanted. But technically, omitting it risks excessive memory usage.*
    - **Modular design — For future scalability.**
    - **Key Technology Involved: Asynchrony**
        
        > When working with async in Rust, we are always dealing with concurrency. Depending on the hardware, the operating system, and the async runtime we are using—more on async runtimes shortly!—that concurrency may also use parallelism under the hood.
        
    - **Rust Error Handling — Realized its critical importance during coding.**
        - [Ownership and Lifetimes](https://www.notion.so/18ab8ea0212c805d8c7be6ef1f859860?pvs=21)
        - [Syntax](https://www.notion.so/18ab8ea0212c8010bc37d34f1dc82a13?pvs=21)
    - **I/O**
    - [Future Technical Improvements](https://www.notion.so/18ab8ea0212c80bd9b73d91e37b57ce4?pvs=21)

- **Why did I initially think of creating this "tool"?**
    - **Love for documenting.**
        - *What’s the difference between digital documentation and pen-and-paper documentation?*
            - **Searchability**
            - **Digital itself is already a "device."**
                - Like a "map."
    - **Observing changes in the "self" over time.**
        - ***Time***
            - *About myself: I’m better at handling urgent problems (those with immediate feedback) but weaker at managing long-term tasks.*
                
                > "Most of us spend too much time on what is urgent and not enough time on what is important."
                > 
            - *I want to experiment: Does adding the parameter of "time" truly influence behavior?*
        - **Behavior**
        - *My own issue: Explore and construct. So, let AI provide actionable suggestions.*
    - **Many ideas arise** ***unintentionally***.
    - **What steps are needed to nurture an idea into action?**
    - **How do we truly** ***interact*** **with tools?**
        - *Human-machine interaction*
            - *I don’t want endless so-called "smart" features.*
        - ***The Whole Earth Catalog***
    - **A thought experiment: In a purely physical world, how do we interact with our environment? and digital world?**
    - **We don’t need a tool that only serves predefined functions.**
    - ***The ultimate purpose of tools is to aid thinking/perceiving the world and enable creation.***
        - ***Interactive Narrative Tech***
        - ***Develop one’s own mental models***
