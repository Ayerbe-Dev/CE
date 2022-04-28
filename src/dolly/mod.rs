// porter? what porter? who's blujay?
use smash::lua2cpp::L2CAgentBase;
use smash_script::{*, macros::*};
use smashline::*;
use smash::hash40;
use smash::app::{self, lua_bind::*, sv_animcmd::{frame, wait}, *};
use smash::lib::{lua_const::*, L2CValueType::*, L2CValueType, L2CAgent, L2CValue, L2CTable, L2CTable_meta, L2CInnerFunctionBase, L2CValueInner};
use smash::phx::*;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use crate::custom::SIZE1;
use crate::custom::SIZE2;
use crate::custom::SIZE3;
use crate::custom::SIZE4;
use crate::custom::SIZE5;
use crate::custom::SIZE6;
use crate::custom::SIZE7;
use crate::custom::SIZE8;
use crate::custom::SIZE9;
use crate::custom::FIGHTER_BOOL_1;
use crate::globals::*;
use crate::custom::{get_player_number, get_attacker_number};
use crate::custom::get_boma;
use crate::custom::FS_METER;
use crate::custom::METER_ENABLED;
pub static mut CHARGE_DA: [bool;9] = [false;9];

#[acmd_script( agent = "dolly", script = "game_attack11", category = ACMD_GAME, low_priority)]
unsafe fn dolly_attack11(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);	
    frame(lua_state, 0.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
        MotionModule::set_rate(module_accessor, 1.5);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 10, 0, 10, 3.2, 0.0, 11.0, 5.5, None, None, None, 1.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 180, 10, 0, 30, 3.2, 0.0, 11.0, 10.0, None, None, None, 1.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(module_accessor, 1, 1.0, false);
        MotionModule::set_rate(module_accessor, 1.0);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::off_flag(module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
    
}

#[acmd_script( agent = "dolly", script = "game_attacks3", category = ACMD_GAME, low_priority)]
unsafe fn dolly_attacks3(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 0.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 37, 40, 0, 53, 3.5, 0.0, 11.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
		if !AttackModule::is_infliction_status(module_accessor, *COLLISION_CATEGORY_MASK_ALL) {
			WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
		}
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 37, 40, 0, 53, 3.5, 0.0, 11.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 37, 40, 0, 53, 3.5, 0.0, 10.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 37, 36, 0, 53, 4.0, 0.0, 10.0, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 37, 40, 0, 53, 3.5, 0.0, 11.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 37, 40, 0, 53, 3.5, 0.0, 11.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 37, 36, 0, 53, 4.0, 0.0, 11.0, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
        WorkModule::off_flag(module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
    }
    
}

#[acmd_script( agent = "dolly", script = "game_attackdash", category = ACMD_GAME, low_priority)]
unsafe fn dolly_attackdash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		if CHARGE_DA[get_player_number(module_accessor)] {
			MotionModule::set_rate(module_accessor, 1.5);
		}
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		if CHARGE_DA[get_player_number(module_accessor)] {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 15.6, 55, 78, 0, 65, 5.0, 0.0, 10.0, 3.0, Some(0.0), Some(6.0), Some(3.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_BODY);
		}
		else {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 55, 78, 0, 65, 5.0, 0.0, 10.0, 3.0, Some(0.0), Some(6.0), Some(3.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_BODY);
		}		
    }
    wait(lua_state, 3.0);
    if is_excute(fighter) {
		if CHARGE_DA[get_player_number(module_accessor)] {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 50, 83, 0, 60, 4.0, 0.0, 10.0, 4.0, Some(0.0), Some(6.0), Some(4.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_BODY);
		}
		else {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 50, 83, 0, 60, 4.0, 0.0, 10.0, 4.0, Some(0.0), Some(6.0), Some(4.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_BODY);
		}
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
		if CHARGE_DA[get_player_number(module_accessor)] {
			MotionModule::set_rate(module_accessor, 1.2);
		}
		CHARGE_DA[get_player_number(module_accessor)] = false;
    }
    
}

#[acmd_script( agent = "dolly", script = "game_attacks4", category = ACMD_GAME, low_priority)]
unsafe fn dolly_attacks4(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 2.0);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 18.0, 361, 73, 0, 60, 4.0, 0.0, 10.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 18.0, 361, 73, 0, 60, 4.0, 0.0, 10.0, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
        HIT_NODE(fighter, Hash40::new("legr"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("kneer"), *HIT_STATUS_NORMAL);
    }
    wait(lua_state, 10.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    
}

