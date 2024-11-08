#![allow(non_upper_case_globals)]

use smash::phx::Vector3f;

// Common
pub const ZERO_VECTOR : Vector3f = Vector3f { x: 0.0, y: 0.0, z: 0.0 };

pub const ATTACK_AIR_N_MASK : i32 = 0b00001;
pub const ATTACK_AIR_F_MASK : i32 = 0b00010;
pub const ATTACK_AIR_B_MASK : i32 = 0b00100;
pub const ATTACK_AIR_HI_MASK : i32 = 0b01000;
pub const ATTACK_AIR_LW_MASK : i32 = 0b10000;

pub const ATTACK_N_MASK : i32 = 0b0000001;
pub const ATTACK_S3_MASK : i32 = 0b0000010;
pub const ATTACK_HI3_MASK : i32 = 0b0000100;
pub const ATTACK_LW3_MASK : i32 = 0b0001000;
pub const ATTACK_S4_MASK : i32 = 0b0010000;
pub const ATTACK_HI4_MASK : i32 = 0b0100000;
pub const ATTACK_LW4_MASK : i32 = 0b1000000;

pub mod fighter {
    pub mod instance {
        pub mod flag {
            pub const DISABLE_SPECIAL_N : i32 = 0x0000;
            pub const DISABLE_SPECIAL_S : i32 = 0x0001;
            pub const DISABLE_SPECIAL_HI : i32 = 0x0002;
            pub const DISABLE_SPECIAL_LW : i32 = 0x0003;
            // pub const GUARD_OFF_ATTACK_CANCEL : i32 = 0x0004;
            // pub const IS_FGC : i32 = 0x0005;
            // pub const DODGE_CANCEL : i32 = 0x0006;
            pub const JUMP_FROM_SQUAT : i32 = 0x0006;
            pub const SUPER_JUMP : i32 = 0x0007;
            pub const SUPER_JUMP_SET_MOMENTUM : i32 = 0x0008;
            // pub const FORCE_ESCAPE_AIR_SLIDE : i32 = 0x0008;
            pub const LEDGE_INTANGIBILITY : i32 = 0x0009;
        }
        pub mod int {
            pub const TARGET_ID : i32 = 0x0000;
            pub const USED_GROUND_NORMALS : i32 = 0x0001;
            pub const USED_AERIALS : i32 = 0x0002;
            // pub const CUSTOM_COMMAND_236_STEP : i32 = 0x0003;
            // pub const CUSTOM_COMMAND_236_TIMER : i32 = 0x0004;
            // pub const CUSTOM_COMMAND_214_STEP : i32 = 0x0005;
            // pub const CUSTOM_COMMAND_214_TIMER : i32 = 0x0006;
            // pub const CUSTOM_COMMAND_623_STEP : i32 = 0x0007;
            // pub const CUSTOM_COMMAND_623_TIMER : i32 = 0x0008;
            // pub const CUSTOM_COMMAND_236236_STEP : i32 = 0x0009;
            // pub const CUSTOM_COMMAND_236236_TIMER : i32 = 0x000A;
            pub const JUMP_FROM_SQUAT_COUNT_STATUS : i32 = 0x000B;
        }
        pub mod float {
            pub const FLICK_DOWN : i32 = 0x0000;
            pub const SUPER_JUMP_FRAME : i32 = 0x0001;
        }
    }
    pub mod status {
        pub mod flag {
            pub const JUMP_CANCEL : i32 = 0x1000;
            pub const NORMAL_CANCEL : i32 = 0x1001;
            pub const DASH_CANCEL : i32 = 0x1002;
            pub const SPECIAL_CANCEL : i32 = 0x1003;
            pub const ENABLE_AERIAL_STRING : i32 = 0x1004;
            pub const IS_DASH_CANCEL : i32 = 0x1005;
            pub const SKIP_IS_STATUS_CLIFF_CHECK : i32 = 0x1006;
            pub const FORCE_ESCAPE_AIR_SLIDE_IN_STATUS : i32 = 0x1007;
        }
        pub mod int {
            pub const ENABLED_AERIALS : i32 = 0x1000;
        }
        pub mod float {
            pub const HIT_FRAME : i32 = 0x1000;
        }
    }
}

pub mod weapon {
    pub mod instance {
        pub mod flag {
            pub const FROM_POCKET : i32 = 0x0000;
        }
        pub mod int {
            pub const ORIGINAL_OWNER : i32 = 0x0000;
        }
    }
}

pub mod appeal {
    pub mod flag {
        pub const HOLD : i32 = 0x1050;
        pub const LOOP : i32 = 0x1051;
        pub const ENABLE_ACTION : i32 = 0x1052;
        pub const ENABLE_ACTION_IMM : i32 = 0x1053;
        pub const ACTION_BUTTON_CHECK : i32 = 0x1054;
        pub const ACTION_BUTTON_ENABLE_SUCCESS : i32 = 0x1055;
        pub const ACTION_BUFFER_SUCCESS : i32 = 0x1056;
        pub const ACTION_BUFFER_LOCKED : i32 = 0x1057;
    }
    pub mod int {
        pub const HOLD_BUTTON : i32 = 0x1050;
        pub const ACTION_BUTTON : i32 = 0x1051;
        pub const ACTION_FRAME : i32 = 0x1052;
        pub const RESTART_FRAME : i32 = 0x1053;
    }
    pub mod int64 {
        pub const ACTION_MOT : i32 = 0x1050;
        pub const LOOP_MOT : i32 = 0x1051;
    }
}

pub mod attack {
    pub mod flag {
        pub const INVALID_HOLD_INPUT : i32 = 0x1051;
    }
}

pub mod attack_air {
    pub mod flag {
        pub const WHIFF : i32 = 0x1051;
        pub const ENABLE_LANDING_ATTACK : i32 = 0x1052;
    }
}

