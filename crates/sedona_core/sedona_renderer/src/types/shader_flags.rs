use serde::{Deserialize, Serialize};

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Deserialize, Serialize)]
    pub struct ShaderFlags: u32 {
        const NONE                 = 0;
        const BLEND_ALPHA          = 0b00000001;
        const DOUBLE_SIDED         = 0b00000010;
        const DEPTH_WRITE          = 0b00000100;
        const DEPTH_TEST           = 0b00001000;
        const CULL_DISABLED        = 0b00010000;
        const WIREFRAME            = 0b00100000;
        const ALPHA_TO_COVERAGE    = 0b01000000;
        const DEPTH_COMPARE_LEQUAL = 0b10000000;
    }
}