#[acmd_script( agent = "dolly", script = "game_attackairf", category = ACMD_GAME, low_priority)]
unsafe fn dolly_attackairf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 2.0);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 48, 60, 0, 45, 3.0, 0.0, 4.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 48, 60, 0, 45, 3.0, 0.0, 2.0, 7.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 11.0, 48, 60, 0, 45, 3.5, 0.0, -1.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 48, 60, 0, 45, 3.0, 0.0, 4.0, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 48, 60, 0, 45, 3.0, 0.0, 2.0, 7.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 8.0, 48, 60, 0, 45, 3.5, 0.0, -1.0, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "dolly", script = "game_attackairhi", category = ACMD_GAME, low_priority)]
unsafe fn dolly_attackairhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
	}
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("legr"), 9.0, 70, 100, 0, 45, 4.4, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneer"), 9.0, 70, 100, 0, 45, 6.0, 3.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("legr"), 9.0, 75, 100, 0, 45, 4.4, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("kneer"), 9.0, 75, 100, 0, 45, 6.0, 3.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 18.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		WorkModule::off_flag(module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL);
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "dolly", script = "game_specialsbattackw", category = ACMD_GAME, low_priority)]
unsafe fn dolly_specialbattackw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
 
	frame(lua_state, 3.0);
	if WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
		if is_excute(fighter) {
			ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 25, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 4.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 10, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 25, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 4.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 25, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 4.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 10, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 25, /*Size*/ 6.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 4.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			
			let crackVec1 = Vector3f{x: -1.0, y: 1.5, z: 0.0};
			KineticModule::add_speed(module_accessor, &crackVec1);
		}
	}
	else {
		if is_excute(fighter) {
			ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 4.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 10, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 4.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 4.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 10, /*KBG*/ 100, /*FKB*/ 80, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 4.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
	}
	frame(lua_state, 4.0);
	if WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
		if is_excute(fighter) {
			ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 15.0, /*Z*/ -2.5, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 2.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 15.0, /*Z*/ -2.5, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 2.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
	}
	else {
		if is_excute(fighter) {
			ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 15.0, /*Z*/ -2.5, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 2.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 0, /*KBG*/ 100, /*FKB*/ 110, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 15.0, /*Z*/ -2.5, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 8.0, /*Z*/ 2.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
	}
	frame(lua_state, 6.0);
	if WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
		if is_excute(fighter) {
			ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 1.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 1.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
	}
	else {
		if is_excute(fighter) {
			ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 1.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 350, /*KBG*/ 100, /*FKB*/ 90, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 1.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 25, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);			
		}
	}
	frame(lua_state, 8.0);
	if WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
		if is_excute(fighter) {
			ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 20, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 1.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 20, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 45, /*BKB*/ 20, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 1.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 20, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			KineticModule::clear_speed_all(module_accessor);
		}
	}
	else {
		if is_excute(fighter) {
			ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 20, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 1.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 20, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 340, /*KBG*/ 100, /*FKB*/ 45, /*BKB*/ 20, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 16.0, /*Z*/ 1.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 20, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			KineticModule::clear_speed_all(module_accessor);
		}
	}
	frame(lua_state, 10.0);
	if WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			let crackVec2 = Vector3f{x: -1.0, y: -1.5, z: 0.0};
			KineticModule::add_speed(module_accessor, &crackVec2);
		}
	}
	else {
		if is_excute(fighter) {
			ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 20, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 3.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 20, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 1.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_G, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 4.0, /*Angle*/ 0, /*KBG*/ 100, /*FKB*/ 60, /*BKB*/ 20, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 3.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK); 
			ATTACK(fighter, /*ID*/ 3, /*Part*/ 0, /*Bone*/ Hash40::new("top"), /*Damage*/ 3.0, /*Angle*/ 30, /*KBG*/ 100, /*FKB*/ 30, /*BKB*/ 20, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 7.0, /*Z*/ 1.0, None, None, None, /*Hitlag*/ 1.2, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_A, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
	}
	frame(lua_state, 11.0);
	if !WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
			ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 68, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 95, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 14.0, /*Z*/ 5.0, None, None, None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 6.0, /*Angle*/ 68, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 95, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 5.0, /*Z*/ 1.0, /*X2*/ Some(0.0), /*Y2*/ Some(8.0), /*Z2*/ Some(1.0), /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
	}
	frame(lua_state, 12.0);
	if WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
		if is_excute(fighter) {
			ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 270, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 8.0, None, None, None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 270, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, None, None, None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			AttackModule::set_add_reaction_frame(module_accessor, /*ID*/ 0, /*Frames*/ 10.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(module_accessor, /*ID*/ 1, /*Frames*/ 10.0, /*Unk*/ false);
		}
	}
	else {
		if is_excute(fighter) {
			ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 68, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 95, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 8.0, None, None, None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 68, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 95, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 9.0, /*Z*/ 3.0, None, None, None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
		}
	}
	frame(lua_state, 13.0);
	if WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
		if is_excute(fighter) {
			ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 270, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 8.0, None, None, None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 270, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 3.0, None, None, None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 8, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			AttackModule::set_add_reaction_frame(module_accessor, /*ID*/ 0, /*Frames*/ 9.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(module_accessor, /*ID*/ 1, /*Frames*/ 9.0, /*Unk*/ false);
		}
	}
	else {
		if is_excute(fighter) {
			ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 68, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 95, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 8.0, None, None, None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 68, /*KBG*/ 40, /*FKB*/ 0, /*BKB*/ 95, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 3.0, None, None, None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);	
		}
	}
	frame(lua_state, 15.0);
	if WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
		if is_excute(fighter) {
			ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 270, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 8.0, None, None, None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 270, /*KBG*/ 0, /*FKB*/ 0, /*BKB*/ 45, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 3.0, None, None, None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ true, /*ShieldDamage*/ 10, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			AttackModule::set_add_reaction_frame(module_accessor, /*ID*/ 0, /*Frames*/ 8.0, /*Unk*/ false);
			AttackModule::set_add_reaction_frame(module_accessor, /*ID*/ 1, /*Frames*/ 8.0, /*Unk*/ false);
		}
	}
	else {
		if is_excute(fighter) {
			ATTACK(fighter, /*ID*/ 0, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 78, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 95, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 8.0, None, None, None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);
			ATTACK(fighter, /*ID*/ 1, /*Part*/ 1, /*Bone*/ Hash40::new("top"), /*Damage*/ 8.0, /*Angle*/ 78, /*KBG*/ 50, /*FKB*/ 0, /*BKB*/ 95, /*Size*/ 5.0, /*X*/ 0.0, /*Y*/ 6.0, /*Z*/ 3.0, None, None, None, /*Hitlag*/ 1.5, /*SDI*/ 1.0, /*Clang/Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct/Indirect*/ true, /*Ground/Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_normal"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_DOLLY_KICK, /*Type*/ *ATTACK_REGION_KICK);		
		}
	}
	frame(lua_state, 17.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		KineticModule::clear_speed_all(module_accessor);
	}
}

