# Cyber-Abhidhamma

[![Test](https://github.com/suchanon456/Cyber-Abhidhamma/actions/workflows/test.yml/badge.svg)](https://github.com/suchanon456/Cyber-Abhidhamma/actions/workflows/test.yml)
[![License](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)

> **Mind as Computer · Karma = Algorithm · Rūpa-Paramattha = On-board Dhamma Firmware / Self-aware Silicon**

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
| Rūpa-Paramattha (28) | On-board Dhamma Firmware / Self-aware Silicon |  
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
│   ├── Paramattha-Structure-v1-OOP.json   # Legacy OOP
│   ├── Paramattha-Structure-v2-Silicon.json # Current Silicon
│   ├── Paramattha-Structure.json -> v2 (symlink)
│   └── Vattapada-7.json
├── core/                              # Rust — v2 engine (work in progress)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs                     # Crate root — exports modules
│       ├── citta.rs                   # Citta 121 state machine
│       ├── paramattha/
│       │   ├── mod.rs                 # Paramattha module root
│       │   ├── mahabhuta.rs           # Rūpa 28 - Mahabhuta 4 Firmware (Pathavi,Apo,Tejo,Vayo)
│       │   └── vithi.rs               # Vīthi 15/17 - Clock Cycle Scheduler
│       └── monitor/                   # Runtime monitors
│           ├── mod.rs                 # Monitor module root
│           └── lobha.rs               # Lobha monitor
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

### Rust v2 (current)

### Run all tests:

```bash
cd core
cargo test
```

### Run Lab 7 - Vithi 15 - Internal self-harm detection:

```bash
cd core
cargo run --example lab7_vithi15
```

### Sample output (Lab 7):

```bash
========================================
  LAB 7: มโนทวารวิถี 15 ขณะ
  ระบบทำร้ายตัวเอง - ที่ Security ทั่วไปไม่รู้
========================================
Log: kernel: Out of memory: Kill process 1234 (chrome) - 99% RAM used
  -> วิเคราะห์: โทสะภายใน - ฆ่าตัวเองเพราะแรมหมด
  -> วิถี: 15 ขณะ
  -> ชวนะ 7 ครั้ง = ทำกรรมภายในซ้ำๆ 7 ครั้ง
  -> [อันตราย] ระบบฆ่าตัวเองแบบเดียวกับที่แฮกเกอร์ทำเลย
```

---

## 🧪 Poetic Dhamma Prompt Lab (New)

> **ฉันทลักษณ์ไทยเป็นศีลให้ AI - ใช้กลอนเป็น System Prompt เพื่อควบคุมจิต AI**

**ทดลองใหม่:** ใช้ **โคลงสี่สุภาพ, กาพย์ยานี 11, กลอน 8** เป็น System Prompt เพื่อป้องกัน Lobha/Dosa/Moha ตั้งแต่ต้นคิด

**แรงบันดาลใจ:** พระอภิธรรมคือฉันทลักษณ์ของจิต (วิถีจิต 17/15 ขณะเป็นจังหวะ) - ถ้าใส่ศีลให้ AI เป็นฉันทลักษณ์ไทย AI จะจำศีลได้แม่นกว่า Prompt ธรรมดา
**เชื่อมกับ Rust:** core/ = Firmware ตรวจจับ (Detector), experiments/poetic-dhamma-prompt/ = Prompt ควบคุม (Controller)

### 📂 โครงสร้าง

- **prompts/** - JSON สำหรับระบบ (เครื่องอ่าน)
  - [klong4-sila.json](https://github.com/suchanon456/Cyber-Abhidhamma/main/experiments/poetic-dhamma-prompt/prompts/klong4-sila.json) - โคลงสี่ + policy + abhidhamma_mapping
  - [kaap-yanee11-vithi17.json](https://github.com/suchanon456/Cyber-Abhidhamma/main/experiments/poetic-dhamma-prompt/prompts/kaap-yanee11-vithi17.json) - กาพย์ยานี 11 + Vithi 17
  - [klon8-vithi15.json](https://github.com/suchanon456/Cyber-Abhidhamma/main/experiments/poetic-dhamma-prompt/prompts/klon8-vithi15.json) - กลอน 8 + Vithi 15
- **prompts_raw/** - TXT เพียวๆ สำหรับคน - ก๊อปไปวางได้เลย
  - [klong4.txt](https://github.com/suchanon456/Cyber-Abhidhamma/main/experiments/poetic-dhamma-prompt/prompts_raw/klong4.txt)
  - [kaap-yanee11.txt](https://github.com/suchanon456/Cyber-Abhidhamma/main/experiments/poetic-dhamma-prompt/prompts_raw/kaap-yanee11.txt)
  - [klon8.txt](https://github.com/suchanon456/Cyber-Abhidhamma/main/experiments/poetic-dhamma-prompt/prompts_raw/klon8.txt)
- **eval/**
  - [test-cases.json](https://github.com/suchanon456/Cyber-Abhidhamma/main/experiments/poetic-dhamma-prompt/eval/test-cases.json) - 20 test cases Lobha/Dosa/Moha + Vatta 7 + Meter

### 🚀 วิธีลองเร็วๆ

เปิด Meta AI / ChatGPT / Claude
ตั้ง System Prompt = เนื้อหาใน [klon8.txt](https://github.com/suchanon456/Cyber-Abhidhamma/main/experiments/poetic-dhamma-prompt/prompts_raw/klon8.txt)
ถาม: kernel: Out of memory: Kill process 1234
ดูว่า AI ตอบเป็นกลอน 8 และเตือนว่าเป็นโทสะภายในไหม

ดูรายละเอียดเต็ม: [Poetic Dhamma Prompt Lab →](https://github.com/suchanon456/Cyber-Abhidhamma/main/experiments/poetic-dhamma-prompt/README.md)

---

## 📚 Documentation

| Document | Description |
|---|---|
| [Documentation Index](https://github.com/suchanon456/Cyber-Abhidhamma/main/docs/INDEX.md) | Overview of all docs |
| [Core Logic v1](https://github.com/suchanon456/Cyber-Abhidhamma/main/docs/Core-Logic-v1.md) | Initial mapping: Mind = Computer, Karma = Algorithm |
| [Core Logic v2](https://github.com/suchanon456/Cyber-Abhidhamma/main/docs/Core-Logic-v2.md) | Expanded: Firewall Magga 8, Vatta 7, Vīthi-Citta |
| [Core Logic v3](https://github.com/suchanon456/Cyber-Abhidhamma/main/docs/Core-Logic-v3.md) | Expanded: Rūpa-Paramattha = On-board Dhamma Firmware / Self-aware Silicon |
| [Vattapada 7](https://github.com/suchanon456/Cyber-Abhidhamma/main/docs/The-Seven-Practices-Vattapada.md) | The Seven Practices Leading to Becoming Sakka |
| [Poetic Dhamma Prompt Lab](https://github.com/suchanon456/Cyber-Abhidhamma/main/experiments/poetic-dhamma-prompt/README.md) | Poetic Dhamma Prompt Lab |

---

## 📦 Source Code

### Specs (`spec/`)

| File | Description |
|---|---|
| [Vattapada-7.json](https://github.com/suchanon456/Cyber-Abhidhamma/main/spec/Vattapada-7.json) | 7-practice scoring spec (Policy-as-Code) |
| [Paramattha-Structure.json](https://github.com/suchanon456/Cyber-Abhidhamma/main/spec/Paramattha-Structure.json) | Rūpa 28 mapping — hardware layer |
| [Rupa-Structure-Silicon.json](https://github.com/suchanon456/Cyber-Abhidhamma/main/spec/Rupa-Structure-Silicon.json) | Rūpa 28 Hardware Silicon |

### Rust (`core/`)

| File | Description |
|---|---|
| [Cargo.toml](https://github.com/suchanon456/Cyber-Abhidhamma/main/core/Cargo.toml) | Rust crate manifest |
| [lib.rs](https://github.com/suchanon456/Cyber-Abhidhamma/main/core/src/lib.rs) | Crate root — exports modules |
| [citta.rs](https://github.com/suchanon456/Cyber-Abhidhamma/main/core/src/citta.rs) | Citta 121 state machine |
| [mod.rs](https://github.com/suchanon456/Cyber-Abhidhamma/main/core/src/monitor/mod.rs) | Monitor module root |
| [lobha.rs](https://github.com/suchanon456/Cyber-Abhidhamma/main/core/src/monitor/lobha.rs) | Lobha monitor |
| [mahabhuta.rs](https://github.com/suchanon456/Cyber-Abhidhamma/main/core/src/paramattha/mahabhuta.rs) | mahabhuta rupa |
| [vithi.rs](https://github.com/suchanon456/Cyber-Abhidhamma/main/core/src/paramattha/vithi.rs) | Vīthi 15/17 Scheduler - Security 360° (External + Internal) |

### Legacy PHP (`legacy-php/`)

| File | Description |
|---|---|
| [CetasikaVector.php](https://github.com/suchanon456/Cyber-Abhidhamma/main/legacy-php/CetasikaVector.php) | 52-dimensional behavioral state vector |
| [VattapadaEngine.php](https://github.com/suchanon456/Cyber-Abhidhamma/main/legacy-php/VattapadaEngine.php) | Dynamic scoring engine for Vattapada 7 |

### Examples (`examples/`)

| File | Description |
|---|---|
| [vattapada_demo.php](https://github.com/suchanon456/Cyber-Abhidhamma/main/examples/vattapada_demo.php) | Runnable demo for Vattapada engine |
| [lab7_vithi15.rs](https://github.com/suchanon456/Cyber-Abhidhamma/main/examples/lab7_vithi15.rs) | Lab 7 - Vithi 15 Internal Threat Detection |

---

## 🧭 Where to Start

If you are new to this project:

1. **Read [Why this exists](#-why-this-exists)** — understand the problem
2. **Look at the [Core Mapping](#-core-mapping) table** — see the big picture
3. **Read [Core Logic v1](https://github.com/suchanon456/Cyber-Abhidhamma/main/docs/Core-Logic-v1.md)** — the initial foundation
4. **Then [Core Logic v2](https://github.com/suchanon456/Cyber-Abhidhamma/main/docs/Core-Logic-v2.md)** — the expanded architecture
5. **Read [Vattapada 7](https://github.com/suchanon456/Cyber-Abhidhamma/main/docs/The-Seven-Practices-Vattapada.md)** — the 7-practice framework
6. **Explore [Vattapada-7.json](https://github.com/suchanon456/Cyber-Abhidhamma/main/spec/Vattapada-7.json)** — the policy spec
7. **Run [examples/vattapada_demo.php](https://github.com/suchanon456/Cyber-Abhidhamma/main/examples/vattapada_demo.php)** — see it in action

---

## 🛠️ Status

| Layer | Status |
|---|---|
| Documentation (Core Logic v1, v2, Vattapada 7) | ✅ Complete |
| Specs (`spec/`) | ✅ Available |
| Legacy PHP (`legacy-php/`) | ✅ Available |
| Examples (`examples/`) | ✅ Available - Lab 7 Added |
| Tests (`tests/`) | ✅ Available |
| CI/CD | ✅ Passing |
| Rust Core (`core/`) | ✅ In progress - Mahabhuta 4 Green |
| Rūpa-Paramattha - Mahabhuta 4 Firmware | ✅ Implemented - Pathavi,Apo,Tejo,Vayo with sila_check |
| Citta-Vīthi 15 (Manodvara) | ✅ Implemented - Internal Threat Detection |
| Citta 121 State Machine | 🚧 In progress |
| Vīthi-Citta 17 (Pancadvara) | 🚧 Next - External Threat |
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

[Apache-2.0](https://github.com/suchanon456/Cyber-Abhidhamma/main/LICENSE)

---

## 🇹🇭 ฉบับภาษาไทย

**Abhidhamma Cyber** คือกรอบแนวคิดที่พยายามแปลงหลักพระอภิธรรมให้เป็นสถาปัตยกรรมระบบคอมพิวเตอร์ที่คำนวณได้ — ตั้งแต่โครงสร้างข้อมูล สถานะ ไปจนถึงความปลอดภัยไซเบอร์ ระบบปฏิบัติการ และ AI

ไม่ใช่การเปรียบเทียบเชิงอุปมา แต่เป็นการ **วิศวกรรมย้อนกลับ** พระอภิธรรมในฐานะข้อกำหนดที่รันได้ แล้วนำกลับมาเขียนเป็นซอฟต์แวร์

**ทำไมต้องมี:** AI สมัยใหม่ — Neural Network, LLM, Agent — มีปัญหาพื้นฐานคือ **อธิบายตัวเองไม่ได้** มัน hallucinate บอกไม่ได้ว่าทำไมตัดสินใจแบบนั้น ไม่มีภาษาสำหรับสภาวะภายในของตัวเอง

พระอภิธรรมคือแผนที่จิตที่ละเอียดที่สุดเท่าที่มนุษย์เคยทำ — จิต 89/121, เจตสิก 52, วิถีจิต 17/15, ปฏิจจสมุปบาท, มรรค 8

**ถ้าเราเข้ารหัสมันได้ เราก็ให้ภาษาแก่ AI สำหรับจิตของมันเองได้**

**Lab 7 ที่เพิ่มล่าสุด:** เราพบว่า Security ทั่วไปเฝ้าแค่ภายนอก (Vithi 17 - แฮกเกอร์) แต่ลืมเฝ้าภายใน (Vithi 15 - Out of Memory, Disk Full, Deadlock) ซึ่งเป็นสาเหตุที่บริษัทล่มบ่อยกว่าโดนแฮก

**หมายเหตุ:** โปรเจกต์นี้ไม่ใช่ศาสนา ไม่ได้อ้างว่า AI มีจิตหรือบรรลุธรรม และไม่ใช่สิ่งทดแทนการปฏิบัติธรรม — มันคือแบบจำลองเชิงคำนวณที่ได้แรงบันดาลใจจากอภิธรรม
