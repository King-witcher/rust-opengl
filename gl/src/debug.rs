use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DebugMessageSource(pub u32);
impl DebugMessageSource {
    pub const API: Self = Self(gl46::GL_DEBUG_SOURCE_API.0);
    pub const WINDOW_SYSTEM: Self = Self(gl46::GL_DEBUG_SOURCE_WINDOW_SYSTEM.0);
    pub const SHADER_COMPILER: Self = Self(gl46::GL_DEBUG_SOURCE_SHADER_COMPILER.0);
    pub const THIRD_PARTY: Self = Self(gl46::GL_DEBUG_SOURCE_THIRD_PARTY.0);
    pub const APPLICATION: Self = Self(gl46::GL_DEBUG_SOURCE_APPLICATION.0);
    pub const OTHER: Self = Self(gl46::GL_DEBUG_SOURCE_OTHER.0);
}

impl Display for DebugMessageSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.0 {
            x if x == gl46::GL_DEBUG_SOURCE_API.0 => "API",
            x if x == gl46::GL_DEBUG_SOURCE_WINDOW_SYSTEM.0 => "WINDOW_SYSTEM",
            x if x == gl46::GL_DEBUG_SOURCE_SHADER_COMPILER.0 => "SHADER_COMPILER",
            x if x == gl46::GL_DEBUG_SOURCE_THIRD_PARTY.0 => "THIRD_PARTY",
            x if x == gl46::GL_DEBUG_SOURCE_APPLICATION.0 => "APPLICATION",
            x if x == gl46::GL_DEBUG_SOURCE_OTHER.0 => "OTHER",
            _ => "UNKNOWN",
        };
        write!(f, "{}", s)
    }
}

impl From<GLenum> for DebugMessageSource {
    fn from(source: GLenum) -> Self {
        DebugMessageSource(source.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DebugMessageType(pub u32);
impl DebugMessageType {
    pub const ERROR: Self = Self(gl46::GL_DEBUG_TYPE_ERROR.0);
    pub const DEPRECATED_BEHAVIOR: Self = Self(gl46::GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR.0);
    pub const UNDEFINED_BEHAVIOR: Self = Self(gl46::GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR.0);
    pub const PORTABILITY: Self = Self(gl46::GL_DEBUG_TYPE_PORTABILITY.0);
    pub const PERFORMANCE: Self = Self(gl46::GL_DEBUG_TYPE_PERFORMANCE.0);
    pub const OTHER: Self = Self(gl46::GL_DEBUG_TYPE_OTHER.0);
    pub const MARKER: Self = Self(gl46::GL_DEBUG_TYPE_MARKER.0);
}

impl Display for DebugMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.0 {
            x if x == gl46::GL_DEBUG_TYPE_ERROR.0 => "ERROR",
            x if x == gl46::GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR.0 => "DEPRECATED_BEHAVIOR",
            x if x == gl46::GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR.0 => "UNDEFINED_BEHAVIOR",
            x if x == gl46::GL_DEBUG_TYPE_PORTABILITY.0 => "PORTABILITY",
            x if x == gl46::GL_DEBUG_TYPE_PERFORMANCE.0 => "PERFORMANCE",
            x if x == gl46::GL_DEBUG_TYPE_OTHER.0 => "OTHER",
            x if x == gl46::GL_DEBUG_TYPE_MARKER.0 => "MARKER",
            _ => "UNKNOWN",
        };
        write!(f, "{}", s)
    }
}

impl From<GLenum> for DebugMessageType {
    fn from(r#type: GLenum) -> Self {
        DebugMessageType(r#type.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DebugMessageSeverity(pub u32);
impl DebugMessageSeverity {
    pub const HIGH: Self = Self(gl46::GL_DEBUG_SEVERITY_HIGH.0);
    pub const MEDIUM: Self = Self(gl46::GL_DEBUG_SEVERITY_MEDIUM.0);
    pub const LOW: Self = Self(gl46::GL_DEBUG_SEVERITY_LOW.0);
    pub const NOTIFICATION: Self = Self(gl46::GL_DEBUG_SEVERITY_NOTIFICATION.0);
}

impl Display for DebugMessageSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.0 {
            x if x == gl46::GL_DEBUG_SEVERITY_HIGH.0 => "HIGH",
            x if x == gl46::GL_DEBUG_SEVERITY_MEDIUM.0 => "MEDIUM",
            x if x == gl46::GL_DEBUG_SEVERITY_LOW.0 => "LOW",
            x if x == gl46::GL_DEBUG_SEVERITY_NOTIFICATION.0 => "NOTIFICATION",
            _ => "UNKNOWN",
        };
        write!(f, "{}", s)
    }
}

impl From<GLenum> for DebugMessageSeverity {
    fn from(severity: GLenum) -> Self {
        DebugMessageSeverity(severity.0)
    }
}

pub type DebugMessageCallback = fn(
    source: DebugMessageSource,
    r#type: DebugMessageType,
    id: u32,
    severity: DebugMessageSeverity,
    message: &str,
    user_param: isize,
) -> ();

pub fn debug_message_callback(callback: Option<DebugMessageCallback>, user_param: isize) {
    static mut CALLBACK: Option<DebugMessageCallback> = None;

    unsafe extern "system" fn middleware(
        source: GLenum,
        r#type: GLenum,
        id: u32,
        severity: GLenum,
        message_size: i32,
        message: *const u8,
        user_param: *const c_void,
    ) {
        unsafe {
            let slice = std::slice::from_raw_parts(message, message_size as usize);
            let str = std::str::from_utf8(slice).unwrap();

            CALLBACK.expect("Debug message callback not set")(
                source.into(),
                r#type.into(),
                id,
                severity.into(),
                str,
                user_param as isize,
            );
        }
    }

    unsafe {
        let Some(callback) = callback else {
            gl().DebugMessageCallback(None, std::ptr::null());
            return;
        };
        CALLBACK = Some(callback);
        gl().DebugMessageCallback(Some(middleware), user_param as *const c_void);
    }
}
