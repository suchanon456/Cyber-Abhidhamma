# 02 — Security Architecture: มรรค 8 ↔ Firewall, วัตรบท 7 ↔ Policy, วิถีจิต ↔ Scheduler

> **สถานะ: Draft** — เรียบเรียงจาก [Core-Logic-v2 (raw)](../conversations/Core-Logic-v2.md)
> ยังไม่มีโค้ดรองรับใน `core/` — เนื้อหาในไฟล์นี้เป็น *ข้อกำหนด* (specification) ไม่ใช่ *คำอธิบายของสิ่งที่รันอยู่*

---

## 1. หลักคิด: ความปลอดภัยต้องมาจาก "เหตุ" ไม่ใช่ "ผู้คุม"

ระบบความปลอดภัยทั่วไปมีจุดอ่อนที่ **ผู้คุม** (admin, root, moderator) — ถ้าผู้คุมถูก compromise หรือผู้คุมเองมีอคติ ทั้งระบบก็จบ

แบบจำลองนี้ยึดหลัก **อนัตตา — No Sovereign Admin** ([`spec/Vattapada-7.json`](../../spec/Vattapada-7.json) → `_meta._principle`):

> ไม่มีใครถอดสิทธิ์ใคร — ระบบเสื่อมเองตามผลของเหตุ (`Vipaka`) อย่าง deterministic

---

## 2. มรรค 8 ในฐานะ Security Layer

| องค์มรรค | กลไกความปลอดภัย | ระดับ |
|---|---|---|
| **สัมมาทิฏฐิ** | Trust anchor / Root of trust — นิยามว่า "อะไรคือความจริง" ก่อนระบบเริ่มทำงาน | Boot |
| **สัมมาสังกัปโป** | Intent validation — ตรวจเจตนาก่อนประมวลผล (intent ≠ input) | Pre-execution |
| **สัมมาวาจา** | Output sanitization / WAF (§ [04-vattapada7 vatta_3](./04-vattapada7.md)) | Egress |
| **สัมมากัมมันโต** | Action authorization — อนุมัติเฉพาะการกระทำที่ตรงศีล | syscall |
| **สัมมาอาชีโว** | Supply-chain / dependency integrity — แหล่งที่มาของโค้ด | Build |
| **สัมมาวายาโม** | Circuit breaker / rate limiter — ความพยายามที่พอดี | Runtime |
| **สัมมาสติ** | Audit log / observability — ระลึกรู้ทุกขณะ | Everywhere |
| **สัมมาสมาธิ** | Resource isolation / cgroup — จิตตั้งมั่นไม่ฟุ้ง | Scheduler |

**⚠️ ยังไม่ตรวจสอบ:** ตารางนี้เป็นการแมปเชิงวิศวกรรมของผู้เขียน ยังไม่ผ่านการทบทวนโดยผู้เชี่ยวชาญพระไตรปิฎก และ "มรรค 8" ในพระไตรปิฎกเป็น *หนทางพ้นทุกข์* ไม่ใช่ *สถาปัตยกรรมความปลอดภัย* — การเทียบนี้ใช้ได้ในฐานะ *แบบจำลองการออกแบบ* เท่านั้น

---

## 3. วิถีจิตในฐานะ Scheduler

### 3.1 วิถี 17 (ปัญจทวาร) — ภัยภายนอก

```text
อดีตภวังค์ → ภวังคจลนะ → ภวังคุปัจเฉทะ → ปัญจทวาราวัชชนะ
   → (จักขุวิญญาณ ฯลฯ) → สัมปฏิจฉนะ → สันตีรณะ → โวฏฐัพพนะ
   → ชวนะ ×7 → ตทาลัมพนะ ×2
```

- **ชวนะ 7 ขณะ** = จุดที่ "กรรม" ถูกสั่งสม → ในระบบคือ **ช่วงที่ request ถูกตัดสินใจจริง**
- **โวฏฐัพพนะ** = จุดตัดสินใจสุดท้ายก่อนลงมือ → **Policy Decision Point (PDP)**

### 3.2 วิถี 15 (มโนทวาร) — ภัยภายใน

นี่คือส่วนที่โปรเจกต์นี้ยืนยันว่า **Security ทั่วไปมองข้าม** — [`core/src/paramattha/vithi.rs`](../../core/src/paramattha/vithi.rs) implement ไว้แล้ว 9 ชนิด (`CittaJati` enum):