#[acmd_script( agent = "dolly", script = "game_specialairlwrisew", category = ACMD_GAME, low_priority)]
unsafe fn dolly_speciallww(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let PD_CHECK = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
	let dunkVec9 = Vector3f{ x: 5.0, y: -1.5, z: 0.0 };
	frame(lua_state, 1.0);
	if WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
		if is_excute(fighter) {
			WHOLE_HIT(fighter, *HIT_STATUS_XLU);
			WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
			KineticModule::clear_speed_all(module_accessor);
			WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 85, 100, 50, 50, 7.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				*PD_CHECK = true;
				SIZE1[get_player_number(module_accessor)] = 13.0;
			}
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) == false {
				SIZE1[get_player_number(module_accessor)] = 8.0;
				*PD_CHECK = false;
			}
		}
	}
	if WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) == false {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
			KineticModule::clear_speed_all(module_accessor);
			WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 85, 100, 50, 50, 7.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				*PD_CHECK = true;
				SIZE1[get_player_number(module_accessor)] = 13.0;
			}
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) == false {
				SIZE1[get_player_number(module_accessor)] = 8.0;
				*PD_CHECK = false;
			}
		}
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 367, 100, 40, 10, SIZE1[get_player_number(module_accessor)], 0.0, 10.0, 3.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			*PD_CHECK = true;
			SIZE1[get_player_number(module_accessor)] = 13.0;
		}
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 50, 100, 30, 10, SIZE1[get_player_number(module_accessor)], 0.0, 10.0, 3.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			*PD_CHECK = true;
			SIZE1[get_player_number(module_accessor)] = 13.0;
		}
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::off_flag(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
	}
	frame(lua_state, 22.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
		KineticModule::add_speed(module_accessor, &dunkVec9);
		WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
	}
	
}

#[acmd_script( agent = "dolly", script = "game_specialairlwrise", category = ACMD_GAME, low_priority)]
unsafe fn dolly_speciallw(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let PD_CHECK = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
	let dunkVec = Vector3f{ x: 0.3, y: -1.5, z: 0.0 };
	if WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
		if is_excute(fighter) {
			WHOLE_HIT(fighter, *HIT_STATUS_XLU);
			WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
			KineticModule::clear_speed_all(module_accessor);
			WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 85, 100, 40, 50, 7.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				*PD_CHECK = true;
				SIZE1[get_player_number(module_accessor)] = 13.0;
			}
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) == false {
				SIZE1[get_player_number(module_accessor)] = 8.0;
				*PD_CHECK = false;
			}
		}
	}
	if WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) == false {
		if is_excute(fighter) {
			WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
			WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
			KineticModule::clear_speed_all(module_accessor);
			WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
			ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 85, 100, 40, 50, 7.0, 0.0, 10.0, 3.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				*PD_CHECK = true;
				SIZE1[get_player_number(module_accessor)] = 13.0;
			}
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) == false {
				SIZE1[get_player_number(module_accessor)] = 8.0;
				*PD_CHECK = false;
			}
		}
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 367, 100, 60, 20, SIZE1[get_player_number(module_accessor)], 0.0, 9.0, 3.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 367, 100, 40, 10, SIZE1[get_player_number(module_accessor)], 0.0, 9.0, 3.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			*PD_CHECK = true;
			SIZE1[get_player_number(module_accessor)] = 13.0;
		}
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 50, 100, 40, 10, SIZE1[get_player_number(module_accessor)], 0.0, 10.0, 3.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_KICK, *ATTACK_REGION_KICK);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 4.0, false);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			*PD_CHECK = true;
			SIZE1[get_player_number(module_accessor)] = 13.0;
		}
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		WorkModule::off_flag(module_accessor, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL);
		WHOLE_HIT(fighter, *HIT_STATUS_NORMAL);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
		KineticModule::add_speed(module_accessor, &dunkVec);
		WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
	}
	
}

