use boa_string::{CodePoint, CommonJsStringBuilder};

pub(crate) trait PushCodePoint {
    fn push_code_point(&mut self, cp: CodePoint);
}

impl PushCodePoint for CommonJsStringBuilder<'_> {
    fn push_code_point(&mut self, cp: CodePoint) {
        let mut buf = [0u16; 2];
        let buf = &*cp.encode_utf16(&mut buf);
        self.push(buf);
    }
}
