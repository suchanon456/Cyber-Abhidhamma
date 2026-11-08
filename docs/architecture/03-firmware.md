# 03 — Rūpa-Paramattha: รูป 28 ในฐานะ On-board Dhamma Firmware

> **สถานะ: Draft (มีโค้ดบางส่วน)** — เรียบเรียงจาก [Core-Logic-v3 (raw)](../conversations/Core-Logic-v3.md)
> โค้ดจริง: [`core/src/paramattha/mahabhuta.rs`](../../core/src/paramattha/mahabhuta.rs) · Spec: [`spec/Rupa-Structure-Silicon.json`](../../spec/Rupa-Structure-Silicon.json)

---

## 1. จาก "OOP Simulation" สู่ "On-board Firmware"

[v1](../conversations/Core-Logic-v1.md) และ [`spec/Paramattha-Structure.json`](../../spec/Paramattha-Structure.json) มองรูปปรมัตถ์เป็น **object ใน OOP** — มีคลาส มี attribute มี method

[v3](../conversations/Core-Logic-v3.md) เปลี่ยนมุมเป็น **firmware ที่มีศีลในตัวเอง**:

| OOP Simulation | On-board Sila Firmware |
|---|---|
| รูปเป็น *ข้อมูล* ที่ถูกประมวลผล | รูปเป็น *ผู้ขับ* (`*Driver`) ที่ **ปฏิเสธคำสั่ง**ได้ |
| ศีลถูกบังคับที่ layer บน | ศีลถูกบังคับที่ชั้นล่างสุด ก่อน I/O ทุกครั้ง |
| ถ้า layer บนถูกเจาะ = จบ | แม้ layer บนถูกเจาะ ฮาร์ดแวร์ยังปฏิเสธ |

---

## 2. Trait กลาง: `MahabhutaRupa`

ทุกธาตุ implement trait เดียวกัน 4 เมธอด:

```rust
pub trait MahabhutaRupa {
    fn what_am_i(&self) -> &'static str;                                         // ลักษณณะ
    fn lakkhana(&self) -> Lakkhana;                                              // เกิด-แก่-ดับ
    fn sila_check(&self, intent: &CetasikaVectorMahabhuta) -> Result<(), DhammaViolation>;  // ศีล
    fn self_heal(&mut self);                                                    // ซ่อมตัวเอง
}
```

`Lakkhana` เก็บ 4 ค่า: `uppada` (เกิด), `santati` (สืบต่อ), `jarata` (แก่/สึก), `aniccata` (ไม่เที่ยง)

`DhammaViolation` มี 3 แบบ: `MicchaKammanta` (ผิดกายกรรม), `MicchaAjiva` (ผิดอาชีพ), `JarataTooHigh` (สึกเกิน)

---

## 3. มหาภูตรูป 4 — พฤติกรรมจริงในโค้ด

| ธาตุ | struct | สภาพที่ถือ | `sila_check` ปฏิเสธเมื่อ | `self_heal` |
|---|---|---|---|---|
| **ปฐวี** (ดิน/แข็ง) | `PathaviDriver` | `base_addr`, `wear_level` | `lobha > 0.85 && dosa > 0.8` → `MicchaAjiva` | `wear_level *= 0.99` |
| **อาโป** (น้ำ/เกาะเกี่ยว) | `ApoDriver` | `cohesion_strength` | `lobha > 0.9` → `MicchaKammanta` | รีเซ็ต `cohesion = 1.0` |
| **เตโช** (ไฟ/ร้อน) | `TejoDriver` | `temp_c` (เริ่ม 30.0) | `dosa > 0.8` → `MicchaKammanta`; `temp_c > 85.0` → `JarataTooHigh` | `temp_c -= 5.0` |
| **วาโย** (ลม/เคลื่อน) | `VayoDriver` | — | `moha > 0.85` → `MicchaAjiva` | ไม่ทำอะไร |

นอกจากนี้ `PathaviDriver::dharana()` ยังปฏิเสธเมื่อ `wear_level > 0.9` ด้วย `JarataTooHigh`

**ตัวรันรวม:** `paramattha_execute(&[driver...], &intent, data)` — วนเรียก `sila_check` ทุก driver และ **short-circuit** ที่ error แรก

---

## 4. ทำไมต้อง short-circuit

เพราะรูปแบบการใช้งานคือ "ถ้าธาตุใดธาตุหนึ่งปฏิเสธ **ทั้งคำสั่ง** ต้องถูกปฏิเสธ" — ไม่ใช่ "โหวตเสียงข้างมาก" หลักนี้ตรงกับ [`spec/Vattapada-7.json`](../../spec/Vattapada-7.json) ที่ใช้ **harmonic mean** แทนค่าเฉลี่ย:

> *"A system failing one vatta should drop sharply, not be rescued by high scores in others."*

---

## 5. สิ่งที่ยังขาด (ต้องทำต่อ)

| # | งาน | เหตุผล |
|---|---|---|
| 1 | **รูป 24 ที่เหลือ** — ปสาทรูป 5, โคจรรูป 4, วิญญัติรูป 2, วิการรูป 3, ลักขณรูป 4, ปริจเฉทรูป 1 | ตอนนี้มีแค่ 4/28 |
| 2 | **Unit test** — ไม่มี `#[cfg(test)]` ใน `mahabhuta.rs` เลย ทั้งที่เป็นตัวบังคับศีล | ต้องมี test ครอบทุก threshold |
| 3 | **`impl Default`** สำหรับ `ApoDriver`, `TejoDriver` | clippy `new_without_default` |
| 4 | **`CetasikaVectorMahabhuta` ซ้ำกับ `CetasikaVector`** (`vithi.rs`) | ควรมี type เดียวใน `paramattha` หรือ share ผ่าน trait |
| 5 | **`paramattha_execute` รับ `_data` แต่ไม่ใช้** | ต้องตัดออกหรือใช้ให้จริง |
| 6 | **ผูกกับ `spec/Rupa-Structure-Silicon.json`** | ตอนนี้ spec กับ code แยกกันโดยไม่มีการตรวจความสอดคล้อง |

---

## 6. ข้อควรระวังทางภาษา

[v3 ต้นฉบับ](../conversations/Core-Logic-v3.md) มีประโยคทำนองว่า *"ไอเดียนี้ไม่มีใครในโลกทำอยู่"* — **ลบออกจากเอกสารหลักแล้ว** เพราะตรวจสอบไม่ได้

ถ้าจะอ้างความใหม่ (novelty) ให้เขียนเป็น:

> ⚠️ **ยังไม่ตรวจสอบ:** ผู้เขียนยังไม่พบงานที่แมปมหาภูตรูป 4 เข้ากับ hardware driver ที่มี `sila_check` ในลักษณะนี้ — ต้องทำ literature review ก่อนอ้างเป็นข้อเท็จจริง

---

## 7. ดูต่อ

- [00-overview.md](./00-overview.md) — ชั้นของระบบ
- [02-security.md](./02-security.md) — ลำดับการทำงานร่วมกับวัตรบท 7
- [`core/src/paramattha/mahabhuta.rs`](../../core/src/paramattha/mahabhuta.rs) — ต้นทางของข้อมูลใน § 3