#[acmd_script( agent = "dolly", script = "game_specialairlw", category = ACMD_GAME, low_priority)]
unsafe fn dolly_speciallwattack(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let PD_CHECK = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
	frame(lua_state, 0.0);
	if is_excute(fighter) {
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
			WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
			let dunkVec = Vector3f{x: 0.3, y: -1.0, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
			WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
			WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
			let dunkVec = Vector3f{x: 1.3, y: -1.5, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
			WorkModule::off_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
		}
	}
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		if WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
			if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
				let dunkVec = Vector3f{x: 0.0, y: -0.3, z: 0.0};
				KineticModule::add_speed(module_accessor, &dunkVec);
				if *PD_CHECK {
					ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 50, 70, 0, 80, 13.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
				}
			}
			if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
				let dunkVec = Vector3f{x: 0.0, y: -1.0, z: 0.0};
				KineticModule::add_speed(module_accessor, &dunkVec);
				if *PD_CHECK {
					ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 50, 65, 0, 80, 13.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
				}
			}
		}
		if WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) == false {
			if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
				let dunkVec = Vector3f{x: 0.0, y: -0.3, z: 0.0};
				KineticModule::add_speed(module_accessor, &dunkVec);
				if *PD_CHECK {
					ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 50, 75, 0, 80, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
				}
			}
			if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
				let dunkVec = Vector3f{x: 0.0, y: -1.0, z: 0.0};
				KineticModule::add_speed(module_accessor, &dunkVec);
				if *PD_CHECK {
					ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 50, 70, 0, 80, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
				}
			}
		}
	}
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
			let dunkVec = Vector3f{x: 0.0, y: 0.0, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
			*PD_CHECK = false;
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
			let dunkVec = Vector3f{x: 0.0, y: -0.5, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
			*PD_CHECK = false;
		}
	}
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		if WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
			if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 50, 70, 0, 80, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
			}
			if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 50, 65, 0, 80, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) == false {
			if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 50, 75, 0, 80, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
			}
			if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 50, 70, 0, 80, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
	}
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
			let dunkVec = Vector3f{x: 0.0, y: 0.05, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
			let dunkVec = Vector3f{x: 0.0, y: 0.2, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
		}		
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		if WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
			if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
				MotionModule::set_rate(module_accessor, 1.2);
				let dunkVec = Vector3f{x: 0.0, y: 0.05, z: 0.0};
				KineticModule::add_speed(module_accessor, &dunkVec);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 50, 80, 0, 60, 4.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
			}
			if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
				MotionModule::set_rate(module_accessor, 1.0);
				let dunkVec = Vector3f{x: 0.0, y: 0.2, z: 0.0};
				KineticModule::add_speed(module_accessor, &dunkVec);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 310, 95, 0, 30, 5.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_PUNCH);
			}
		}
		if WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) == false {
			if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
				MotionModule::set_rate(module_accessor, 1.2);
				let dunkVec = Vector3f{x: 0.0, y: 0.05, z: 0.0};
				KineticModule::add_speed(module_accessor, &dunkVec);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 50, 80, 0, 60, 4.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
			}
			if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
				MotionModule::set_rate(module_accessor, 1.0);
				let dunkVec = Vector3f{x: 0.0, y: 0.2, z: 0.0};
				KineticModule::add_speed(module_accessor, &dunkVec);
				ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 50, 80, 0, 60, 4.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
			let dunkVec = Vector3f{x: 0.0, y: 0.05, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
			let dunkVec = Vector3f{x: 0.0, y: 0.2, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
		}		
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
			let dunkVec = Vector3f{x: 0.0, y: 0.05, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
			let dunkVec = Vector3f{x: 0.0, y: 0.2, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
		}		
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
			let dunkVec = Vector3f{x: 0.0, y: 0.05, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
			let dunkVec = Vector3f{x: 0.0, y: 0.2, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
		}		
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
			let dunkVec = Vector3f{x: 0.0, y: 0.05, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
			let dunkVec = Vector3f{x: 0.0, y: 0.2, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
		}		
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
			let dunkVec = Vector3f{x: 0.0, y: 0.05, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
			let dunkVec = Vector3f{x: 0.0, y: 0.2, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
		}		
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
			let dunkVec = Vector3f{x: 0.0, y: 0.0, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
			let dunkVec = Vector3f{x: 0.0, y: 0.2, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
		}		
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
			let dunkVec = Vector3f{x: 0.0, y: 0.0, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
			let dunkVec = Vector3f{x: 0.0, y: 0.2, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
		}		
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
			let dunkVec = Vector3f{x: 0.0, y: 0.0, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
		}
		if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_S {
			let dunkVec = Vector3f{x: 0.0, y: 0.2, z: 0.0};
			KineticModule::add_speed(module_accessor, &dunkVec);
		}		
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		if WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_FLAG_COMMAND) {
			if WorkModule::get_int(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_COMMON_WORK_INT_STRENGTH) == *FIGHTER_DOLLY_STRENGTH_W {
				ATTACK(fighter, 0, 0, Hash40::new("top"), 14.0, 50, 80, 0, 60, 4.0, 0.0, 6.0, 7.0, Some(0.0), Some(4.0), Some(2.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_PUNCH);
			}
		}
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 10.0);
	}
	frame(lua_state, 35.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_DOLLY_SPECIAL_LW_FALL);
		MotionModule::set_rate(module_accessor, 1.0);
	}
	frame(lua_state, 37.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_STATUS_SPECIAL_LW_WORK_FLAG_LANDING_HEAVY);
	}
	
}

#[acmd_script( agent = "dolly", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn dolly_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(10.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "dolly", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn dolly_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.6, 0.0, 9.0, 5.0, Some(0.0), Some(9.0), Some(11.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "dolly", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn dolly_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 7.2, -5.0, Some(0.0), Some(7.2), Some(-13.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "dolly_wave", script = "game_normalairw", category = ACMD_GAME, low_priority)]
unsafe fn dolly_wave_normalairw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 30, 0, 85, 3.0, 0.0, 10.0, -2.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 45, 30, 0, 85, 3.0, 0.0, 0.0, -7.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 30, 0, 85, 3.0, 0.0, 9.4, -0.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 45, 30, 0, 85, 3.0, 0.0, 0.0, -8.6, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 30, 0, 85, 3.0, 0.0, 8.8, 0.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 45, 30, 0, 85, 3.0, 0.0, 0.0, -10.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 30, 0, 85, 3.0, 0.0, 8.2, 1.6, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 45, 30, 0, 85, 3.0, 0.0, 0.0, -11.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 30, 0, 85, 3.0, 0.0, 7.6, 2.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 45, 30, 0, 85, 3.0, 0.0, 0.0, -13.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 30, 0, 85, 3.0, 0.0, 7.0, 4.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 45, 30, 0, 85, 3.0, 0.0, 0.0, -15.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 30, 0, 85, 3.0, 0.0, 6.4, 5.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 45, 30, 0, 85, 3.0, 0.0, 0.0, -16.6, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 30, 0, 85, 3.0, 0.0, 5.8, 6.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 45, 30, 0, 85, 3.0, 0.0, 0.0, -18.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 30, 0, 85, 3.0, 0.0, 5.2, 7.6, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 45, 30, 0, 85, 3.0, 0.0, 0.0, -19.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 30, 0, 85, 3.0, 0.0, 4.6, 8.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 45, 30, 0, 85, 3.0, 0.0, 0.0, -21.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    
}

#[acmd_script( agent = "dolly_wave", script = "game_normalair", category = ACMD_GAME, low_priority)]
unsafe fn dolly_wave_normalair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 45, 30, 0, 85, 3.0, 0.0, 10.0, -2.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 45, 30, 0, 85, 3.0, 0.0, 0.0, -7.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 45, 30, 0, 85, 3.0, 0.0, 9.2, -0.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 45, 30, 0, 85, 3.0, 0.0, 0.0, -9.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 45, 30, 0, 85, 3.0, 0.0, 8.4, 1.6, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 45, 30, 0, 85, 3.0, 0.0, 0.0, -11.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 45, 30, 0, 85, 3.0, 0.0, 7.6, 3.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 45, 30, 0, 85, 3.0, 0.0, 0.0, -14.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 45, 30, 0, 85, 3.0, 0.0, 6.8, 5.2, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 45, 30, 0, 85, 3.0, 0.0, 0.0, -16.6, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 45, 30, 0, 85, 3.0, 0.0, 6.0, 7.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 45, 30, 0, 85, 3.0, 0.0, 0.0, -19.0, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.0, 45, 30, 0, 85, 3.0, 0.0, 5.2, 8.8, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.0, 45, 30, 0, 85, 3.0, 0.0, 0.0, -21.4, Some(0.0), Some(0.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3.2, 0.0, 0, true, true, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_ENERGY);
    }
    
}

