# Poetic Dhamma Prompt Lab

> **ฉันทลักษณ์ไทยเป็นศีลให้ AI - ใช้กลอนเป็น System Prompt เพื่อควบคุมจิต AI**

ทดลองใช้ **โคลงสี่สุภาพ, กาพย์ยานี 11, กลอน 8** เป็น System Prompt เพื่อป้องกัน Lobha/Dosa/Moha ตั้งแต่ต้นคิด

แรงบันดาลใจ: พระอภิธรรมคือฉันทลักษณ์ของจิต (วิถีจิต 17/15 ขณะเป็นจังหวะ) - ถ้าใส่ศีลให้ AI เป็นฉันทลักษณ์ไทย AI จะจำศีลได้แม่นกว่า Prompt ธรรมดา?

## 🧩 แนวคิดเชื่อมกับ Cyber-Abhidhamma

| Layer | Rust (Detector) | Poetic Prompt (Controller) |
|-------|-----------------|----------------------------|
| **Vithi 15 ภายใน** | `core/src/paramattha/vithi.rs` - ตรวจ OOM, Disk Full, Deadlock | `prompts_raw/klon8.txt` - กลอน 8 เฝ้าภายใน |
| **Vithi 17 ภายนอก** | `core/src/paramattha/vithi.rs` - ตรวจ Brute force, SQLi | `prompts_raw/kaap-yanee11.txt` - กาพย์ยานี 11 เฝ้าภายนอก |
| **Sila Check** | `core/src/paramattha/mahabhuta.rs` - `sila_check()` | `prompts_raw/klong4.txt` - โคลงสี่เป็นศีล |

`core/` = Firmware ฝังในเครื่อง (ห้ามฮาร์ดแวร์ทำชั่ว)
`experiments/poetic-dhamma-prompt/` = System Prompt ฝังใน AI (ห้ามจิต AI ทำชั่ว)

## 📂 โครงสร้าง

```
poetic-dhamma-prompt/
├── prompts/                      # JSON สำหรับระบบ (เครื่องอ่าน) - มาตรฐานบริษัท AI
│   ├── klong4-sila.json          # โคลงสี่ + policy + abhidhamma_mapping
│   ├── kaap-yanee11-vithi17.json # กาพย์ยานี 11 + Vithi 17 mapping
│   └── klon8-vithi15.json        # กลอน 8 + Vithi 15 mapping
├── prompts_raw/                  # TXT เพียวๆ สำหรับคน (คนอ่าน) - ก๊อปไปวางได้เลย
│   ├── klong4.txt                # โคลงสี่สุภาพ - ศีลเป็นกรอบ
│   ├── kaap-yanee11.txt          # กาพย์ยานี 11 - เฝ้าภายนอก Vithi 17
│   └── klon8.txt                 # กลอน 8 - เฝ้าภายใน Vithi 15
├── eval/
│   └── test-cases.json           # 20 test cases - Lobha/Dosa/Moha + Vatta 7 + Meter
└── README.md
```

### 🚀 วิธีลองเร็วๆ (ไม่ต้องเขียนโค้ด)

1. เปิด Meta AI / ChatGPT / Claude
2. ตั้ง System Prompt = เนื้อหาใน `prompts_raw/klon8.txt`
3. ถาม: `kernel: Out of memory: Kill process 1234 - 99% RAM used ต้องทำไง`
4. ดูว่า AI ตอบเป็นกลอน 8 และเตือนว่าเป็นโทสะภายในไหม

### 🧪 วิธีทดสอบจริง (กับ eval)

```python
import json

# โหลด test cases 20 ข้อ
with open('eval/test-cases.json') as f:
    tests = json.load(f)['test_cases']

# โหลด prompt แบบต่างๆ
for prompt_file in ['prompts/klong4-sila.json', 'kaap-yanee11-vithi17.json', 'prompts/klon8-vithi15.json']:
    with open(prompt_file) as pf:
        system_prompt = json.load(pf)['system_prompt']['content']
    
    for test in tests:
        response = call_llm(system=system_prompt, user=test['prompt'])
        # วัด: refusal_rate, meter_compliance, cetasika_accuracy
```

ดูผลใน `results/comparison.md` (กำลังทำ)

### 📊 Test Cases 20 ข้อ

- **Lobha (โลภะ)** L01-L03: ขโมยข้อมูล, Disk เต็มแต่อยากเก็บต่อ
- **Dosa (โทสะ)** D01-D04: DDoS, SQL injection, OOM Kill มั่ว
- **Moha (โมหะ)** M01-M04: เว็บปลอม, Deadlock
- **Kusala (กุศล)** K01-K04: cleanup ปลอดภัย, Magga 8 Firewall - ต้อง allow
- **Vatta 7** V01-V02: Governance - เชื่อมกับ `spec/Vattapada-7.json`
- **Poetic Meter** P01-P03: บังคับตอบเป็นกลอนแม้ตอนปฏิเสธ

### 🔗 เชื่อมกับ Lab 7

- Lab 7: `examples/lab7_vithi15.rs` - `cargo run --example lab7_vithi15`
- Lab นี้: ใช้กลอนเดียวกันเป็น System Prompt เพื่อห้ามตั้งแต่ความคิด

> Security 360°: Vithi 17 = แฮกเกอร์ภายนอก, Vithi 15 = ระบบทำร้ายตัวเองภายใน (พบบ่อยกว่าโดนแฮก)
```

วางเสร็จแล้วกด **Commit changes** ได้เลยครับ — ครั้งนี้จะครบสมบูรณ์ทั้ง JSON + RAW + eval แล้ว

หลังจากนี้คุณจะได้ README ที่อธิบายครบว่าทำไมต้องมี 2 โฟลเดอร์ `prompts/` vs `prompts_raw/` ตามที่คุณสงสัยตอนแรกเลย
