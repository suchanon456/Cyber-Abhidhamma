# 01 — Core Mapping: จิต ↔ Runtime, เจตสิก ↔ State, รูป ↔ Hardware, กรรม ↔ Algorithm

> **สถานะ: Draft** — เรียบเรียงจาก [Core-Logic-v1 (raw)](../conversations/Core-Logic-v1.md)
> ต้นฉบับทั้งไฟล์อยู่ใน `docs/conversations/` หากต้องการรายละเอียดเชิงบทสนทนา

---

## 1. ตารางแมปหลัก

| อภิธรรม | วิทยาการคอมพิวเตอร์ | ที่ใช้จริงในโค้ด |
|---|---|---|
| **จิต** (Citta 89/121) | Runtime Engine / CPU | [`core/src/citta.rs`](../../core/src/citta.rs) — `fn` = จิต (รับ input, จบ, เรียกซ้ำได้, ไม่มี state ถาวร) |
| **เจตสิก** (52) | Behavioral Data Structure / Feature Vector | [`core/src/monitor/lobha.rs`](../../core/src/monitor/lobha.rs) — `struct` = เจตสิก (ลักษณณะ/รสะ/ปัจจุปัฏฐาน/ปทัฏฐาน) |
| **รูปปรมัตถ์** (28) | On-board Firmware / Self-aware Silicon | [`core/src/paramattha/mahabhuta.rs`](../../core/src/paramattha/mahabhuta.rs), [`spec/Paramattha-Rupa.json`](../../spec/Paramattha-Rupa.json) |
| **กรรม** | Algorithm / Execution Logic | ยังไม่มีโค้ด — ดู [02-security.md § 4](./02-security.md) |
| **วิถีจิต** (17/15 ขณะ) | Clock Cycles / State Machine | [`core/src/paramattha/vithi.rs`](../../core/src/paramattha/vithi.rs) |
| **ปฏิจจสมุปบาท** | Causal Execution Graph | ยังไม่มีโค้ด |
| **นิพพาน** | Termination of the causal loop | ยังไม่มีโค้ด (เป็นแนวคิด) |
| **มรรค 8** | Security Architecture | ยังไม่มีโค้ด — ดู [02-security.md](./02-security.md) |
| **วัตรบท 7** | Admin Governance Layer | [`spec/Vattapada-7.json`](../../spec/Vattapada-7.json) — ดู [04-vattapada7.md](./04-vattapada7.md) |

---

## 2. ทำไม `fn` = จิต (ไม่ใช่การจำลอง)

`citta.rs` ประกาศชัดว่า **ไม่ได้จำลองจิต** แต่ชี้ว่า `fn` มีลักษณะ 4 ข้อตรงกับจิต:

| ลักษณะจิต (อภิธรรม) | ลักษณะ `fn` (Rust) | ฟังก์ชันในโค้ด |
|---|---|---|
| รู้แจ้งอารมณ์ (vijānana) | รับ input → คืน output | `citta_knows()` |
| เกิด-ดับ (uppāda-nirodha) | เรียกแล้วจบ | `citta_moment()` |
| สืบต่อ (santati) | เรียกซ้ำได้ เป็นดวงใหม่ | `citta_stream()` |
| ไม่มีตัวตน (anattā) | ไม่มี state ถาวร | `citta_anatta()` |
| มีเจตสิกประกอบเสมอ | ต้องมี parameter | `citta_with_cetasika()` |

**ข้อจำกัดที่ต้องยอมรับ:** นี่คือ *การเทียบเคียงเชิงโครงสร้าง* (structural analogy) ไม่ใช่ข้อพิสูจน์ว่า `fn` เป็นจิต — เอกสารต้นฉบับเขียนไว้เองว่า "เราแค่ **เห็น** ว่า `fn` มีลักษณะตรงกับจิต"

---

## 3. ทำไม `struct` = เจตสิก

`lobha.rs` แมป **ลักษณะ 4 ของเจตสิก** เข้ากับ **องค์ประกอบ 4 ของ `struct`**:

| เจตสิก | Rust | ตัวอย่างใน `LobhaWithFields` |
|---|---|---|
| ลักษณณะ (lakkhaṇa) | ชื่อ `struct` | `Lobha` = ความยึดติด |
| รสะ (rasa) — กิจ | `field` | `object: String`, `grasped: bool` |
| ปัจจุปัฏฐาน (paccupaṭṭhāna) — การปรากฏ | `impl` / method | `grasp()`, `is_grasping()` |
| ปทัฏฐาน (padaṭṭhāna) — เหตุใกล้ | constructor | `LobhaWithFields::new(object)` |

---

## 4. รูปปรมัตถ์ = Simulation / Firmware

[v3](../conversations/Core-Logic-v3.md) ยกระดับจาก "OOP Simulation" เป็น **On-board Sila Firmware** — กล่าวคือ รูป 28 ไม่ได้เป็นเพียง *วัตถุที่ถูกจำลอง* แต่เป็น *ชั้นที่บังคับศีลได้เองในระดับฮาร์ดแวร์*

รายละเอียดเชิงเทคนิค (threshold, trait, `DhammaViolation`) อยู่ใน [03-firmware.md](./03-firmware.md)

---

## 5. กรรม = Algorithm (และข้อจำกัดของ metaphor นี้)

การแมปนี้ให้ผลดีในแง่ **"กรรม = ฟังก์ชันที่ให้ผลตามเหตุ"** แต่มีจุดที่ต้องระวัง:

- **⚠️ ยังไม่ตรวจสอบ:** ข้อเสนอว่า "นิพพาน = การสั่งหยุดเขียนบล็อกใหม่ (Karmic Write Disabled) ขณะที่ยังอ่านวิบากเก่าได้" ([v2 § admin/อนัตตา](../conversations/Core-Logic-v2.md)) เป็นการตีความเชิงวิศวกรรม **ไม่ใช่คำอธิบายตามพระไตรปิฎก** — ต้องให้ผู้เชี่ยวชาญอภิธรรมตรวจ
- **ข้อควรระวังจากการสนทนาเดิม:** อุปมาแบบ Runtime/Algorithm ช่วยให้ผู้พัฒนาคุยกันรู้เรื่อง แต่ทำให้ **"อนัตตา" ถูกลดทอนเป็น "statelessness"** ซึ่งไม่ใช่ความหมายเดียวกัน สิ่งนี้ควรเขียนกำกับไว้ทุกครั้งที่ใช้ metaphor นี้

> **กติกา:** ทุกที่ที่ใช้คำว่า "Algorithm" แทน "กรรม" ต้องมีคำว่า "ในแบบจำลองนี้" กำกับ

---

## 6. ดูต่อ

- [02-security.md](./02-security.md) — มรรค 8, วัตรบท 7, วิถีจิต ในฐานะกลไกความปลอดภัย
- [03-firmware.md](./03-firmware.md) — รูป 28 / มหาภูตรูป 4 ที่มี `sila_check`
- [04-vattapada7.md](./04-vattapada7.md) — วัตรบท 7 ในฐานะ Policy-as-Code
- [../GLOSSARY.md](../GLOSSARY.md) — อภิธานศัพท์