pub mod attack_dash {
    pub mod flag {
        pub const ENABLE_AIR_FALL : i32 = 0x1050;
        pub const ENABLE_AIR_CONTINUE : i32 = 0x1051;
        pub const ENABLE_GRAVITY : i32 = 0x1052;
        pub const GRAVITY_ENABLED : i32 = 0x1053;
        pub const ENABLE_AIR_LANDING : i32 = 0x1054;
    }
    pub mod float {
        pub const FALL_SPEED_Y_MUL : i32 = 0x1050;
    }
}

pub mod cliff {
    pub mod flag {
        pub const CLIFF_JUMP_BUTTON : i32 = 0x1050;
        pub const CLIFF_JUMP_MINI : i32 = 0x1051;
    }
}

pub mod damage_fly_roll {
    pub mod flag {
        pub const DISABLE_PASSIVE : i32 = 0x1050;
    }
}

pub mod dash {
    pub mod flag {
        pub const DISABLE_RUN : i32 = 0x1051;
        pub const DISABLE_PIVOT_TURN_DASH : i32 = 0x1052;
    }
}

pub mod escape {
    pub mod flag {
        pub const DODGE_CANCEL : i32 = 0x1050;
    }
}

pub mod escape_air {
    pub mod flag {
        pub const SLIDE_ENABLE_ATTACK : i32 = 0x1050;
        pub const SLIDE_ENABLE_CANCEL : i32 = 0x1051;
    }
}

pub mod guard {
    pub mod flag {
        pub const ADD_BUFFER : i32 = 0x1050;
        pub const SET_SHIELD_LOW_SMOKE : i32 = 0x1051;
    }
    pub mod int {
        pub const SHIELD_EFF_ID : i32 = 0x1050;
        pub const GUARD_OFF_RESERVE_CAT1 : i32 = 0x1051;
    }
}

pub mod thrown {
    pub mod flag {
        pub const FORCE_LAUNCHED : i32 = 0x1051;
    }
}

pub mod ryu {
    pub mod instance {
        pub mod flag {
            pub const DENJIN_CHARGE : i32 = 0x0100;
            pub const DENJIN_RUSH_INHERIT : i32 = 0x0101;
            pub const SKIP_FINAL_PROX_CHECK : i32 = 0x0102;
        }
        pub mod int {
            pub const DENJIN_EFF_HANDLE : i32 = 0x0100;
            pub const RUSH_VC_TYPE : i32 = 0x0101;
            pub const IMPACT_PUNISH_VC_TYPE : i32 = 0x0102;
        }
    }
    pub mod status {
        pub mod flag {
            pub const USED_DENJIN_CHARGE : i32 = 0x1100;
            pub const SET_DENJIN_AURA : i32 = 0x1101;

            pub const SPECIAL_HI_SPECIAL_EFFECT : i32 = 0x1102;

            pub const SPECIAL_LW_RUSH_RESUME_ENERGY : i32 = 0x1102;
            pub const SPECIAL_LW_RUSH_ENABLE_ATTACK : i32 = 0x1103;

            pub const SPECIAL_LW_IMPACT_HIT : i32 = 0x1102;
            pub const SPECIAL_LW_IMPACT_SHIELD : i32 = 0x1103;
            pub const SPECIAL_LW_IMPACT_JUST_SHIELD : i32 = 0x1104;
            pub const SPECIAL_LW_IMPACT_SHIELD_WIND : i32 = 0x1105;
            pub const SPECIAL_LW_IMPACT_ENABLED_ARMOR : i32 = 0x1106;
            pub const SPECIAL_LW_IMPACT_REMOVE_ARMOR : i32 = 0x1107;

            pub const SPECIAL_DECIDE_STRENGTH : i32 = 0x1150;
        }
        pub mod int {
            pub const GUARD_SPECIAL_LW_KIND : i32 = 0x1100;
        }
    }

    pub const GUARD_SPECIAL_LW_KIND_IMPACT : i32 = 0x0;
    pub const GUARD_SPECIAL_LW_KIND_REVERSAL : i32 = 0x1;
}

pub mod ken {
    pub mod instance {
        pub mod flag {
            pub const QUICK_STEP_INHERIT : i32 = 0x0100;

            pub use super::super::super::ryu::instance::flag::SKIP_FINAL_PROX_CHECK;
        }
    }
    pub mod status {
        pub mod flag {
            pub const SPECIAL_N2_GROUND_BRANCH_CHECK : i32 = 0x1100;
            pub const SPECIAL_N2_GROUND_BRANCH_LM : i32 = 0x1101;
            pub const SPECIAL_N2_GROUND_BRANCH_H : i32 = 0x1102;

            pub const SPECIAL_N2_AIR_DISABLE_GRAVITY : i32 = 0x1100;
            pub const SPECIAL_N2_AIR_ENABLE_LANDING : i32 = 0x1101;

            pub use super::super::super::ryu::status::flag::SPECIAL_HI_SPECIAL_EFFECT;

            pub const SPECIAL_LW_ENABLE_ACTION : i32 = 0x1100;
            pub const SPECIAL_LW_UNABLE_ACTION : i32 = 0x1101;
            pub const SPECIAL_LW_ENABLED_ACTION : i32 = 0x1102;
            pub const SPECIAL_LW_RESET_GRAVITY : i32 = 0x1103;

            pub use super::super::super::ryu::status::flag::SPECIAL_DECIDE_STRENGTH;
            pub const QUICK_STEP_INHERITED : i32 = 0x1151;
        }
        pub mod int64 {
            pub const SPECIAL_N2_GROUND_BRANCH_MOTION : i32 = 0x1100;
        }
    }
}