unsafe fn interp_damage(module_accessor: &mut app::BattleObjectModuleAccessor, damage: f32) -> f32 {
	let mut multiplier = 200.0 / FS_METER[get_player_number(module_accessor)];
	if !METER_ENABLED {
		multiplier = 1.0;
	}
	return damage / multiplier;
}

#[acmd_script( agent = "dolly", script = "game_finalstart", category = ACMD_GAME, low_priority)]
unsafe fn dolly_finalstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, true);
		sv_animcmd::FT_SET_FINAL_SMASH_LIGHT(lua_state);
		fighter.clear_lua_stack();
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 15.0, 40.0);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 40);
		REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstart.nuanmb"), true);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 35.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 40.0);
	if is_excute(fighter) {
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RESET);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, 45, 0, 0, 0);
		ArticleModule::generate_article(module_accessor, *FIGHTER_DOLLY_GENERATE_ARTICLE_BURST, false, 0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), interp_damage(module_accessor, 6.0), 20, 100, 25, 30, 11.0, 0.0, 10.0, 18.0, Some(0.0), Some(39.0), Some(28.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BOMB);
		AttackModule::set_no_dead_all(module_accessor, true, false);
		MotionModule::set_rate(module_accessor, 1.5);
	}
	frame(lua_state, 50.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 60.0);
	if is_excute(fighter) {
		ATTACK_ABS(fighter, *FIGHTER_DOLLY_ATTACK_ABSOLUTE_KIND_FINAL, 0, interp_damage(module_accessor, 6.0), 361, 80, 1, 0, 0.5, 1.0, *ATTACK_LR_CHECK_POS, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, true);
		WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_STATUS_FINAL_WORK_FLAG_ABS_HIT);
		ArticleModule::generate_article(module_accessor, *FIGHTER_DOLLY_GENERATE_ARTICLE_BURST, false, 0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), interp_damage(module_accessor, 6.0), 20, 100, 25, 30, 11.0, 0.0, 10.0, 33.0, Some(0.0), Some(39.0), Some(43.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BOMB);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 70.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 90.0);
	if is_excute(fighter) {
		ATTACK_ABS(fighter, *FIGHTER_DOLLY_ATTACK_ABSOLUTE_KIND_FINAL, 0, interp_damage(module_accessor, 12.0), 361, 80, 1, 0, 0.3, 1.0, *ATTACK_LR_CHECK_POS, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, true);
		WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_STATUS_FINAL_WORK_FLAG_ATTACK_ABS_FINAL3);
		WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_STATUS_FINAL_WORK_FLAG_ABS_HIT);
		ArticleModule::generate_article(module_accessor, *FIGHTER_DOLLY_GENERATE_ARTICLE_BURST, false, 0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), interp_damage(module_accessor, 18.0), 45, 75, 0, 33, 11.0, 0.0, 10.0, 48.0, Some(0.0), Some(39.0), Some(58.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BOMB);
		AttackModule::set_no_dead_all(module_accessor, true, false);
		MotionModule::set_rate(module_accessor, 1.0);
	}
	frame(lua_state, 100.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		if !AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			FS_METER[get_player_number(module_accessor)] = 0.0;
		}
	}
	frame(lua_state, 120.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	
}

