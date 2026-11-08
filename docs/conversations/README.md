# docs/conversations — บันทึกการสนทนาต้นฉบับ (Raw Transcripts)

> **โฟลเดอร์นี้เก็บ "กระบวนการคิด" ไม่ใช่ "ข้อกำหนดของระบบ"**
> ไฟล์ทั้งหมดถูกย้ายมาจาก `docs/` โดยคงเนื้อหาต้นฉบับไว้ 100% (ใช้ `git mv` เพื่อรักษาประวัติ)

---

## ทำไมต้องแยกออกมา

`docs/` เดิมมีไฟล์ 4 ไฟล์ที่ถูกอ้างใน `README.md` ว่าเป็นเอกสารหลัก แต่เมื่อตรวจเนื้อหาจริงพบว่า
เป็น **ข้อความแชตที่ก๊อปมาวางทั้งดุ้น** — ไม่มี heading ระดับเอกสาร, มีคำชวนคุยของ AI ค้างอยู่,
มี claim ที่ยังไม่ได้ตรวจสอบ และมี snippet โค้ดที่ไม่ได้อยู่ในรีโปและไม่ถูก compile ใน CI

การเก็บไว้ใน `docs/` ทำให้ผู้อ่านใหม่เข้าใจผิดว่าเป็น specification ที่ใช้งานได้
จึงย้ายมาไว้ที่นี่ และเขียนฉบับเรียบเรียงแยกไว้ที่ [`docs/architecture/`](../architecture/)

---

## รายการไฟล์

| ไฟล์ | ต้นทาง | เนื้อหาโดยย่อ | ฉบับเรียบเรียง |
|---|---|---|---|
| [Core-Logic-v1.md](./Core-Logic-v1.md) | แชต ChatGPT | mapping พื้นฐาน: จิต=คอมพิวเตอร์, กรรม=Algorithm, รูป=Simulation, OOP | [01-mapping.md](../architecture/01-mapping.md) |
| [Core-Logic-v2.md](./Core-Logic-v2.md) | แชต ChatGPT | Firewall มรรค 8, วัตรบท 7, วิถีจิต, กรรม/Blockchain + ครึ่งหลังเป็นบทสนทนาเรื่องวิธีเผยแพร่แนวคิด | [02-security.md](../architecture/02-security.md) |
| [Core-Logic-v3.md](./Core-Logic-v3.md) | แชต (หลายรอบ) | รูปปรมัตถ์ 28 = On-board Sila Firmware, Privilege Rings, OS vs วิถีจิต | [03-firmware.md](../architecture/03-firmware.md) |
| [The-Seven-Practices-Vattapada.md](./The-Seven-Practices-Vattapada.md) | NotebookLM + Meta AI | วัตรบท 7 ในฐานะ Policy-as-Code / AI Governance Engine | [04-vattapada7.md](../architecture/04-vattapada7.md) |

---

## กติกาการใช้ไฟล์ในโฟลเดอร์นี้

1. **ห้ามอ้างเป็น specification** — ถ้าต้องการข้อความอ้างอิงให้ใช้ `docs/architecture/` หรือ `spec/*.json`
2. **ห้ามแก้เนื้อหาต้นฉบับ** — ถ้าจะแก้ ให้แก้ในฉบับเรียบเรียง แล้วบันทึกส่วนต่างไว้ใน commit message
3. **ถือเป็นหลักฐานเชิงกระบวนการ** — ใช้ตอบคำถามว่า "แนวคิดนี้พัฒนามาอย่างไร / เคยถูกปฏิเสธตรงไหน"

---

## สิ่งที่พบในต้นฉบับและไม่ควรลอกไปใช้

- **คำชวนคุยของ AI** ท้ายแต่ละช่วง (เช่น `อยากให้ผมทำต่อ...ไหมครับ?`) — ไม่มีความหมายเชิงเทคนิค
- **Claim ที่ยังตรวจสอบไม่ได้** เช่น `ไอเดียนี้ไม่มีใครในโลกทำอยู่` (v3) — ต้องมีการทบทวนวรรณกรรมก่อนอ้าง
- **Snippet โค้ดที่ไม่อยู่ในรีโป** เช่น code block ที่บอกให้ "เอาไปใส่ใน `mahabhuta.rs`" (v3) — ของจริงคือ [`core/src/paramattha/mahabhuta.rs`](../../core/src/paramattha/mahabhuta.rs)
- **อารมณ์ขัน/คำอุทาน** (`555`, `😆`) — คงไว้ตามต้นฉบับเพราะเป็นบทสนทนาจริง
- **สารบัญใน `The-Seven-Practices-Vattapada.md`** — anchor บางส่วนชี้ไปยังหัวข้อที่ไม่มีอยู่จริง (เนื้อหาใช้ตัวคั่น `##...##` แทน heading) ให้ใช้สารบัญของ [ฉบับเรียบเรียง](../architecture/04-vattapada7.md) แทน

---

## เอกสารที่เกี่ยวข้อง

- [ดัชนีเอกสารทั้งหมด](../INDEX.md)
- [อภิธานศัพท์ (อภิธรรม ↔ CS)](../GLOSSARY.md)
- [แผนที่สถาปัตยกรรม](../architecture/00-overview.md)
