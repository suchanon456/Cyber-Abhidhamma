mod paramattha;
use paramattha::Vithi15;

fn main() {
    println!("========================================");
    println!("  LAB 7: มโนทวารวิถี 15 ขณะ");
    println!("  ระบบทำร้ายตัวเอง - ที่ Security ทั่วไปไม่รู้");
    println!("========================================\n");

    let internal_logs = vec![
        "kernel: Out of memory: Kill process 1234 (chrome) - 99% RAM used",
        "disk: /var/log 100% full - access.log 50GB",
        "service A waiting for B, B waiting for A - deadlock detected",
        "cron: cleanup /tmp - SUCCESS freed 2GB",
        "GC: freed 2GB heap - INFO",
        "systemd: Starting Session - INFO",
    ];

    println!("เหตุการณ์เหล่านี้ไม่ได้โดนแฮก แต่ทำให้บริษัทล่มจริง:\n");

    for log in internal_logs {
        let vithi = Vithi15::from_internal_log(log);
        
        println!("Log: {}", vithi.source);
        println!("  -> วิเคราะห์: {}", vithi.internal_cause);
        println!("  -> วิถี: {} ขณะ", vithi.moments.len());
        
        // แสดงชวนะ 7 ครั้ง
        let javana_count = vithi.moments.iter().filter(|m| matches!(m, paramattha::CittaJati::Javana(_))).count();
        println!("  -> ชวนะ {} ครั้ง = ทำกรรมภายในซ้ำๆ {} ครั้ง", javana_count, javana_count);
        
        if vithi.internal_cause.contains("โทสะ") {
            println!("  -> [อันตราย] ระบบฆ่าตัวเองแบบเดียวกับที่แฮกเกอร์ทำเลย แต่ไม่มีใครเตือน\n");
        } else if vithi.internal_cause.contains("โลภะ") {
            println!("  -> [อันตราย] ระบบอ้วก log จนตาย - บริษัทใหญ่ล่มเพราะอันนี้บ่อยกว่าโดนแฮก\n");
        } else if vithi.internal_cause.contains("โมหะ") {
            println!("  -> [อันตราย] Deadlock - 2 service ทะเลาะกันเองจนค้าง\n");
        } else {
            println!("  -> [OK] กุศลภายใน / กลางๆ - ไม่เป็นไร\n");
        }
    }

    println!("--- สรุป Lab 7 ---");
    println!("1. ระบบทำร้ายตัวเองมีจริง และบ่อยกว่าโดนแฮก");
    println!("2. สาย Security ทั่วไปไม่รู้ เพราะเขาเฝ้าแต่ภายนอก (Vithi 17)");
    println!("3. คุณต้องเฝ้าทั้ง 2 ทวาร: Vithi17 (นอก) + Vithi15 (ใน)");
    println!("\nLab 7 สำเร็จ: ตอนนี้คุณเห็นภาพ Security 360 องศาแล้ว");
    println!("จากที่เคยเห็นแค่แฮกเกอร์ ตอนนี้เห็นระบบฆ่าตัวเองด้วย");
}