#[acmd_script( agent = "dolly", script = "game_finalairstart", category = ACMD_GAME, low_priority)]
unsafe fn dolly_finalairstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		fighter.clear_lua_stack();
		lua_args!(fighter, true);
		sv_animcmd::FT_SET_FINAL_SMASH_LIGHT(lua_state);
		fighter.clear_lua_stack();
		CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
		SLOW_OPPONENT(fighter, 15.0, 40.0);
	}
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 40);
		REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstart.nuanmb"), true);
		FT_START_CUTIN(fighter);
	}
	frame(lua_state, 35.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	frame(lua_state, 40.0);
	if is_excute(fighter) {
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RESET);
		camera!(fighter, *MA_MSC_CMD_CAMERA_CAM_RECT, 45, 0, 0, 0);
		ArticleModule::generate_article(module_accessor, *FIGHTER_DOLLY_GENERATE_ARTICLE_BURST, false, 0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), interp_damage(module_accessor, 6.0), 20, 100, 25, 30, 11.0, 0.0, 10.0, 18.0, Some(0.0), Some(39.0), Some(28.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BOMB);
		AttackModule::set_no_dead_all(module_accessor, true, false);
		MotionModule::set_rate(module_accessor, 1.5);
	}
	frame(lua_state, 50.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 60.0);
	if is_excute(fighter) {
		ATTACK_ABS(fighter, *FIGHTER_DOLLY_ATTACK_ABSOLUTE_KIND_FINAL, 0, interp_damage(module_accessor, 6.0), 361, 80, 1, 0, 0.5, 1.0, *ATTACK_LR_CHECK_POS, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, true);
        WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_STATUS_FINAL_WORK_FLAG_ABS_HIT);
		ArticleModule::generate_article(module_accessor, *FIGHTER_DOLLY_GENERATE_ARTICLE_BURST, false, 0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), interp_damage(module_accessor, 6.0), 20, 100, 25, 30, 11.0, 0.0, 10.0, 33.0, Some(0.0), Some(39.0), Some(43.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BOMB);
		AttackModule::set_no_dead_all(module_accessor, true, false);
	}
	frame(lua_state, 70.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 90.0);
	if is_excute(fighter) {
		ATTACK_ABS(fighter, *FIGHTER_DOLLY_ATTACK_ABSOLUTE_KIND_FINAL, 0, interp_damage(module_accessor, 12.0), 361, 80, 1, 0, 0.3, 1.0, *ATTACK_LR_CHECK_POS, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_CRITICAL, *ATTACK_REGION_NONE);
		AttackModule::set_no_dead_all(module_accessor, true, true);
		WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_STATUS_FINAL_WORK_FLAG_ATTACK_ABS_FINAL3);
        WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_STATUS_FINAL_WORK_FLAG_ABS_HIT);
		ArticleModule::generate_article(module_accessor, *FIGHTER_DOLLY_GENERATE_ARTICLE_BURST, false, 0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), interp_damage(module_accessor, 18.0), 45, 75, 0, 33, 11.0, 0.0, 10.0, 48.0, Some(0.0), Some(39.0), Some(58.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BOMB);
		AttackModule::set_no_dead_all(module_accessor, true, false);
		MotionModule::set_rate(module_accessor, 1.0);
	}
	frame(lua_state, 100.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		if !AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			FS_METER[get_player_number(module_accessor)] = 0.0;
		}
	}
	frame(lua_state, 120.0);
	if is_excute(fighter) {
		CAM_ZOOM_OUT(fighter);
	}
	
}

