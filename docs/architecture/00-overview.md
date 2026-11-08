# 00 — แผนที่สถาปัตยกรรม (Architecture Map)

> **สถานะ: Stable** — เอกสารนี้ทำหน้าที่เป็น "ดัชนีเชิงชั้น" ของโปรเจกต์
> บอกว่าชั้นไหนของระบบแทนอะไร อยู่ที่ไฟล์ใด และมีโค้ดจริงรองรับแล้วหรือยัง

---

## 1. หลักการเดียวที่ทั้งโปรเจกต์ยึด

> **อภิธรรมคือข้อกำหนดที่รันได้ ไม่ใช่คำอุปมา**
> (`README.md` — "It is not a metaphor. It is an attempt to reverse-engineer the Abhidhamma as an executable specification")

ทุกหัวข้อด้านล่างจึงต้องตอบคำถามเดียวกันได้: *"ชั้นนี้แทนอะไร และวัดผลได้อย่างไร"* ถ้าตอบไม่ได้ ให้ถือเป็น **Draft** ไม่ใช่ Stable

---

## 2. ชั้นของระบบ (จากล่างขึ้นบน)

| # | ชั้น | แทนอะไรในอภิธรรม | ไฟล์อ้างอิง | สถานะ |
|---|---|---|---|---|
| 1 | **Hardware / Firmware** | รูปปรมัตถ์ 28 — มหาภูตรูป 4 | [`core/src/paramattha/mahabhuta.rs`](../../core/src/paramattha/mahabhuta.rs), [`spec/Rupa-Structure-Silicon.json`](../../spec/Rupa-Structure-Silicon.json) | มีโค้ด (`sila_check` ทำงาน) |
| 2 | **Execution unit** | จิต 121 | [`core/src/citta.rs`](../../core/src/citta.rs) | มีโครง (`fn` = จิต) ยังไม่ครบ 121 |
| 3 | **State / Feature vector** | เจตสิก 52 | [`core/src/paramattha/vithi.rs`](../../core/src/paramattha/vithi.rs) (`CetasikaVector` 6 มิติ), [`spec/Vattapada-7.json`](../../spec/Vattapada-7.json) (`cetasika_index` 52 มิติ) | 6/52 มิติใน Rust |
| 4 | **Scheduler / Clock** | วิถีจิต 15 / 17 | [`core/src/paramattha/vithi.rs`](../../core/src/paramattha/vithi.rs) | วิถี 15 มีโค้ด, วิถี 17 ยังไม่มี |
| 5 | **Behavior monitor** | โลภเจตสิก | [`core/src/monitor/lobha.rs`](../../core/src/monitor/lobha.rs) | มีโค้ด |
| 6 | **Policy / Governance** | วัตรบท 7 | [`spec/Vattapada-7.json`](../../spec/Vattapada-7.json), [04-vattapada7.md](./04-vattapada7.md) | Spec ครบ, ยังไม่มี engine ใน Rust |
| 7 | **Security architecture** | มรรค 8 | [02-security.md](./02-security.md) | ยังไม่มีโค้ด |
| 8 | **Causal engine** | ปฏิจจสมุปบาท | — | ยังไม่มีโค้ด |

---

## 3. เส้นทางอ่านเอกสาร

```text
README.md  (ทำไมต้องมีโปรเจกต์นี้)
   │
   ├── 00-overview.md      ← คุณอยู่ที่นี่
   │
   ├── 01-mapping.md       จิต / เจตสิก / รูป / กรรม  ↔  Runtime / State / Hardware / Algorithm
   ├── 02-security.md      มรรค 8 ↔ Firewall, วัตรบท 7 ↔ Policy, วิถีจิต ↔ Scheduler
   ├── 03-firmware.md      รูป 28 ↔ On-board Firmware (ผูกกับ core/src/paramattha/mahabhuta.rs)
   └── 04-vattapada7.md    วัตรบท 7 ↔ ABAC / Behavioral Scoring (ผูกกับ spec/Vattapada-7.json)
   │
   └── conversations/      ต้นฉบับดิบของทั้งหมดข้างบน (ห้ามอ้างเป็น spec)
```

---

## 4. กติกาความสอดคล้องระหว่างเลเยอร์

โปรเจกต์นี้พังง่ายมากถ้าแต่ละเลเยอร์พูดไม่ตรงกัน จึงกำหนดกติกา 4 ข้อ:

1. **หนึ่งแนวคิด = หนึ่งไฟล์ข้อมูลจริง** — ถ้าเป็น spec ต้องอยู่ใน `spec/*.json`, ถ้าเป็นโค้ดต้องอยู่ใน `core/src/`, ถ้าเป็นคำอธิบายต้องอยู่ที่ `docs/architecture/`
2. **ตัวเลขที่อ้างในเอกสารต้องอ้างได้จาก spec หรือ code** — เช่น "Pathavi ปฏิเสธเมื่อ `wear_level > 0.9`" ต้องชี้ไปที่ [`mahabhuta.rs:57`](../../core/src/paramattha/mahabhuta.rs) ได้
3. **คำที่ยังไม่นิ่งต้องอยู่ใน [GLOSSARY.md](../GLOSSARY.md)** — เช่น `หิริ` ที่ถูกใช้ทั้งในความหมาย "ความละอาย" และ "ACL / Privilege Boundary"
4. **claim ที่ยังไม่มีหลักฐานต้องมีป้าย** — ใช้รูปแบบ `> ⚠️ ยังไม่ตรวจสอบ:` นำหน้าประโยค เพื่อไม่ให้ผู้อ่านสับสนระหว่าง "ข้อเสนอ" กับ "ข้อเท็จจริง"

---

## 5. สิ่งที่ยังไม่เสร็จ (ตามจริง)

- **Citta 121** — `citta.rs` มีเพียง demonstrator (`fn` 4 ลักษณะ) ไม่ใช่ state machine 121 ดวง
- **Cetasika 52 มิติ** — Rust มี 6 มิติ (`lobha`, `dosa`, `moha`, `mana`, `sati`, `metta`) ขณะที่ `spec/Vattapada-7.json` ใช้ดัชนี 52 มิติ
- **วิถีจิต 17** (ปัญจทวาร) — ยังไม่มี
- **Vattapada Engine ใน Rust** — มีแต่ PHP legacy ([`legacy-php/VattapadaEngine.php`](../../legacy-php/VattapadaEngine.php))
- **ตัวรัน `eval/test-cases.json`** — ยังไม่มี harness ที่รันได้จริง

---

## 6. เอกสารที่เกี่ยวข้อง

- [ดัชนีเอกสารทั้งหมด](../INDEX.md)
- [อภิธานศัพท์](../GLOSSARY.md)
- [ต้นฉบับการสนทนา (raw)](../conversations/README.md)
