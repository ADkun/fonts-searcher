use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use winapi::um::wingdi::{EnumFontFamiliesExW, LOGFONTW, FONTENUMPROCW};
use winapi::um::winuser::GetDC;
use winapi::shared::windef::HDC;
use winapi::ctypes::c_int;
use std::sync::Mutex;
use std::env;

// 使用 Mutex 来存储匹配的字体名称
lazy_static::lazy_static! {
    static ref MATCHED_FONTS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

fn main() {
    // 从命令行参数获取搜索关键词
    let args: Vec<String> = env::args().collect();
    let search_term = if args.len() > 1 {
        args[1].to_lowercase()
    } else {
        String::new()
    };

    unsafe {
        let hdc = GetDC(std::ptr::null_mut());
        if hdc.is_null() {
            println!("Failed to get device context");
            return;
        }

        let mut log_font: LOGFONTW = std::mem::zeroed();
        log_font.lfCharSet = 1; // DEFAULT_CHARSET

        EnumFontFamiliesExW(
            hdc,
            &mut log_font,
            Some(enum_font_names_proc),
            &search_term as *const String as isize, // 传递搜索关键词
            0,
        );
    }

    // 打印匹配的字体
    let matched_fonts = MATCHED_FONTS.lock().unwrap();
    if matched_fonts.is_empty() {
        println!("No fonts found matching the search term.");
    } else {
        println!("Matching fonts:");
        for font in matched_fonts.iter() {
            println!("{}", font);
        }
    }
}

unsafe extern "system" fn enum_font_names_proc(
    lplf: *const LOGFONTW,
    _: *const winapi::um::wingdi::TEXTMETRICW,
    _: winapi::shared::minwindef::DWORD,
    lparam: winapi::shared::minwindef::LPARAM,
) -> c_int {
    let font_name = OsString::from_wide(&(*lplf).lfFaceName)
        .to_string_lossy()
        .into_owned();
    
    if !font_name.is_empty() {
        // 获取搜索关键词
        let search_term = &*(lparam as *const String);
        
        // 如果字体名称包含搜索关键词（不区分大小写），则添加到匹配列表
        if font_name.to_lowercase().contains(search_term) {
            let mut matched_fonts = MATCHED_FONTS.lock().unwrap();
            matched_fonts.push(font_name);
        }
    }

    1 // continue enumeration
}
