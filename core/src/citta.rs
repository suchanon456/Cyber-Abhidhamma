//! # Citta — จิตในฐานะ `fn` ของ Rust
//!
//! ## หลักการ
//!
//! ไฟล์นี้ **ไม่ใช่** การจำลองจิต
//! แต่เป็นการ **อธิบาย** ว่า `fn` ใน Rust
//! ทำงาน **เหมือน** จิตอย่างไร
//!
//! เราไม่ได้สร้าง "จิต" ขึ้นมา
//! เราแค่ **เห็น** ว่า `fn` มีลักษณะ **ตรงกับ** จิต
//!
//! ---
//!
//! ## ลักษณะของจิตในอภิธรรม
//!
//! จิตมีลักษณะ 4 ประการ:
//!
//! 1. **รู้แจ้งอารมณ์** (vijānana) — จิตรู้อารมณ์
//! 2. **เกิด-ดับ** (uppāda-nirodha) — จิตเกิดแล้วดับทันที
//! 3. **สืบต่อ** (santati) — จิตดวงหนึ่งดับ จิตดวงถัดไปเกิด
//! 4. **ไม่มีตัวตน** (anattā) — จิตไม่ใช่ "เรา" ไม่ใช่ "ของเรา"
//!
//! ## ลักษณะของ `fn` ใน Rust
//!
//! `fn` มีลักษณะ 4 ประการ:
//!
//! 1. **รับ input → คืน output** — เหมือนจิตรู้อารมณ์
//! 2. **เรียกแล้วจบ** — เหมือนจิตเกิดแล้วดับ
//! 3. **เรียกซ้ำได้** — เหมือนจิตดวงใหม่
//! 4. **ไม่มี state ถาวร** — เหมือนจิตไม่มีตัวตน
//!
//! ## การเห็น
//!
//! เมื่อเห็นว่า `fn` = จิต — คุณจะ **ไม่เขียน "จิต"**
//! แต่จะ **ใช้ `fn`** เพื่อทำงานที่จิตทำ
//!
//! ---

use crate::monitor::lobha::LobhaWithFields;

// =========================================================
// 1. จิต = fn — การรู้แจ้งอารมณ์
// =========================================================

/// **จิต = fn** — การรู้แจ้งอารมณ์
///
/// ในอภิธรรม จิตทำหน้าที่ "รู้แจ้งอารมณ์" (vijānana)
/// ใน Rust `fn` ทำหน้าที่ "รับ input → คืน output"
///
/// ทั้งสอง — **คือการทำงานเดียวกัน** — ต่างแค่ภาษา
///
/// ## ตัวอย่าง
///
/// ```
/// use cyber_abhidhamma::citta_knows;
/// let cognition = citta_knows("gold");
/// assert_eq!(cognition, "gold is known");
/// ```
pub fn citta_knows(object: &str) -> String {
    format!("{} is known", object)
}

// =========================================================
// 2. จิต = fn — การเกิด-ดับ
// =========================================================

/// **จิต = fn** — การเกิด-ดับ
///
/// ในอภิธรรม จิตเกิดแล้วดับทันที (uppāda-nirodha)
/// ใน Rust `fn` เรียกแล้วจบ (return)
///
/// ทั้งสอง — **คือการทำงานเดียวกัน**
/// `fn` ไม่มี "ความจำ" ระหว่างการเรียกแต่ละครั้ง
/// จิตก็ไม่มี "ความจำ" ระหว่างการเกิดแต่ละครั้ง
///
/// ## ตัวอย่าง
///
/// ```
/// use cyber_abhidhamma::citta_moment;
/// let moment1 = citta_moment(1);
/// let moment2 = citta_moment(2);
/// // moment1 และ moment2 ไม่มี "ความจำ" ร่วมกัน
/// ```
pub fn citta_moment(id: u32) -> String {
    format!("citta moment #{}", id)
}

// =========================================================
// 3. จิต = fn — การสืบต่อ
// =========================================================

/// **จิต = fn** — การสืบต่อ
///
/// ในอภิธรรม จิตดวงหนึ่งดับ จิตดวงถัดไปเกิด (santati)
/// ใน Rust `fn` เรียกแล้วจบ เรียกใหม่ได้ (sequential calls)
///
/// ทั้งสอง — **คือการทำงานเดียวกัน**
/// `fn` แต่ละครั้ง — เป็น "ดวงใหม่"
/// แต่ `fn` ทั้งหมด — เป็น "กระแสเดียวกัน"
///
/// ## ตัวอย่าง
///
/// ```
/// use cyber_abhidhamma::citta_stream;
/// let stream: Vec<String> = citta_stream(5);
/// assert_eq!(stream.len(), 5);
/// ```
pub fn citta_stream(count: u32) -> Vec<String> {
    (1..=count).map(citta_moment).collect()
}

