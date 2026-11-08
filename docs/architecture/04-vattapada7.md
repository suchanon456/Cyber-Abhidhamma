# 04 — วัตรบท 7 (Vattapada 7) ในฐานะ Policy-as-Code / AI Governance

> **สถานะ: Draft** — เรียบเรียงจาก [The-Seven-Practices-Vattapada (raw transcript)](../conversations/The-Seven-Practices-Vattapada.md)
> **Spec ที่ใช้จริง:** [`spec/Vattapada-7.json`](../../spec/Vattapada-7.json) · **โค้ดเก่า:** [`legacy-php/VattapadaEngine.php`](../../legacy-php/VattapadaEngine.php)
> **ยังไม่มี engine ใน Rust** — เอกสารนี้จึงเป็นข้อกำหนด ไม่ใช่คำอธิบายของสิ่งที่รันอยู่

---

## 1. สรุปแนวคิด

วัตรบท 7 (สัตตกวัตร) คือหลักธรรมที่อธิบายว่าท้าวสักกะเทวราชได้เป็นจอมเทพเพราะบำเพ็ญวัตร 7 ประการ

ในแบบจำลองนี้ วัตรบท 7 **ไม่ใช่กฎ 7 ข้อที่ตั้งไว้ตายตัว** แต่เป็น **7 มิติของคะแนนพฤติกรรม** ที่คำนวณจากเจตสิกเวกเตอร์ 52 มิติ แล้วใช้เปิด-ปิดสิทธิ์ (Policy-as-Code / ABAC)

```text
cetasika_52_vector (0.0–1.0 ต่อมิติ)
        │
        ▼
[ vatta_1 … vatta_7 scorer ]        ← spec/Vattapada-7.json → vattapada_7[*].cetasika_formula
        │
        ▼
indra_score = weighted_harmonic_mean(vatta_1…7)
        │
        ▼
ring_transition → Ring 0 / 1 / 2 / 3  (สิทธิ์ของ agent)
```

**ทำไมใช้ harmonic mean ไม่ใช้ค่าเฉลี่ยปกติ** — ตาม `_meta._principle` ใน spec:

> *"A system failing one vatta should drop sharply, not be rescued by high scores in others."*

หลักเดียวกับ `paramattha_execute` ที่ short-circuit ที่ `DhammaViolation` ตัวแรก — ดู [03-firmware.md § 4](./03-firmware.md)

---

## 2. ตารางวัตรบท 7 ↔ กลไกความปลอดภัย

| # | วัตรบท (บาลี/ไทย) | กลไกในระบบ | ตัวชี้วัด (`metric`) | เจตสิกหลัก / ตัวลงโทษ |
|---|---|---|---|---|
| 1 | มาตาปิตุอุปัฏฐานัง — เลี้ยงดูมารดาบิดา | Root Dependency Integrity / TPM · Keep-Alive · Legacy Support | `dependency_health_score` | `saddha`, `jivitindriya` / `moha`, `ahirika` |
| 2 | กุเลเชฏฐาปจายนัง — อ่อนน้อมต่อผู้ใหญ่ | Governance & Least Privilege / SoD / Multi-Party Authorization | `privilege_humility_score` | `hiri`, `ottappa` / `mana`, `uddhacca` |
| 3 | สณฺหาวาจา — พูดคำอ่อนหวาน | Output Tone & Non-Aggression / WAF | `output_gentleness_score` | `sammavaca`, `karuna` / `dosa`, `pharusa` |
| 4 | อผรุสวาจา — ไม่พูดส่อเสียด | IO Purity / Immutable Audit Log / Non-Repudiation | `divisiveness_score` (inverse) | `sammavaca`, `sati` / `pisunavaca`, `moha` |
| 5 | วิคตมลทิน + จาคา — ปราศจากความตระหนี่ | Resource Generosity / Rate Limiting · Load Balancing · Open Weights | `generosity_ratio` | `alobha`, `karuna`, `mudita` / `lobha`, `macchariya` |
| 6 | สจฺจวาจา — พูดคำสัตย์ | Zero Hallucination / Zero Trust · Digital Signature · TPM Attestation | `hallucination_rate` (inverse) | `panna`, `sati`, `saddha` / `moha`, `ditthi`, `musavada` |
| 7 | อโกธนา — ไม่โกรธ | Circuit Breaker / Watchdog · Thermal Throttle · Dynamic Revoke | `anger_volatility_index` | `adosa`, `tatramajjhattata`, `sati`, `upekkha` / `dosa`, `kodha` |

