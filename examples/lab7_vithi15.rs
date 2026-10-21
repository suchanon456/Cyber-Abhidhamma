use cyber_abhidhamma_core::{Vithi15, CittaJati};

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
    ];

    for log in internal_logs {
        let vithi = Vithi15::from_internal_log(log);
        let javana_count = vithi.moments.iter().filter(|m| matches!(m, CittaJati::Javana(_))).count();
        println!("Log: {} -> {} ({}ขณะ ชวนะ {}ครั้ง)", vithi.source, vithi.internal_cause, vithi.moments.len(), javana_count);
    }
}
