use crate::imports::status_imports::*;
use super::super::helper::*;

unsafe extern "C" fn ryu_attack_s3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion = if VarModule::is_flag(fighter.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE)
    || fighter.global_table[PREV_STATUS_KIND].get_i32() == *FIGHTER_STATUS_KIND_TURN_RUN {
        *FIGHTER_KINETIC_TYPE_MOTION_RUN_STOP
    }
    else {
        *FIGHTER_KINETIC_TYPE_MOTION
    };
    StatusModule::init_settings(
        fighter.module_accessor,
        SituationKind(*SITUATION_KIND_GROUND),
        motion,
        *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32,
        GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        true,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_NO_REAC,
        false,
        false,
        false,
        *FIGHTER_LOG_MASK_FLAG_ACTION_CATEGORY_KEEP as u64,
        0,
        *FIGHTER_POWER_UP_ATTACK_BIT_ATTACK_3 as u32,
        0
    );
    0.into()
}

unsafe extern "C" fn ryu_attack_s3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_RUSH_INHERIT) {
        VarModule::on_flag(fighter.module_accessor, ryu::status::flag::USED_DENJIN_CHARGE);
        VarModule::off_flag(fighter.module_accessor, ryu::instance::flag::DENJIN_RUSH_INHERIT);
    }
    ryu_attack_s3_main_inner(fighter)
}

pub fn install(agent: &mut smashline::Agent) {
    agent.status(smashline::Pre, *FIGHTER_STATUS_KIND_ATTACK_S3, ryu_attack_s3_pre);
    agent.status(smashline::Main, *FIGHTER_STATUS_KIND_ATTACK_S3, ryu_attack_s3_main);
}