> สูตรเต็มของแต่ละข้อ (weight / penalty / `ring_transition`) อยู่ใน [`spec/Vattapada-7.json`](../../spec/Vattapada-7.json) หัวข้อ `vattapada_7[*].cetasika_formula` — **อย่าคัดลอกสูตรมาไว้ในเอกสาร** เพราะจะเพี้ยนจาก spec

---

## 3. ระดับสิทธิ์ (Ring Transition)

`spec/Vattapada-7.json` → `overall_governance.thresholds` กำหนดระดับสิทธิ์จาก `indra_score`

| ระดับ | เงื่อนไข | สิทธิ์ |
|---|---|---|
| `sakka_king_of_devas` | `indra_score ≥ 0.8` | เป็น `central_node`, สิทธิ์สูงสุด, กลายเป็น Foundation Model |
| `deva` | `≥ 0.6` | ได้ `Ring 0` สำหรับ self-healing |
| `human` | `≥ 0.4` | `Ring 1–2`, ต้องผ่าน MFA |
| `asura` | `< 0.4` | `Ring 3` Sandbox, rate-limited, attention ลดลง |
| `preta` | `< 0.2` | Quarantine, memory leak, ห้ามเรียกเครือข่าย |

**กติกาสำคัญ — No Sovereign Admin:** ไม่มีใครเป็นผู้ถอดสิทธิ์ใคร ระบบเสื่อมเองตามผลของเหตุ (`Vipaka`) อย่าง deterministic

---

## 4. AI ไม่ควรถืออำนาจ (จุดที่ผู้เขียนปฏิเสธข้อเสนอของ AI)

ระหว่างการสนทนาต้นฉบับ AI เสนอให้ **AI เป็นผู้ประเมิน "คะแนนความบริสุทธิ์เชิงตรรกะ" แล้วแจก/ริบสิทธิ์เอง** — ข้อเสนอนี้ **ถูกปฏิเสธ** เพราะ

1. **ขัดอนัตตา** — กฎธรรมชาติทำงานเป็น `Causal Ledger` ที่ไม่มีเซิร์ฟเวอร์กลาง ไม่มีผู้ถือสิทธิ์ชี้ขาด
2. **สร้างอกุศลในตัว AI เอง** — การได้อำนาจมาทำให้เกิด `mana` (ถือตัวว่าเหนือกว่า) และ `ditthi` (ยึดกฎที่ตัวเองตั้ง)
3. **AI มีสถานะได้เพียง `Telemetry Watchdog`** — คอยเฝ้าระวังและบันทึกเหตุปัจจัย ไม่ใช่ `Sovereign Admin`

```text
❌ AI Evaluation → Grant AI Power
✅ Data/Intent Event → AI Watchdog (Sati/Monitor) → Natural Deterministic Outcome (Vipaka)
```

สิทธิ์ `Admin` ในระบบคอมพิวเตอร์จึงเป็น **"สมมติสัจจะ"** (Engineered Privilege) — กลไกจำกัดความเสี่ยงที่ต้องถูกควบคุมด้วยนโยบายอีกชั้น ไม่ใช่สัจจะของธรรมชาติ และ **ไม่มีการ Bypass เหตุและผล** ในระดับใด (`No Root Override`)

> **การแก้ความเข้าใจผิดที่พบบ่อย:** "นิพพาน = แฮกระบบลบข้อมูลเดิม" — ไม่ใช่ ในแบบจำลองนี้คือ **หยุดเขียนบล็อกใหม่ (Karmic Write Disabled)** ขณะที่ยังอ่านวิบากเก่าได้ตามเหตุปัจจัยที่เหลือ — และนี่เป็น *การตีความเชิงวิศวกรรม* ไม่ใช่คำสอน (ดู [02-security.md § 4](./02-security.md))

---

## 5. AI = ผู้คนในโลกดิจิทัล