// =========================================================
// 4. จิต = fn — การไม่มีตัวตน
// =========================================================

/// **จิต = fn** — การไม่มีตัวตน
///
/// ในอภิธรรม จิตไม่มี "ตัวตน" (anattā)
/// ใน Rust `fn` ไม่มี "state ถาวร"
///
/// ทั้งสอง — **คือการทำงานเดียวกัน**
/// `fn` ไม่มี "เจ้าของ" — มันเป็นแค่ "การทำงาน"
/// จิตก็ไม่มี "เจ้าของ" — มันเป็นแค่ "การรู้"
///
/// ## ตัวอย่าง
///
/// ```
/// use cyber_abhidhamma::citta_knows;
/// let a = citta_knows("red");
/// let b = citta_knows("blue");
/// ```
pub fn citta_anatta(object: &str) -> String {
    citta_knows(object)
}

// =========================================================
// 5. จิต = fn — การรู้แจ้งที่มี "เจตสิก" ประกอบ
// =========================================================

/// **จิต = fn** — การรู้แจ้งที่มี "เจตสิก" ประกอบ
///
/// ในอภิธรรม จิตไม่เกิดลำพัง — ต้องมีเจตสิกประกอบเสมอ
/// ใน Rust `fn` ไม่ทำงานลำพัง — ต้องมี parameter
///
/// ทั้งสอง — **คือการทำงานเดียวกัน**
/// parameter ของ `fn` = เจตสิกของจิต
///
/// ## ตัวอย่าง
///
/// ```
/// use cyber_abhidhamma::{citta_with_cetasika, LobhaWithFields};
/// let lobha = LobhaWithFields::new("gold");
/// let cognition = citta_with_cetasika("gold", lobha);
/// ```
pub fn citta_with_cetasika(object: &str, lobha: LobhaWithFields) -> Cognition {
    Cognition {
        object: object.to_string(),
        lobha_active: lobha.is_grasping(),
        lobha_intensity: if lobha.is_grasping() { 1.0 } else { 0.0 },
    }
}

// =========================================================
// 6. ผลรวม — Cognition
// =========================================================

/// **ผลรวม = struct** — สิ่งที่จิตทิ้งไว้
///
/// ในอภิธรรม จิตเกิด-ดับแล้วทิ้ง "ร่องรอย" (ผล)
/// ใน Rust `fn` ทำงานแล้วทิ้ง "return value"
///
/// ทั้งสอง — **คือการทำงานเดียวกัน**
/// `Cognition` = "ผลรวม" ของการทำงาน
#[derive(Debug, Clone)]
pub struct Cognition {
    /// อารมณ์ที่จิตรู้
    pub object: String,
    /// เจตสิกที่ประกอบ — lobha
    pub lobha_active: bool,
    /// ความเข้มข้น
    pub lobha_intensity: f32,
}

// =========================================================
// 7. TESTS — พิสูจน์ว่า "fn = จิต"
// =========================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fn_knows_like_citta() {
        // fn รับ input → คืน output
        // เหมือนจิต รู้แจ้งอารมณ์
        let result = citta_knows("gold");
        assert_eq!(result, "gold is known");
    }

    #[test]
    fn fn_arises_and_ceases_like_citta() {
        // fn เรียกแล้วจบ → เกิด-ดับ
        let m1 = citta_moment(1);
        let m2 = citta_moment(2);
        assert_ne!(m1, m2);
    }

    #[test]
    fn fn_continues_like_citta() {
        // fn เรียกซ้ำ → สืบต่อ
        let stream = citta_stream(5);
        assert_eq!(stream.len(), 5);
        assert_eq!(stream[0], "citta moment #1");
        assert_eq!(stream[4], "citta moment #5");
    }

    #[test]
    fn fn_is_anatta_like_citta() {
        // fn ไม่มี state ถาวร → อนัตตา
        let a = citta_anatta("red");
        let b = citta_anatta("blue");
        assert_ne!(a, b);
        // ไม่มี "state" ของ citta ที่ค้างอยู่
    }

    #[test]
    fn fn_with_cetasika_like_citta() {
        let mut lobha = LobhaWithFields::new("gold");
        lobha.grasp();
    
        let cog = citta_with_cetasika("gold", lobha);
        assert_eq!(cog.object, "gold");
        assert!(cog.lobha_active);
        assert_eq!(cog.lobha_intensity, 1.0);
    }
    
    #[test]
    fn fn_without_cetasika_like_citta() {
        let lobha = LobhaWithFields::new("water");
    
        let cog = citta_with_cetasika("water", lobha);
        assert_eq!(cog.object, "water");
        assert!(!cog.lobha_active);
        assert_eq!(cog.lobha_intensity, 0.0);
    }
}