#[acmd_script( agent = "dolly", script = "game_visualscene02", category = ACMD_GAME, low_priority )]
unsafe fn dolly_visualscene02(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_DOLLY_ATTACK_ABSOLUTE_KIND_FINAL, 0, interp_damage(module_accessor, 6.0), 361, 80, 1, 0, 0.0, 1.0, *ATTACK_LR_CHECK_POS, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(module_accessor, true, true);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_STATUS_FINAL_SCENE_WORK_FLAG_HIT_ABS);
    }
    
}

#[acmd_script( agent = "dolly", script = "game_visualscene05", category = ACMD_GAME, low_priority )]
unsafe fn dolly_visualscene05(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ATTACK_ABS(fighter, *FIGHTER_DOLLY_ATTACK_ABSOLUTE_KIND_FINAL, 0, interp_damage(module_accessor, 6.0), 361, 80, 1, 0, 0.0, 1.0, *ATTACK_LR_CHECK_POS, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_no_dead_all(module_accessor, true, true);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_STATUS_FINAL_SCENE_WORK_FLAG_HIT_ABS);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_STATUS_FINAL_SCENE_WORK_FLAG_HIT_ABS);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_STATUS_FINAL_SCENE05_WORK_FLAG_DAMAGE_FLY_OPPONENT);
    }
    
}

#[acmd_script( agent = "dolly", script = "game_finalend", category = ACMD_GAME, low_priority )]
unsafe fn dolly_finalend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        ATTACK_ABS(fighter, *FIGHTER_DOLLY_ATTACK_ABSOLUTE_KIND_FINAL, 0, interp_damage(module_accessor, 10.0), 60, 144, 0, 58, 0.0, 1.0, *ATTACK_LR_CHECK_POS, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_STATUS_FINAL_WORK_FLAG_ABS_SET);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_VISUAL_SCENE_FLAG_END_EXIT);
		FS_METER[get_player_number(module_accessor)] = 0.0;
    }
    
}

#[acmd_script( agent = "dolly", script = "game_finalairend", category = ACMD_GAME, low_priority )]
unsafe fn dolly_finalairend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        ATTACK_ABS(fighter, *FIGHTER_DOLLY_ATTACK_ABSOLUTE_KIND_FINAL, 0, interp_damage(module_accessor, 10.0), 60, 144, 0, 58, 0.0, 1.0, *ATTACK_LR_CHECK_POS, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        WorkModule::on_flag(module_accessor, *FIGHTER_DOLLY_STATUS_FINAL_WORK_FLAG_ABS_SET);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_VISUAL_SCENE_FLAG_END_EXIT);
		FS_METER[get_player_number(module_accessor)] = 0.0;
    }
    
}

#[acmd_script( agent = "dolly_burst", script = "game_final", category = ACMD_GAME, low_priority)]
unsafe fn dolly_burst_final(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		if AttackModule::is_infliction_status(owner_module_accessor, *COLLISION_KIND_MASK_HIT) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 20, 100, 25, 30, 11.0, 0.0, 10.0, -1.0, Some(0.0), Some(39.0), Some(9.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BOMB);
			AttackModule::set_no_dead_all(module_accessor, true, false);	
		}
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "dolly_burst", script = "game_final2", category = ACMD_GAME, low_priority)]
unsafe fn dolly_burst_final2(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		if AttackModule::is_infliction_status(owner_module_accessor, *COLLISION_KIND_MASK_HIT) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 20, 100, 25, 30, 11.0, 0.0, 10.0, -1.0, Some(0.0), Some(39.0), Some(9.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BOMB);
		AttackModule::set_no_dead_all(module_accessor, true, false);
		}
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "dolly_burst", script = "game_final3", category = ACMD_GAME, low_priority)]
unsafe fn dolly_burst_final3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	frame(lua_state, 2.0);
	if is_excute(fighter) {
		if AttackModule::is_infliction_status(owner_module_accessor, *COLLISION_KIND_MASK_HIT) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 45, 75, 0, 33, 11.0, 0.0, 10.0, -1.0, Some(0.0), Some(39.0), Some(9.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DOLLY_PUNCH, *ATTACK_REGION_BOMB);
			AttackModule::set_no_dead_all(module_accessor, true, false);
		}
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

