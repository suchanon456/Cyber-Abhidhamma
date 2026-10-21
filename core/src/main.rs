mod paramattha;
use paramattha::vithi::{Vithi15, CittaJati};

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

    for log in internal_logs {
        let vithi = Vithi15::from_internal_log(log);
        println!("Log: {}", vithi.source);
        println!("  -> วิเคราะห์: {}", vithi.internal_cause);
        println!("  -> วิถี: {} ขณะ", vithi.moments.len());
        
        let javana_count = vithi.moments.iter().filter(|m| matches!(m, CittaJati::Javana(_))).count();
        println!("  -> ชวนะ {} ครั้ง", javana_count);

        if vithi.internal_cause.contains("โทสะ") {
            println!("  -> [อันตราย] ระบบฆ่าตัวเอง\n");
        } else if vithi.internal_cause.contains("โลภะ") {
            println!("  -> [อันตราย] ระบบอ้วก log จนตาย\n");
        } else if vithi.internal_cause.contains("โมหะ") {
            println!("  -> [อันตราย] Deadlock\n");
        } else {
            println!("  -> [OK] กุศลภายใน\n");
        }
    }
}
