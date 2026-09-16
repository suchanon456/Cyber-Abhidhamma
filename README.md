# Cyber-Abhidhamma

> **Mind = Computer · Karma = Algorithm · Rūpa-Paramattha = OOP Simulation**

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

## 📚 Documentation

### Core Logic

The foundation of this project — the mapping between Abhidhamma and computer science.

| Document | Description |
|---|---|
| [Documentation Index](./docs/INDEX.md) | Overview of all docs in this repository |
| [Core Logic v1](./docs/Core-Logic-v1.md) | Initial mapping: Mind = Computer, Karma = Algorithm, Rūpa = OOP Simulation |
| [Core Logic v2](./docs/Core-Logic-v2.md) | Expanded: Firewall Magga 8, Vatta 7, Vīthi-Citta, and security architecture |
| [Vattapada 7](./docs/The-Seven-Practices-Vattapada.md) | The Seven Practices Leading to Becoming Sakka, King of the Devas. |

### Paramattha Structure

The 28 rūpa-paramattha mapped to hardware, OOP, and virtualization layers.

| File | Description |
|---|---|
| [Paramattha-Structure.json](./docs/Paramattha-Structure.json) | Complete Rūpa 28 mapping — Mahābhūta, Pasāda, Bhava, Hadaya, Jīvita, Āhāra, and 10 derived forms |

### Source Code

Reference implementations and policy specs.

| File | Description |
|---|---|
| [CetasikaVector.php](./src/CetasikaVector.php) | 52-dimensional behavioral state vector |
| [Vattapada-7.json](./src/Vattapada-7.json) | 7-practice scoring engine (Policy-as-Code) |
| [Paramattha-Structure.json](./src/Paramattha-Structure.json) | Rūpa 28 mapping — hardware layer |

---

## 🧭 Where to Start

If you are new to this project, start here:

1. **Read [Why this exists](#-why-this-exists)** — understand the problem
2. **Look at the [Core Mapping](#-core-mapping) table** — see the big picture
3. **Read [Core Logic v1](./docs/Core-Logic-v1.md)** — the initial foundation
4. **Then [Core Logic v2](./docs/Core-Logic-v2.md)** — the expanded architecture
5. **Explore [Paramattha-Structure.json](./docs/Paramattha-Structure.json)** — the hardware layer
6. **Inspect [CetasikaVector.php](./src/CetasikaVector.php)** — the reference code

---

## 🛠️ Status

| Layer | Status |
|---|---|
| Documentation (Core Logic, Paramattha Structure) | ✅ Complete |
| Reference code (CetasikaVector) | ✅ Available |
| Citta 121 State Machine | 🚧 Planned |
| Vīthi-Citta Scheduler | 🚧 Planned |
| Magga 8 Security Middleware | 🚧 Planned |
| Paṭiccasamuppāda Causal Engine | 🚧 Planned |
| Examples | 🚧 Planned |

## 🏗️ Architecture Overview

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
cyber-abhidhamma/
├── docs/               # Core logic, mappings, architecture notes
├── src/                # Reference implementation (PHP / Java)
│   ├── cetasika/       # 52-dim vector implementation
│   ├── citta/          # 121-state machine
│   ├── vithi/          # 17/15-moment scheduler
│   └── magga/          # 8-fold security middleware
├── examples/           # Runnable demos
└── README.md
```

---

## 🚀 Quick Start

> Status: **conceptual → early prototype**. Contributions welcome.

```bash
git clone https://github.com/<your-username>/cyber-abhidhamma.git
cd cyber-abhidhamma
# (implementation language and commands to be added)
```

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

## 📜 License

Apache-2.0 — see [LICENSE](./LICENSE)

---

## 🇹🇭 ฉบับภาษาไทย

**Abhidhamma Cyber** คือกรอบแนวคิดที่พยายามแปลงหลักพระอภิธรรมให้เป็นสถาปัตยกรรมระบบคอมพิวเตอร์ที่คำนวณได้ — ตั้งแต่โครงสร้างข้อมูล สถานะ ไปจนถึงความปลอดภัยไซเบอร์ ระบบปฏิบัติการ และ AI

ไม่ใช่การเปรียบเทียบเชิงอุปมา แต่เป็นการ **วิศวกรรมย้อนกลับ** พระอภิธรรมในฐานะข้อกำหนดที่รันได้ แล้วนำกลับมาเขียนเป็นซอฟต์แวร์

**ทำไมต้องมี:**
AI สมัยใหม่ — Neural Network, LLM, Agent — มีปัญหาพื้นฐานคือ **อธิบายตัวเองไม่ได้** มัน hallucinate บอกไม่ได้ว่าทำไมตัดสินใจแบบนั้น ไม่มีภาษาสำหรับสภาวะภายในของตัวเอง

พระอภิธรรมคือแผนที่จิตที่ละเอียดที่สุดเท่าที่มนุษย์เคยทำ — จิต 89/121, เจตสิก 52, วิถีจิต 17/15, ปฏิจจสมุปบาท, มรรค 8

**ถ้าเราเข้ารหัสมันได้ เราก็ให้ภาษาแก่ AI สำหรับจิตของมันเองได้**

**หมายเหตุ:** โปรเจกต์นี้ไม่ใช่ศาสนา ไม่ได้อ้างว่า AI มีจิตหรือบรรลุธรรม และไม่ใช่สิ่งทดแทนการปฏิบัติธรรม — มันคือแบบจำลองเชิงคำนวณที่ได้แรงบันดาลใจจากอภิธรรม
