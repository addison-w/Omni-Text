use std::ffi::c_void;

type CFBooleanRef = *const c_void;
type CFStringRef = *const c_void;
type CFDictionaryRef = *const c_void;
type AXUIElementRef = *const c_void;
type AXError = i32;
type CFTypeRef = *const c_void;

const K_AX_ERROR_SUCCESS: AXError = 0;

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXIsProcessTrustedWithOptions(options: CFDictionaryRef) -> bool;
    fn AXUIElementCreateSystemWide() -> AXUIElementRef;
    fn AXUIElementCopyAttributeValue(
        element: AXUIElementRef,
        attribute: CFStringRef,
        value: *mut CFTypeRef,
    ) -> AXError;
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    static kCFBooleanTrue: CFBooleanRef;
    fn CFStringCreateWithCString(
        alloc: *const c_void,
        c_str: *const u8,
        encoding: u32,
    ) -> CFStringRef;
    fn CFDictionaryCreate(
        allocator: *const c_void,
        keys: *const *const c_void,
        values: *const *const c_void,
        num_values: isize,
        key_callbacks: *const c_void,
        value_callbacks: *const c_void,
    ) -> CFDictionaryRef;
    fn CFRelease(cf: *const c_void);
}

const K_CF_STRING_ENCODING_UTF8: u32 = 0x08000100;

fn cf_string(s: &str) -> CFStringRef {
    unsafe {
        let c_str = std::ffi::CString::new(s).unwrap();
        CFStringCreateWithCString(std::ptr::null(), c_str.as_ptr() as *const u8, K_CF_STRING_ENCODING_UTF8)
    }
}

/// Check if the app has accessibility permission
pub fn check_permission() -> bool {
    unsafe {
        let key = cf_string("AXTrustedCheckOptionPrompt");
        // Create dict with prompt = false (just check, don't prompt)
        let keys = [key as *const c_void];
        let false_val = std::ptr::null(); // kCFBooleanFalse equivalent - we pass null to not prompt
        let values = [false_val];
        let dict = CFDictionaryCreate(
            std::ptr::null(),
            keys.as_ptr(),
            values.as_ptr(),
            0, // empty dict = no prompt
            std::ptr::null(),
            std::ptr::null(),
        );
        let result = AXIsProcessTrustedWithOptions(dict);
        CFRelease(dict);
        CFRelease(key);
        result
    }
}

/// Request accessibility permission (shows system prompt dialog)
pub fn request_permission() {
    unsafe {
        let key = cf_string("AXTrustedCheckOptionPrompt");
        let keys = [key as *const c_void];
        let values = [kCFBooleanTrue as *const c_void];
        let dict = CFDictionaryCreate(
            std::ptr::null(),
            keys.as_ptr(),
            values.as_ptr(),
            1,
            std::ptr::null(),
            std::ptr::null(),
        );
        AXIsProcessTrustedWithOptions(dict);
        CFRelease(dict);
        CFRelease(key);
    }
}

/// Get the currently focused UI element's selected text via AX API
pub fn get_selected_text_ax() -> Result<String, String> {
    unsafe {
        let system_wide = AXUIElementCreateSystemWide();
        let focused_attr = cf_string("AXFocusedUIElement");
        let mut focused_element: CFTypeRef = std::ptr::null();

        let err = AXUIElementCopyAttributeValue(
            system_wide,
            focused_attr,
            &mut focused_element,
        );

        if err != K_AX_ERROR_SUCCESS || focused_element.is_null() {
            CFRelease(system_wide);
            CFRelease(focused_attr);
            return Err("Could not get focused element".into());
        }

        let selected_text_attr = cf_string("AXSelectedText");
        let mut selected_text_value: CFTypeRef = std::ptr::null();

        let err = AXUIElementCopyAttributeValue(
            focused_element as AXUIElementRef,
            selected_text_attr,
            &mut selected_text_value,
        );

        let result = if err == K_AX_ERROR_SUCCESS && !selected_text_value.is_null() {
            // Convert CFString to Rust String
            let cf_str = selected_text_value as *const c_void;
            let length = CFStringGetLength(cf_str);
            let max_size = CFStringGetMaximumSizeForEncoding(length, K_CF_STRING_ENCODING_UTF8) + 1;
            let mut buffer = vec![0u8; max_size as usize];
            let success = CFStringGetCString(
                cf_str,
                buffer.as_mut_ptr() as *mut i8,
                max_size,
                K_CF_STRING_ENCODING_UTF8,
            );
            if success {
                let c_str = std::ffi::CStr::from_ptr(buffer.as_ptr() as *const i8);
                Ok(c_str.to_string_lossy().into_owned())
            } else {
                Err("Failed to convert selected text to string".into())
            }
        } else {
            Err("No text selected or accessibility error".into())
        };

        if !selected_text_value.is_null() {
            CFRelease(selected_text_value);
        }
        CFRelease(selected_text_attr);
        CFRelease(focused_element);
        CFRelease(focused_attr);
        CFRelease(system_wide);

        result
    }
}

/// Check if the focused element is a secure text field
pub fn is_secure_field() -> bool {
    unsafe {
        let system_wide = AXUIElementCreateSystemWide();
        let focused_attr = cf_string("AXFocusedUIElement");
        let mut focused_element: CFTypeRef = std::ptr::null();

        let err = AXUIElementCopyAttributeValue(
            system_wide,
            focused_attr,
            &mut focused_element,
        );

        if err != K_AX_ERROR_SUCCESS || focused_element.is_null() {
            CFRelease(system_wide);
            CFRelease(focused_attr);
            return false;
        }

        let role_attr = cf_string("AXRole");
        let mut role_value: CFTypeRef = std::ptr::null();

        let err = AXUIElementCopyAttributeValue(
            focused_element as AXUIElementRef,
            role_attr,
            &mut role_value,
        );

        let is_secure = if err == K_AX_ERROR_SUCCESS && !role_value.is_null() {
            let cf_str = role_value as *const c_void;
            let length = CFStringGetLength(cf_str);
            let max_size = CFStringGetMaximumSizeForEncoding(length, K_CF_STRING_ENCODING_UTF8) + 1;
            let mut buffer = vec![0u8; max_size as usize];
            let success = CFStringGetCString(
                cf_str,
                buffer.as_mut_ptr() as *mut i8,
                max_size,
                K_CF_STRING_ENCODING_UTF8,
            );
            if success {
                let c_str = std::ffi::CStr::from_ptr(buffer.as_ptr() as *const i8);
                c_str.to_string_lossy().contains("SecureTextField")
            } else {
                false
            }
        } else {
            false
        };

        if !role_value.is_null() {
            CFRelease(role_value);
        }
        CFRelease(role_attr);
        CFRelease(focused_element);
        CFRelease(focused_attr);
        CFRelease(system_wide);

        is_secure
    }
}

// Additional CoreFoundation string helpers
extern "C" {
    fn CFStringGetLength(the_string: *const c_void) -> isize;
    fn CFStringGetMaximumSizeForEncoding(length: isize, encoding: u32) -> isize;
    fn CFStringGetCString(
        the_string: *const c_void,
        buffer: *mut i8,
        buffer_size: isize,
        encoding: u32,
    ) -> bool;
}
