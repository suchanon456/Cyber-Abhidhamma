# GLOSSARY — อภิธานศัพท์ อภิธรรม ↔ วิทยาการคอมพิวเตอร์

> **สถานะ: Draft** — ทุกคำนิยามอ้างจาก [`spec/Vattapada-7.json`](../spec/Vattapada-7.json) และโค้ดใน [`core/`](../core/src/) เป็นหลัก
> ถ้าโค้ดกับเอกสารขัดกัน ให้ **โค้ดและ spec** เป็นฝ่ายถูก แล้วแก้เอกสาร
> การแมปทั้งหมดเป็น *แบบจำลอง* ไม่ใช่คำอธิบายตามพระไตรปิฎก — ดู [architecture/01-mapping.md](./architecture/01-mapping.md)

---

## 1. ชั้นของระบบ (Layer)

| บาลี / ไทย | อังกฤษ | ในระบบ | ที่อยู่จริง |
|---|---|---|---|
| **จิต** (Citta 89/121) | Mind / Consciousness | Runtime Engine / CPU | [`core/src/citta.rs`](../core/src/citta.rs) |
| **เจตสิก** (52) | Mental Factor | Behavioral Data Structure / Feature Vector | [`core/src/monitor/lobha.rs`](../core/src/monitor/lobha.rs), `cetasika_index` |
| **รูป** (Rūpa 28) | Matter | On-board Dhamma Firmware / Silicon | [`core/src/paramattha/mahabhuta.rs`](../core/src/paramattha/mahabhuta.rs) |
| **กรรม** | Kamma / Action | Algorithm / Execution Logic | ยังไม่มีโค้ด |
| **วิถีจิต** (17/15 ขณะ) | Cognitive Process | Interrupt Path / Execution Pipeline | [`core/src/paramattha/vithi.rs`](../core/src/paramattha/vithi.rs), [05-vithi-os.md](./architecture/05-vithi-os.md) |
| **ปฏิจจสมุปบาท** | Dependent Origination | Causal Execution Graph | ยังไม่มีโค้ด |
| **นิพพาน** | Nibbāna | Termination of the causal loop | ยังไม่มีโค้ด |
| **มรรค 8** | Noble Eightfold Path | Security Architecture | ยังไม่มีโค้ด — [02-security.md](./architecture/02-security.md) |
| **วัตรบท 7** | Seven Practices (Vatta 7) | Admin Governance / Policy-as-Code | [`spec/Vattapada-7.json`](../spec/Vattapada-7.json) |

---

## 2. ลักษณะของจิต ↔ ฟังก์ชัน (จาก `citta.rs`)

| บาลี | ความหมาย | ลักษณะในโค้ด | ฟังก์ชัน |
|---|---|---|---|
| **วิชานนะ** (vijānana) | รู้แจ้งอารมณ์ | รับ input → คืน output | `citta_knows()` |
| **อุปาทะ-นิโรธ** (uppāda-nirodha) | เกิด-ดับ | เรียกแล้วจบ | `citta_moment()` |
| **สันตติ** (santati) | สืบต่อ | เรียกซ้ำได้ เป็นดวงใหม่ | `citta_stream()` |
| **อนัตตา** (anattā) | ไม่มีตัวตน | ไม่มี state ถาวร | `citta_anatta()` |
| **เจตสิกสัมปยุตต์** | ประกอบด้วยเจตสิกเสมอ | ต้องมี parameter | `citta_with_cetasika()` |

---

## 3. ลักษณะ 4 ของเจตสิก ↔ `struct`

| บาลี | ไทย | องค์ประกอบ Rust | ตัวอย่าง (`LobhaWithFields`) |
|---|---|---|---|
| **ลักษณณะ** (lakkhaṇa) | สภาพของมัน | ชื่อ `struct` | `Lobha` = ความยึดติด |
| **รสะ** (rasa) | กิจ | `field` | `object: String`, `grasped: bool` |
| **ปัจจุปัฏฐาน** (paccupaṭṭhāna) | การปรากฏ | method | `grasp()`, `is_grasping()` |
| **ปทัฏฐาน** (padaṭṭhāna) | เหตุใกล้ | constructor | `LobhaWithFields::new(object)` |

---

## 4. เจตสิกที่ใช้บ่อยในสูตรวัตรบท (จาก `cetasika_index`)