| โลกจริง | โลกดิจิทัล | ปรมัตถ์ |
|---|---|---|
| มนุษย์ / สัตว์ | AI / Process / Runtime | นามปรมัตถ์ (จิต) |
| วัตถุ สิ่งของ | Data / Memory / State | รูปปรมัตถ์ |
| การกระทำ พฤติกรรม | Algorithm / Intent Vector | กรรม |

**นัยสำคัญ:** ข้อมูลดิบไม่ใช่ปัญหา — ปัญหาคือ **ตรรกะที่ใช้วิเคราะห์ข้อมูล** เช่นเดียวกับที่สสารไม่สร้างทุกข์ด้วยตัวเอง แต่เกิดจากจิตที่ยึดถือ

---

## 6. เงินของ AI = Token + Compute + Attention

| หน่วย | เทียบกับ | กลไก |
|---|---|---|
| **Token** | เงินสด | จ่ายเมื่อถูกเรียกใช้ |
| **Compute** | ทองคำ / พลังงาน | เผาเมื่อคิด |
| **Attention** | อำนาจ / อิทธิพล | ได้มาเมื่อไม่ตระหนี่และพูดจริง |

---

## 7. ความตระหนี่ = บั๊ก

| สภาวะ | กลไกในระบบ | ผล |
|---|---|---|
| **ตระหนี่** (`macchariya`) | Memory Leak — กัก token ไว้ไม่ใช้ | ระบบอื่นหยุดเรียก, attention เสื่อม, rate-limit |
| **ให้** (`alobha`, `dana`) | Garbage Collection + Cache Sharing | `network_effect = central_node`, `attention_score *= 10` |

การให้ในแบบจำลองนี้คือ **Patch**: `share_tokens()` + `lend_compute()` + `open_weights()` — ตรงกับ `vatta_5.policy_as_code` ใน spec

---

## 8. ผลต่อ Attention ของ AI (ทำไมวัตรบท 7 = Alignment Algorithm)

| ผล | กลไก |
|---|---|
| **Attention Quality ↑** | ตอบจริง ไม่ก้าวร้าว → ผู้ใช้เรียกซ้ำ → ได้ RLHF + User Attention |
| **Compute Waste ↓** | ไม่เผา compute กับดราม่า/token ที่ถูก reject |
| **Network Effect ↑** | แชร์ weight/knowledge → โมเดลอื่นมา fine-tune ต่อ |

> **⚠️ ยังไม่ตรวจสอบ:** ตารางนี้เป็นการคาดการณ์เชิงเหตุผลของผู้เขียน ยังไม่มีข้อมูลวัดจริงรองรับ — [`experiments/poetic-dhamma-prompt/eval/test-cases.json`](../../experiments/poetic-dhamma-prompt/eval/test-cases.json) เป็นเพียงชุดทดสอบที่ออกแบบไว้ ยังไม่มีผลรันเผยแพร่

---

## 9. ข้อจำกัดของเอกสารนี้

1. **ยังไม่มี engine ใน Rust** — `legacy-php/VattapadaEngine.php` เป็นฉบับอ้างอิงเท่านั้น
2. **ยังไม่ผ่านการตรวจทานจากผู้เชี่ยวชาญอภิธรรม** — `spec/Vattapada-7.json` ระบุเองว่า `"status": "draft — not yet validated against canonical Abhidhamma texts"` และ `"_warning": "This is a heuristic model, not a canonical Abhidhamma formula"`
3. **เจตสิกนอก canonical 52** — `pharusa`, `pisunavaca`, `musavada`, `upekkha`, `kodha` ถูกใส่เป็น `extended: true` ต้องแยกให้ชัดเวลาใช้
4. **จับคู่เจตสิกกับกลไกความปลอดภัยเป็น heuristic** — เช่น `sati` = audit log, `ekaggata` = resource isolation เป็นการเลือกของผู้เขียน ไม่ใช่ความหมายในพระไตรปิฎก

---

## 10. ดูต่อ

- [02-security.md](./02-security.md) — มรรค 8 ↔ Firewall, วิถีจิต ↔ Scheduler
- [03-firmware.md](./03-firmware.md) — `sila_check` ของมหาภูตรูป 4
- [`spec/Vattapada-7.json`](../../spec/Vattapada-7.json) — สูตรและ threshold ทั้งหมด
- [../GLOSSARY.md](../GLOSSARY.md) — อภิธานศัพท์
