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
    static kCFTypeDictionaryKeyCallBacks: c_void;
    static kCFTypeDictionaryValueCallBacks: c_void;
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
        // Pass NULL options to just check without prompting
        AXIsProcessTrustedWithOptions(std::ptr::null())
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
            &kCFTypeDictionaryKeyCallBacks as *const c_void,
            &kCFTypeDictionaryValueCallBacks as *const c_void,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_permission_does_not_crash() {
        // This was the root cause of the onboarding crash:
        // check_permission() previously created a malformed CFDictionary
        // that caused a SIGSEGV in AXIsProcessTrustedWithOptions.
        // Now it passes NULL which safely checks without prompting.
        let result = check_permission();
        // Result depends on system state, just verify no crash
        assert!(result == true || result == false);
    }

    #[test]
    fn request_permission_does_not_crash() {
        // Verify CFDictionaryCreate with proper callbacks doesn't crash.
        // Note: This will show the system accessibility prompt dialog
        // in CI/interactive environments, but won't crash.
        request_permission();
    }

    #[test]
    fn cf_string_creates_valid_ref() {
        unsafe {
            let s = cf_string("AXFocusedUIElement");
            assert!(!s.is_null());
            CFRelease(s);
        }
    }

    #[test]
    fn cf_string_roundtrip() {
        unsafe {
            let original = "TestString123";
            let cf = cf_string(original);
            assert!(!cf.is_null());

            let length = CFStringGetLength(cf);
            assert!(length > 0);

            let max_size = CFStringGetMaximumSizeForEncoding(length, K_CF_STRING_ENCODING_UTF8) + 1;
            let mut buffer = vec![0u8; max_size as usize];
            let success = CFStringGetCString(
                cf,
                buffer.as_mut_ptr() as *mut i8,
                max_size,
                K_CF_STRING_ENCODING_UTF8,
            );
            assert!(success);

            let c_str = std::ffi::CStr::from_ptr(buffer.as_ptr() as *const i8);
            assert_eq!(c_str.to_string_lossy(), original);

            CFRelease(cf);
        }
    }

    #[test]
    fn get_selected_text_returns_error_without_focused_app() {
        // Without a focused UI element, should return Err, not crash
        let result = get_selected_text_ax();
        assert!(result.is_err());
    }

    #[test]
    fn is_secure_field_returns_false_without_focused_app() {
        // Without a focused UI element, should return false, not crash
        let result = is_secure_field();
        assert!(!result);
    }
}
