// core/examples/simulation.rs
// Simulation Engine — รันท่อทั้งเส้น:
//   Request → Firewall → Vithi17 → VithiMapper → KammaImpact → RupaCluster
//
// ⚠️ ทั้งหมดเป็นการจำลองในหน่วยความจำ (analogy ตาม docs/GLOSSARY.md)
// ไม่มีการสั่งการฮาร์ดแวร์จริงใด ๆ
//
// รันด้วย: cargo run --example simulation

use cyber_abhidhamma::firewall::stages::{AllowedRoutes, PayloadPatterns, RateLimit};
use cyber_abhidhamma::firewall::vithi_gate::inspect_through_vithi;
use cyber_abhidhamma::firewall::{Decision, Firewall, Request};
use cyber_abhidhamma::paramattha::cetasika::{ADOSA, DOSA, LOBHA, MOHA, SATI};
use cyber_abhidhamma::paramattha::kamma_impact::RupaImpact;
use cyber_abhidhamma::paramattha::vipaka_loop::VipakaLoop;
use cyber_abhidhamma::paramattha::vithi::CetasikaVector;
use cyber_abhidhamma::paramattha::vithi17::Dvara;
use cyber_abhidhamma::paramattha::vithi_mapper::VithiMapper;

/// หนึ่งเหตุการณ์ที่ป้อนเข้าสู่การจำลอง
struct Scenario {
    label: &'static str,
    path: &'static str,
    body: &'static str,
    now_ms: u64,
    dvara: Dvara,
    cetasika: CetasikaVector,
}

fn scenarios() -> Vec<Scenario> {
    vec![
        Scenario {
            label: "คำขอปกติ (จิตเป็นกลาง)",
            path: "/api/items",
            body: "q=shoes",
            now_ms: 0,
            dvara: Dvara::Cakkhu,
            cetasika: CetasikaVector::new(),
        },
        Scenario {
            label: "payload อันตราย (โทสะ)",
            path: "/api/items",
            body: "q=' OR 1=1 --",
            now_ms: 1,
            dvara: Dvara::Sota,
            cetasika: CetasikaVector::new().with(DOSA, 0.9),
        },
        Scenario {
            label: "เส้นทางต้องห้าม (โลภะ)",
            path: "/admin",
            body: "",
            now_ms: 2,
            dvara: Dvara::Kaya,
            cetasika: CetasikaVector::new().with(LOBHA, 0.9),
        },
        Scenario {
            label: "คำขอถี่เกิน (โมหะ)",
            path: "/api/x",
            body: "",
            now_ms: 3,
            dvara: Dvara::Jivha,
            cetasika: CetasikaVector::new().with(MOHA, 0.9),
        },
        Scenario {
            label: "คำขอถี่เกิน อีกครั้ง (โมหะ)",
            path: "/api/x",
            body: "",
            now_ms: 4,
            dvara: Dvara::Jivha,
            cetasika: CetasikaVector::new().with(MOHA, 0.8),
        },
        Scenario {
            label: "คำขอถี่เกิน ครั้งที่สาม (ถูก throttle)",
            path: "/api/x",
            body: "",
            now_ms: 5,
            dvara: Dvara::Ghana,
            cetasika: CetasikaVector::new().with(MOHA, 0.7),
        },
        Scenario {
            label: "คำขอที่ประกอบด้วยสติ/เมตตา",
            path: "/api/items",
            body: "q=books",
            now_ms: 6,
            dvara: Dvara::Cakkhu,
            cetasika: CetasikaVector::new().with(SATI, 0.9).with(ADOSA, 0.9),
        },
        Scenario {
            label: "คำขอปกติอีกครั้ง (ฟื้นตัว)",
            path: "/api/items",
            body: "q=lamps",
            now_ms: 7,
            dvara: Dvara::Cakkhu,
            cetasika: CetasikaVector::new().with(SATI, 0.6),
        },
    ]
}

fn pipeline() -> Firewall {
    Firewall::new(vec![
        Box::new(PayloadPatterns::default()),
        Box::new(AllowedRoutes::new(vec!["/api/".into()])),
        Box::new(RateLimit::new(2, 1000)),
    ])
}

fn decision_label(d: &Decision) -> String {
    match d {
        Decision::Allow => "ALLOW".to_string(),
        Decision::Throttle { reason } => format!("THROTTLE ({reason})"),
        Decision::Block { stage, reason } => format!("BLOCK@{stage} ({reason})"),
    }
}