pub unsafe fn get_command_stick_direction(module_accessor: &mut app::BattleObjectModuleAccessor) -> i32 {
	let status_kind = StatusModule::status_kind(module_accessor);
	let mut stick_x = ControlModule::get_stick_x(module_accessor) * PostureModule::lr(module_accessor);
	if status_kind == *FIGHTER_STATUS_KIND_TURN_RUN {
		stick_x *= -1.0;
	}
	if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_CSTICK_ON) {
		return 5;
	}

	if stick_x >= 0.2 {
		if ControlModule::get_stick_y(module_accessor) <= -0.2 {
			return 3;
		}
		else {
			return 6;
		}
	}
	else if stick_x <= -0.2 {
		if ControlModule::get_stick_y(module_accessor) <= -0.2 {
			return 1;
		}
		else {
			return 4;
		}
	}
	else {
		if ControlModule::get_stick_y(module_accessor) <= -0.2 {
			return 2;
		}
		else {
			return 0;
		}
	}
}

#[fighter_frame( agent = FIGHTER_KIND_DOLLY )]
pub fn dolly_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let mut globals = fighter.globals_mut().clone();
		let cat = fighter.global_table[CMD_CAT1].get_int() as i32;
		let cat4 = fighter.global_table[CMD_CAT4].get_int() as i32;
		let situation_kind = fighter.global_table[SITUATION_KIND].get_int() as i32;
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;

		if fighter_kind == FIGHTER_KIND_DOLLY {
			if let L2CValueType::Void = globals["back_charge"].val_type {
				globals["back_charge"] = 0.0.into();
			}
			if let L2CValueType::Void = globals["back_charge_timer"].val_type {
				globals["back_charge_timer"] = 0.0.into();
			}
			if motion_kind == hash40("attack_13") {
				if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_ALL) {
					if (cat & *FIGHTER_PAD_CMD_CAT1_FLAG_SPECIAL_N) != 0 {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_N, true);
					}
				}
			}

			if situation_kind == *SITUATION_KIND_GROUND {
				//Enable super specials in exchange for 100 points of meter while meter is enabled
				
				if METER_ENABLED {
					WorkModule::set_flag(module_accessor, FS_METER[get_player_number(module_accessor)] >= 100.0, *FIGHTER_DOLLY_INSTANCE_WORK_ID_FLAG_ENABLE_SUPER_SPECIAL);
				}


				if get_command_stick_direction(module_accessor) == 4 || get_command_stick_direction(module_accessor) == 1 || get_command_stick_direction(module_accessor) == 7 {
					globals["back_charge_timer"] = 0.0.into();
					if globals["back_charge"].get_num() < 24.0 {
						globals["back_charge"] = (globals["back_charge"].get_num() + 1.0).into();
					}
				}
				else {
					if globals["back_charge_timer"].get_num() == 10.0 {
						globals["back_charge"] = 0.0.into();
					}
					else {
						globals["back_charge_timer"] = (globals["back_charge_timer"].get_num() + 1.0).into();
					}
				}

				if status_kind != *FIGHTER_STATUS_KIND_ATTACK_DASH && CHARGE_DA[get_player_number(module_accessor)] {
					globals["back_charge"] = 0.0.into();
					globals["back_charge_timer"] = 10.0.into();
					CHARGE_DA[get_player_number(module_accessor)] = false;
				}


				if get_command_stick_direction(module_accessor) == 3 && globals["back_charge"].get_num() == 24.0 {
					if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
						if WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_DASH) || (AttackModule::is_infliction_status(module_accessor, *COLLISION_CATEGORY_MASK_ALL) 
						&& WorkModule::is_flag(module_accessor, *FIGHTER_DOLLY_STATUS_ATTACK_WORK_FLAG_HIT_CANCEL)) || (status_kind != *FIGHTER_STATUS_KIND_ATTACK_DASH && MotionModule::frame(module_accessor) < 1.0) {
							CHARGE_DA[get_player_number(module_accessor)] = true;
							StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_DASH, true);
						}
					}
				}
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		dolly_attack11,
        dolly_attacks3,
		dolly_attackdash,
        dolly_attacks4,
        dolly_attackairf,
		dolly_attackairhi,
		dolly_specialbattackw,
		dolly_speciallww,
		dolly_speciallw,
		dolly_speciallwattack,
		dolly_catch,
		dolly_catchdash,
		dolly_catchturn,
		dolly_burst_final,
		dolly_burst_final2,
		dolly_burst_final3,
		dolly_wave_normalairw,
        dolly_wave_normalair,
		dolly_finalstart,
		dolly_finalairstart,
		dolly_visualscene02,
        dolly_visualscene05,
        dolly_finalend,
        dolly_finalairend
	);
	smashline::install_agent_frames!(
		dolly_functions
	);
}