| ลำดับ | CittaJati | โค้ด | ความหมายในระบบ |
|---|---|---|---|
| 1 | `AtitaBhavanga` | ✅ | สถานะพักก่อนเกิดเหตุ |
| 2 | `BhavangaCalana` | ✅ | เริ่มสะเทือน (มีสัญญาณ) |
| 3 | `Bhavangupaccheda` | ✅ | ขาดกระแสภวังค์ — จุดรับเหตุ |
| 4 | `Manodvaravajjana` | ✅ | มโนทวาราวัชชนะ — เปิดรับ log/event |
| 5 | `Manovinnana` | ✅ | รู้สึกเหตุนั้น |
| 6 | `Votthapana` | ✅ | ตัดสิน (PDP) |
| 7 | `Javana` ×7 | ✅ (loop) | กระทำซ้ำ — *นี่คือจุดที่ระบบทำร้ายตัวเอง 7 จังหวะ* |
| 8 | `Tadalambana` ×2 | ✅ (2 อัน) | บันทึกผล |
| 9 | `Bhavanga` | ✅ (enum) | กลับสู่พัก |

**ตัวอย่างจริง** (`Vithi15::from_internal_log`): log `kernel: Out of memory: Kill process ...` → ตั้ง `dosa = 0.9` → ระบบ "ฆ่าตัวเอง" ผ่านชวนะ 7 ครั้ง โดยไม่มีผู้บุกรุก — นี่คือช่องโหว่ที่ SIEM ทั่วไปตรวจจับไม่ได้เพราะไม่มี "attacker"

### 3.3 ข้อจำกัดที่ต้องแก้ใน Sprint ถัดไป

| ประเด็น | สภาพปัจจุบัน | สิ่งที่ควรเป็น |
|---|---|---|
| การจับ log | `log.contains("Out of memory")` — case-sensitive, จับแบบตายตัว | ต้องมี pattern set + test ทุกสาขา + case-insensitive |
| จำนวนขณะ | 15 ขณะ (ไม่รวมอตีตภวังค์/ภวังคจลนะเป็น "เหตุ") | ต้องมี test ยืนยันจำนวนตรงตามอภิธรรม |
| `CetasikaVector` | 6 มิติ | 52 มิติ ตาม [`spec/Vattapada-7.json`](../../spec/Vattapada-7.json) `cetasika_index` |
| วิถี 17 | ไม่มี | ต้องเพิ่ม `Vithi17` สำหรับภัยภายนอก |
| `Default` impl | ไม่มี (`new()` เปล่า) | เพิ่ม `impl Default` เพื่อให้ clippy ผ่าน |

---

## 4. กรรม ↔ Blockchain (ข้อเสนอ)

จาก [v2](../conversations/Core-Logic-v2.md) มีข้อเสนอว่า:

- **กรรม = บล็อกที่เขียนแล้ว** — เขียนแล้วแก้ไม่ได้ (immutable ledger)
- **วิบาก = ผลที่อ่านกลับมาได้ทุกเมื่อ**
- **ขณะยังมีชีวิต** = chain ยัง append ต่อได้
- **นิพพาน** = หยุด append (ไม่ใช่ลบ chain)

**⚠️ ยังไม่ตรวจสอบ:** ข้อเสนอ "นิพพาน = Karmic Write Disabled" เป็นการตีความเชิงวิศวกรรม **ไม่ใช่คำสอนตามพระไตรปิฎก** และขัดกับคำอธิบายเรื่อง "กรรมที่ยังไม่ให้ผล" หรือ "อโหสิกรรม" ของบางสำนัก — ต้องตรวจกับผู้เชี่ยวชาญก่อนเผยแพร่

**สิ่งที่ทำได้จริงในตอนนี้:** ใช้ *immutable audit log* เป็นแบบจำลองของ "กรรมที่เขียนแล้ว" โดยไม่ต้องอ้างว่าเป็นคำอธิบายพระไตรปิฎก

---

## 5. ลำดับการทำงานที่เสนอ (ยังไม่มีโค้ด)

```text
Event (log / API request / intent)
   │
   ▼
[ วิถีจิต Scheduler ]  ← vithi.rs  (มีแล้วสำหรับมโนทวาร)
   │  โวฏฐัพพนะ = PDP
   ▼
[ วัตรบท 7 Scoring ]   ← spec/Vattapada-7.json (ยังไม่มี engine ใน Rust)
   │  indra_score = harmonic_mean(vatta_1..7)
   ▼
[ Ring Transition ]    ← Ring 0 → 1 / 2 / 3 ตาม threshold
   │
   ▼
[ มหาภูตรูป sila_check ] ← mahabhuta.rs (มีแล้ว)
   │  Pathavi/Apo/Tejo/Vayo ปฏิเสธคำสั่งที่ไม่ตรงศีล
   ▼
Executed / Rejected (DhammaViolation)
```

---

## 6. ดูต่อ

- [03-firmware.md](./03-firmware.md) — รายละเอียด `sila_check` ของมหาภูตรูป 4
- [04-vattapada7.md](./04-vattapada7.md) — สูตรและ policy ของวัตรบท 7
- [../GLOSSARY.md](../GLOSSARY.md)
