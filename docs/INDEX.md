# Documentation Index / สารบัญเอกสาร

> ทุกไฟล์ใน `docs/` ใช้ **relative link** — เปิดได้ทั้งบน GitHub, local preview และ editor

**สถานะเอกสาร:** `Stable` = ใช้ได้, `Draft` = กำลังเรียบเรียง, `Transcript` = บทสนทนาต้นฉบับ (ห้ามอ้างเป็น spec), `Machine` = ไฟล์ข้อมูลที่เครื่องอ่าน

---

## 1. เริ่มต้นที่นี่

| เอกสาร | สถานะ | เนื้อหา |
|---|---|---|
| [README.md](../README.md) | Stable | ภาพรวมโปรเจกต์, วิธีรัน, โครงสร้างรีโป |
| [architecture/00-overview.md](./architecture/00-overview.md) | Stable | แผนที่สถาปัตยกรรม: ชั้นใดของระบบแทนอะไร และอยู่ไฟล์ไหน |
| [GLOSSARY.md](./GLOSSARY.md) | Draft | อภิธานศัพท์ อภิธรรม ↔ วิทยาการคอมพิวเตอร์ |

## 2. Architecture (ฉบับเรียบเรียง)

| เอกสาร | สถานะ | เนื้อหา |
|---|---|---|
| [01-mapping.md](./architecture/01-mapping.md) | Draft | จิต=Runtime, เจตสิก=State, รูป=Hardware, กรรม=Algorithm |
| [02-security.md](./architecture/02-security.md) | Draft | Firewall มรรค 8, วัตรบท 7, วิถีจิต 15/17, กรรมในฐานะ Ledger |
| [03-firmware.md](./architecture/03-firmware.md) | Draft | รูปปรมัตถ์ 28 = On-board Sila Firmware, Privilege Rings |
| [04-vattapada7.md](./architecture/04-vattapada7.md) | Draft | วัตรบท 7 ในฐานะ Policy-as-Code / AI Governance |
| [threat-model.md](./threat-model.md) | Draft | Threat model: STRIDE + mapping กลับไปยังชั้นอภิธรรม |

## 3. Specification (machine-readable)

| ไฟล์ | สถานะ | เนื้อหา |
|---|---|---|
| [spec/Paramattha-Index.json](../spec/Paramattha-Index.json) | Machine | ดัชนีไฟล์ Paramattha (แยกจาก Paramattha-Structure เดิม) |
| [spec/Paramattha-Rupa.json](../spec/Paramattha-Rupa.json) | Machine | รูป 28 mapped กับ OS/Hardware (V1-OOP) |
| [spec/Paramattha-Cetasika.json](../spec/Paramattha-Cetasika.json) | Machine | เจตสิก 52 |
| [spec/Paramattha-Citta.json](../spec/Paramattha-Citta.json) | Machine | จิต 121 |
| [spec/Paramattha-Nibbana.json](../spec/Paramattha-Nibbana.json) | Machine | นิพพาน (ส่วนขยายใหม่ ไม่มีใน Paramattha-Structure v3.0.1) |
| [spec/Rupa-Structure-Silicon.json](../spec/Rupa-Structure-Silicon.json) | Machine | รูป 28 ในระดับ Silicon (V2) |
| [spec/Vattapada-7.json](../spec/Vattapada-7.json) | Machine | วัตรบท 7 + policy hooks |

## 4. Code

| ไฟล์ | เนื้อหา |
|---|---|
| [core/src/lib.rs](../core/src/lib.rs) | entry point ของ crate `cyber_abhidhamma` |
| [core/src/citta.rs](../core/src/citta.rs) | Citta 121 |
| [core/src/vattapada.rs](../core/src/vattapada.rs) | Vattapada-7 scoring engine (อ่าน `spec/Vattapada-7.json`) |
| [core/src/attention.rs](../core/src/attention.rs) | Attention score loop (โทสะลด / ทานเพิ่ม) |
| [core/src/authority.rs](../core/src/authority.rs) | แปลง Report → Ring + capabilities |
| [core/src/interceptor.rs](../core/src/interceptor.rs) | Network gate: Authority → Decision |
| [core/src/firewall/mod.rs](../core/src/firewall/mod.rs) | Request gateway pipeline + audit log |
| [core/src/firewall/stages.rs](../core/src/firewall/stages.rs) | Stages: normalize, payload, route, rate limit |
| [core/src/paramattha/vithi.rs](../core/src/paramattha/vithi.rs) | วิถีจิต 15 (ภายใน) |
| [core/src/paramattha/mahabhuta.rs](../core/src/paramattha/mahabhuta.rs) | มหาภูตรูป 4 + `sila_check` |
| [core/src/monitor/lobha.rs](../core/src/monitor/lobha.rs) | Runtime monitor: โลภะ |
| [core/examples/lab7_vithi15.rs](../core/examples/lab7_vithi15.rs) | Lab 7 — วิถีจิต 15 (example ของ crate) |
| [core/tests/spec_integrity.rs](../core/tests/spec_integrity.rs) | เทสต์ความสอดคล้องข้ามไฟล์ spec |
| [tests/VattapadaEngineTest.php](../tests/VattapadaEngineTest.php) | legacy PHP test |

## 5. Experiments

| เอกสาร | สถานะ | เนื้อหา |
|---|---|---|
| [experiments/poetic-dhamma-prompt/README.md](../experiments/poetic-dhamma-prompt/README.md) | Draft | ใช้ฉันทลักษณ์ไทยเป็น System Prompt |
| [FOR-WORKFLOW-BUILDERS.md](../FOR-WORKFLOW-BUILDERS.md) | Draft | คู่มือสำหรับผู้สร้าง workflow |

## 6. Raw Transcripts (ห้ามอ้างเป็น spec)

ย้ายมาจาก `docs/` เดิมทั้ง 4 ไฟล์ — ดูรายละเอียดและกติกาที่ [conversations/README.md](./conversations/README.md)

- [conversations/Core-Logic-v1.md](./conversations/Core-Logic-v1.md) — `Transcript`
- [conversations/Core-Logic-v2.md](./conversations/Core-Logic-v2.md) — `Transcript`
- [conversations/Core-Logic-v3.md](./conversations/Core-Logic-v3.md) — `Transcript`
- [conversations/The-Seven-Practices-Vattapada.md](./conversations/The-Seven-Practices-Vattapada.md) — `Transcript`