fn main() {
    let mapper = VithiMapper::canonical().expect("spec/Paramattha-Citta.json ต้องโหลดได้");
    let missing = mapper.verify();
    assert!(missing.is_empty(), "mapper อ้าง id ที่ไม่มีใน spec: {missing:?}");

    let mut firewall = pipeline();
    let mut loop_ = VipakaLoop::new();

    println!("=== Cyber-Abhidhamma — Simulation Engine (analogy) ===\n");
    println!(
        "ทะเบียนจิต {} ดวง | วิถี 17 ขณะ | ธาตุ 4 (ปฐวี/อาโป/เตโช/วาโย) | Vipaka feedback เปิด\n",
        mapper.count()
    );
    println!(
        "{:<34} {:>6} {:>4} {:>7} {:<26} {:>7} {:>7} {:<22}",
        "เหตุการณ์", "ขณะ", "ชวนะ", "ชวนจิต", "ผลต่อรูป", "temp", "เสื่อม", "แรงหนุนรอบถัดไป"
    );
    println!("{}", "-".repeat(128));

    let mut allowed = 0usize;
    let mut throttled = 0usize;
    let mut blocked = 0usize;

    for s in scenarios() {
        let request = Request {
            actor: "alice".into(),
            path: s.path.into(),
            body: s.body.into(),
            now_ms: s.now_ms,
        };

        // 0) รูปที่เสื่อมอยู่แล้วย้อนกลับมาเป็นปัจจัยหนุนจิตของวิถีนี้
        let cetasika = loop_.condition(s.cetasika);
        let feedback = loop_.pressure();

        // 1) firewall ตัดสินที่โวฏฐัพพนะของวิถี 17
        let verdict = inspect_through_vithi(&mut firewall, s.dvara, &request, cetasika);
        let vithi = &verdict.vithi;

        match &verdict.decision {
            Decision::Allow => allowed += 1,
            Decision::Throttle { .. } => throttled += 1,
            Decision::Block { .. } => blocked += 1,
        }

        // 2) แมปขั้นวิถี → จิต 121 ดวง
        let mapped = mapper
            .map_vithi17(vithi)
            .expect("วิถีที่ได้จาก firewall ต้องแมปได้");
        let javana_id = mapped
            .javana_citta()
            .map(|m| m.citta_id.as_str())
            .unwrap_or("—");

        // 3) ชวนะ → ผลต่อรูป 4 ธาตุ (และป้อนกลับเข้าวงจร)
        let impact: RupaImpact = loop_.absorb_vithi17(vithi);
        let effect = if impact.is_empty() {
            "ไม่มีชวนะ → ไม่มีผล".to_string()
        } else if impact.is_harmful() {
            format!(
                "wear+{:.3} heat+{:.1} turb+{:.2}",
                impact.pathavi_wear, impact.tejo_heat, impact.vayo_turbulence
            )
        } else {
            format!(
                "cool-{:.1} calm-{:.2} relief-{:.3}",
                impact.tejo_cooling, impact.vayo_calm, impact.pathavi_relief
            )
        };

        println!(
            "{:<34} {:>6} {:>4} {:>7} {:<26} {:>7.1} {:>7.3} {:<22}",
            truncate(s.label, 34),
            vithi.moment_count(),
            vithi.javana_count(),
            truncate(javana_id, 26),
            truncate(&effect, 26),
            loop_.cluster().tejo.temp_c,
            loop_.degradation(),
            truncate(&pressure_label(&feedback), 22),
        );

        println!("{:<34}   → {}\n", "", decision_label(&verdict.decision));
    }

    println!(
        "สรุป: ALLOW {allowed} | THROTTLE {throttled} | BLOCK {blocked} | วิถีที่ผ่านวงจร {} | audit log {} รายการ",
        loop_.vithi_seen(),
        firewall.audit_log().len()
    );
    let cluster = loop_.cluster();
    println!(
        "รูป akhir: ปฐวีเสื่อม {:.3} | อาโป {:.3} | เตโช {:.1}°C | วาโยฟุ้ง {:.2} | degradation {:.3}",
        cluster.pathavi.wear_level,
        cluster.apo.cohesion_strength,
        cluster.tejo.temp_c,
        cluster.vayo_turbulence,
        cluster.degradation(),
    );
    println!("แรงหนุนสุดท้าย: {}", pressure_label(&loop_.pressure()));

    let intent = CetasikaVector::new().with(DOSA, 0.9);
    match cluster.sila_check(&intent) {
        Ok(()) => println!("สีลาจากธาตุ: ผ่าน"),
        Err(v) => println!("สีลาจากธาตุ: ปฏิเสธ ({v})"),
    }
}

/// สรุปแรงหนุนของรูปที่เสื่อมต่อวิถีถัดไป
fn pressure_label(p: &CetasikaVector) -> String {
    if p.get(LOBHA) == 0.0 && p.get(DOSA) == 0.0 && p.get(MOHA) == 0.0 {
        return "ไม่มี (รูปสมบูรณ์)".to_string();
    }
    format!(
        "โลภะ+{:.2} โทสะ+{:.2} โมหะ+{:.2}",
        p.get(LOBHA),
        p.get(DOSA),
        p.get(MOHA)
    )
}

/// ตัดข้อความให้ยาวไม่เกิน `width` ไบต์ (นับตาม char)
fn truncate(text: &str, width: usize) -> String {
    if text.chars().count() <= width {
        return text.to_string();
    }
    let mut out: String = text.chars().take(width.saturating_sub(1)).collect();
    out.push('…');
    out
}
