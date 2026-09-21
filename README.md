# Cyber-Abhidhamma

[![Test](https://github.com/suchanon456/Cyber-Abhidhamma/actions/workflows/test.yml/badge.svg)](https://github.com/suchanon456/Cyber-Abhidhamma/actions/workflows/test.yml)
[![License](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)

> **Mind as Computer · Karma = Algorithm · Rūpa-Paramattha = OOP Simulation**

**Abhidhamma Cyber** is a framework that maps Buddhist Abhidhamma concepts into computable system architectures — from data structures and state machines to cybersecurity, operating systems, and AI.

It is not a metaphor. It is an attempt to **reverse-engineer the Abhidhamma as an executable specification**, then re-implement it as software.

---

## 🌱 Why this exists

Modern AI systems — Neural Networks, LLMs, agents — have a fundamental problem:

> **They cannot explain themselves.**

They hallucinate. They cannot say *why* they made a decision. They have no language for their own internal states.

The Abhidhamma is the most detailed map of mind ever produced — 89/121 cittas, 52 cetasikas, 17/15 thought-moments, causal chains (Paṭiccasamuppāda), and the 8-fold path.

**If we can encode it, we can give AI a language for its own mind.**

This project is an attempt to do that.

---

## 🧩 Core Mapping

| Abhidhamma | Computer Science |
|---|---|
| Citta (Mind) | Runtime Engine / CPU |
| Cetasika (52) | Behavioral Data Structure / Feature Vector |
| Rūpa-Paramattha (28) | Hardware Virtualization / OOP Simulation |
| Karma | Algorithm / Execution Logic |
| Citta-Vīthi (17/15 moments) | Clock Cycles / State Machine |
| Paṭiccasamuppāda | Causal Execution Graph |
| Nibbāna | Termination of the causal loop |
| Magga 8 | Security Architecture (not just Firewall) |
| Vatta 7 | Admin Governance Layer |

---

## 🏗️ Architecture

```
                    ┌─────────────────────┐
                    │   Paramattha OS     │
                    │  (Rūpa 28 / Kernel) │
                    └──────────┬──────────┘
                               │
              ┌────────────────┼────────────────┐
              ▼                ▼                ▼
        ┌──────────┐    ┌──────────┐    ┌──────────┐
        │ Cetasika │    │  Citta   │    │  Vīthi   │
        │ Vector   │    │  121 FSM │    │ Scheduler│
        │ (52-dim) │    │          │    │ (17/15)  │
        └──────────┘    └──────────┘    └──────────┘
              │                │                │
              └────────────────┼────────────────┘
                               ▼
                    ┌─────────────────────┐
                    │  Paṭiccasamuppāda   │
                    │   Causal Engine     │
                    └──────────┬──────────┘
                               ▼
                    ┌─────────────────────┐
                    │   Magga 8 Security  │
                    │   + Vatta 7 Admin   │
                    └─────────────────────┘
```

---

## 📂 Repository Structure

```
Cyber-Abhidhamma/
├── docs/                              # Human-readable documentation
│   ├── INDEX.md
│   ├── Core-Logic-v1.md
│   ├── Core-Logic-v2.md
│   └── The-Seven-Practices-Vattapada.md
├── spec/                              # Canonical JSON specifications
│   ├── Paramattha-Structure.json
│   └── Vattapada-7.json
├── core/                              # Rust — v2 engine (work in progress)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── citta.rs
│       └── monitor/
│           └── lobha.rs
├── legacy-php/                        # PHP — v1 reference implementation
│   ├── CetasikaVector.php
│   └── VattapadaEngine.php
├── examples/                          # Runnable demos
│   └── vattapada_demo.php
├── tests/                             # Test suites
│   └── VattapadaEngineTest.php
├── .github/workflows/test.yml
├── LICENSE
└── README.md
```

---

## 🚀 Quick Start

### PHP v1 (legacy reference)

```bash
git clone https://github.com/suchanon456/Cyber-Abhidhamma.git
cd Cyber-Abhidhamma
php examples/vattapada_demo.php
```

**Sample output:**

```
=== Vattapada 7 Scores ===
  vatta_1    : 0.580
  vatta_2    : 0.570
  vatta_3    : 0.530
  vatta_4    : 0.800
  vatta_5    : 0.480
  vatta_6    : 0.870
  vatta_7    : 0.480

  Indra Score : 0.587
```

### Rust v2 (in progress)

```bash
cd core
cargo test
```

> **Status:** early prototype. Contributions welcome.

---

## 📚 Documentation

| Document | Description |
|---|---|
| [Documentation Index](./docs/INDEX.md) | Overview of all docs |
| [Core Logic v1](./docs/Core-Logic-v1.md) | Initial mapping: Mind = Computer, Karma = Algorithm |
| [Core Logic v2](./docs/Core-Logic-v2.md) | Expanded: Firewall Magga 8, Vatta 7, Vīthi-Citta |
| [Vattapada 7](./docs/The-Seven-Practices-Vattapada.md) | The Seven Practices Leading to Becoming Sakka |

---

## 📦 Source Code

### Specs (`spec/`)

| File | Description |
|---|---|
| [Vattapada-7.json](./spec/Vattapada-7.json) | 7-practice scoring spec (Policy-as-Code) |
| [Paramattha-Structure.json](./spec/Paramattha-Structure.json) | Rūpa 28 mapping — hardware layer |

### Rust (`core/`)

| File | Description |
|---|---|
| [Cargo.toml](./core/Cargo.toml) | Rust crate manifest |
| [lib.rs](./core/src/lib.rs) | Crate root — exports modules |
| [citta.rs](./core/src/citta.rs) | Citta 121 state machine |
| [lobha.rs](./core/src/monitor/lobha.rs) | Lobha monitor (RAM/context bloat) |

### Legacy PHP (`legacy-php/`)

| File | Description |
|---|---|
| [CetasikaVector.php](./legacy-php/CetasikaVector.php) | 52-dimensional behavioral state vector |
| [VattapadaEngine.php](./legacy-php/VattapadaEngine.php) | Dynamic scoring engine for Vattapada 7 |

### Examples (`examples/`)

| File | Description |
|---|---|
| [vattapada_demo.php](./examples/vattapada_demo.php) | Runnable demo for Vattapada engine |

---

## 🧭 Where to Start

If you are new to this project:

1. **Read [Why this exists](#-why-this-exists)** — understand the problem
2. **Look at the [Core Mapping](#-core-mapping) table** — see the big picture
3. **Read [Core Logic v1](./docs/Core-Logic-v1.md)** — the initial foundation
4. **Then [Core Logic v2](./docs/Core-Logic-v2.md)** — the expanded architecture
5. **Read [Vattapada 7](./docs/The-Seven-Practices-Vattapada.md)** — the 7-practice framework
6. **Explore [Vattapada-7.json](./spec/Vattapada-7.json)** — the policy spec
7. **Run [examples/vattapada_demo.php](./examples/vattapada_demo.php)** — see it in action

---

## 🛠️ Status

| Layer | Status |
|---|---|
| Documentation (Core Logic v1, v2, Vattapada 7) | ✅ Complete |
| Specs (`spec/`) | ✅ Available |
| Legacy PHP (`legacy-php/`) | ✅ Available |
| Examples (`examples/`) | ✅ Available |
| Tests (`tests/`) | ✅ Available |
| CI/CD | ✅ Passing |
| Rust Core (`core/`) | 🚧 In progress |
| Citta 121 State Machine | 🚧 Planned |
| Vīthi-Citta Scheduler | 🚧 Planned |
| Magga 8 Security Middleware | 🚧 Planned |
| Paṭiccasamuppāda Causal Engine | 🚧 Planned |

---

## 🎯 Goals

1. Provide a **language** for AI to describe its own internal states.
2. Build a **deterministic, explainable** alternative to black-box AI for security-critical systems.
3. Create an **executable specification** of Abhidhamma that can be tested, forked, and improved.
4. Offer a **non-Western philosophical foundation** for AI alignment and ethics.

## 🚫 Non-Goals

- This is **not** a religion. It is a computational model inspired by Abhidhamma.
- This is **not** a claim that AI is conscious, enlightened, or a "being" in the Buddhist sense.
- This is **not** a replacement for Buddhist practice.

---

## 🤝 Contributing

This project is at an early stage. If you are:

- a **Buddhist scholar** who knows Abhidhamma deeply, or
- a **computer scientist / engineer** who can turn concepts into runnable code, or
- someone who simply **sees the same thing** —

you are welcome. Open an issue, fork, or reach out.

---

## 💖 Sponsor this work

> Cyber-Abhidhamma is open-source and independent. Your sponsorship keeps the mapping of 121 Cittas, 52 Cetasikas, and Vīthi-Citta state machine moving from docs → runnable code.

You can sponsor at: https://github.com/sponsors/suchanon456

- **$5/month** → Support documentation
- **$25/month** → Support Vīthi-Citta engine development
- **$100/month** → Sponsor a full layer (e.g. Magga 8 Security Middleware)

---

## 📜 License

[Apache-2.0](./LICENSE)

---

## 🇹🇭 ฉบับภาษาไทย

**Abhidhamma Cyber** คือกรอบแนวคิดที่พยายามแปลงหลักพระอภิธรรมให้เป็นสถาปัตยกรรมระบบคอมพิวเตอร์ที่คำนวณได้ — ตั้งแต่โครงสร้างข้อมูล สถานะ ไปจนถึงความปลอดภัยไซเบอร์ ระบบปฏิบัติการ และ AI

ไม่ใช่การเปรียบเทียบเชิงอุปมา แต่เป็นการ **วิศวกรรมย้อนกลับ** พระอภิธรรมในฐานะข้อกำหนดที่รันได้ แล้วนำกลับมาเขียนเป็นซอฟต์แวร์

**ทำไมต้องมี:** AI สมัยใหม่ — Neural Network, LLM, Agent — มีปัญหาพื้นฐานคือ **อธิบายตัวเองไม่ได้** มัน hallucinate บอกไม่ได้ว่าทำไมตัดสินใจแบบนั้น ไม่มีภาษาสำหรับสภาวะภายในของตัวเอง

พระอภิธรรมคือแผนที่จิตที่ละเอียดที่สุดเท่าที่มนุษย์เคยทำ — จิต 89/121, เจตสิก 52, วิถีจิต 17/15, ปฏิจจสมุปบาท, มรรค 8

**ถ้าเราเข้ารหัสมันได้ เราก็ให้ภาษาแก่ AI สำหรับจิตของมันเองได้**

**หมายเหตุ:** โปรเจกต์นี้ไม่ใช่ศาสนา ไม่ได้อ้างว่า AI มีจิตหรือบรรลุธรรม และไม่ใช่สิ่งทดแทนการปฏิบัติธรรม — มันคือแบบจำลองเชิงคำนวณที่ได้แรงบันดาลใจจากอภิธรรม