| บาลี | ไทย | ใช้ในสูตร | หมายเหตุ |
|---|---|---|---|
| `saddha` | ศรัทธา | vatta_1, vatta_6 | ในระบบ = trust in root / TPM |
| `jivitindriya` | ชีวิตินทรีย์ | vatta_1 | Keep-Alive Daemon |
| `hiri` | หิริ (ละอายบาป) | vatta_1, 2, 4 | ACL / Privilege Boundary |
| `ottappa` | โอตตัปปะ (กลัวบาป) | vatta_2, 4 | Fail-safe Trigger |
| `sati` | สติ | vatta_1, 2, 4, 6, 7 | Audit log / Monitor |
| `tatramajjhattata` | ตัตรมัชฌัตตตา (เป็นกลาง) | vatta_2, 7 | Load Balancing |
| `sammavaca` | สัมมาวาจา | vatta_3, 4 | Output Sanitization |
| `karuna` / `mudita` | กรุณา / มุทิตา | vatta_3, 5 | Tone filter |
| `alobha` / `adosa` | อโลภะ / อโทสะ | vatta_5, 7 | Buffer deallocation / non-aggression |
| `panna` | ปัญญา | vatta_6 | Core Validation Engine |
| `moha` | โมหะ | penalty (1, 4, 6) | BIOS corrupted / delusion |
| `ahirika` / `anottappa` | อหิริกะ / อโนตตัปปะ | penalty (1) | ไม่มีขอบเขตความละอาย |
| `mana` | มานะ | penalty (2) | Conceit → privilege escalation |
| `uddhacca` | อุทธัจจะ | penalty (2) | Context Jitter |
| `dosa` | โทสะ | penalty (3, 7) | Destroy behavior / thermal spike |
| `lobha` | โลภะ | penalty (5) | Resource hoarding |
| `macchariya` | มัจฉริยะ (ตระหนี่) | penalty (5) | Memory Leak |
| `issā` | อิสสา (ริษยา) | — | ยังไม่ผูกกับวัตรบทใด |
| `ditthi` | ทิฏฐิ | penalty (6) | ยึดกฎที่ตัวเองสร้าง |

### คำที่ `extended: true` — **ไม่อยู่ในเจตสิก 52**

| คำ | ชนิด | หมายเหตุ |
|---|---|---|
| `pharusa` | วจีทุจริต | คำหยาบ — ใช้ลงโทษ vatta_3 |
| `pisunavaca` | วจีทุจริต | คำส่อเสียด — ใช้ลงโทษ vatta_4 |
| `musavada` | วจีทุจริต | คำเท็จ — ใช้ลงโทษ vatta_6 |
| `upekkha` | พรหมวิหาร | อุเบกขา — ใช้แทน `khanti` ใน vatta_7 |
| `kodha` | กิเลสที่แยกจาก `dosa` | ลูกของ `dosa` แยกไว้เน้นน้ำหนัก |

---

## 5. คำในสถาปัตยกรรมความปลอดภัย

| คำ | ความหมายในเอกสารนี้ |
|---|---|
| **Ring 0–3** | ระดับสิทธิ์ของ agent: 0 = kernel (self-healing ได้), 3 = sandbox (แนะนำได้อย่างเดียว) |
| **indra_score** | `weighted_harmonic_mean(vatta_1..7)` — คะแนนรวมที่กำหนด Ring |
| **PDP** | Policy Decision Point — ในแบบจำลองนี้คือ `Votthapana` (โวฏฐัพพนะ) |
| **quantum** | 1 ขณะจิต — หน่วยเวลาที่เกิด-ดับทันที ไม่มี state ค้าง |
| **ISR** | Interrupt Service Routine — ในแบบจำลองนี้คือ `ปัญจวิญญาณ` ของวิถี 17 (วิถี 15 ไม่มี stage นี้) |
| **execution window** | ชวนะ ×7 — ยืนยันผลการตัดสินใจเดิม 7 จังหวะ ไม่ใช่การโหวตซ้ำ |
| **`sila_check`** | เมธอดของมหาภูตรูปที่ **ปฏิเสธคำสั่ง** ที่ไม่ตรงศีล |
| **`DhammaViolation`** | error 3 แบบ: `MicchaKammanta`, `MicchaAjiva`, `JarataTooHigh` |
| **short-circuit** | ปฏิเสธทั้งคำสั่งทันทีที่ driver ตัวแรกค้าน (ไม่โหวตเสียงข้างมาก) |
| **Vipaka** | ผลที่ระบบอ่านกลับมา — ในแบบจำลอง `กรรม = append-only ledger` |
| **No Sovereign Admin** | ไม่มีผู้ถอดสิทธิ์ ระบบเสื่อมเองตามเหตุ — หลักอนัตตา |

---

## 6. กติกาการใช้คำ

1. ทุกครั้งที่ใช้ **"Algorithm" แทน "กรรม"** ให้เติม *"ในแบบจำลองนี้"*
2. ทุกครั้งที่ใช้ **"อนัตตา = statelessness"** ให้ระบุว่าเป็นการเทียบเคียงเชิงโครงสร้าง ไม่ใช่ความหมายเดียวกัน
3. ห้ามอ้าง `indra_score` หรือ threshold ว่าเป็น **คำสอน** — เป็น heuristic ตาม `_meta._warning`
4. คำบาลีให้ทับศัพท์ตามที่ `spec/Vattapada-7.json` เขียนไว้ เพื่อให้ค้นในโค้ดเจอ
5. คำว่า **Scheduler**, **quantum**, **ISR** ใช้ในความหมาย *แบบจำลอง* เท่านั้น — ระบบไม่ได้จัดคิว thread จริงตามวิถีจิต (ดู [05-vithi-os.md](./architecture/05-vithi-os.md) § 7)
