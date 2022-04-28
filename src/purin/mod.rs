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
use crate::globals::*;
use crate::custom::{get_player_number, get_attacker_number};
use crate::custom::get_boma;
use crate::FIGHTER_MANAGER_ADDR;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use skyline::nn::ro::LookupSymbol;
use crate::custom::TOTAL_FIGHTER;
use std::mem;
use crate::custom::jump_checker_buffer;


#[acmd_script( agent = "purin", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn purin_attackairb(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.25);
	}
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 138, 120, 0, 0, 4.0, 0.0, 4.0, -13.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 138, 120, 0, 30, 4.5, 0.0, 4.0, -8.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 28.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 52.0);
	if is_excute(fighter) {
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
	}
	
}

#[acmd_script( agent = "purin", script = "game_specialnturn", category = ACMD_GAME, low_priority)]
unsafe fn purin_specialnturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
		JostleModule::set_status(module_accessor, true);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 30, 60, 0, 60, 2.5, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
}

#[acmd_script( agent = "purin", script = "game_specialairnturn", category = ACMD_GAME, low_priority)]
unsafe fn purin_specialairnturn(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
		JostleModule::set_status(module_accessor, true);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 30, 60, 0, 60, 2.5, 0.0, 5.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
}

#[acmd_script( agent = "purin", script = "game_specialairnhitend", category = ACMD_GAME, low_priority)]
unsafe fn purin_specialnhitend(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		JostleModule::set_status(module_accessor, true);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS);
	}
	
}

#[acmd_script( agent = "purin", script = "game_specialhil", category = ACMD_GAME, low_priority)]
unsafe fn purin_specialhil(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 23.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.0, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.0, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 46.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 57.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.0, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.0, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 84.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 97.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.0, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.0, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	if is_excute(fighter) {
		AttackModule::set_size(module_accessor, 0, 16.0);
		AttackModule::set_size(module_accessor, 1, 16.0);
	}
	frame(lua_state, 126.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "purin", script = "game_specialhir", category = ACMD_GAME, low_priority)]
unsafe fn purin_specialhir(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 23.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.0, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.0, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 46.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 57.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.0, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.0, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 84.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 97.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.0, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.0, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	if is_excute(fighter) {
		AttackModule::set_size(module_accessor, 0, 16.0);
		AttackModule::set_size(module_accessor, 1, 16.0);
	}
	frame(lua_state, 126.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "purin", script = "game_specialairhil", category = ACMD_GAME, low_priority)]
unsafe fn purin_specialairhil(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 23.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.0, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.0, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 46.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 57.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.0, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.0, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 84.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 97.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.0, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.0, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	if is_excute(fighter) {
		AttackModule::set_size(module_accessor, 0, 16.0);
		AttackModule::set_size(module_accessor, 1, 16.0);
	}
	frame(lua_state, 126.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "purin", script = "game_specialairhir", category = ACMD_GAME, low_priority)]
unsafe fn purin_specialairhir(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 23.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.0, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.0, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 46.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 57.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.0, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.0, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	frame(lua_state, 84.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 97.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("bust"), 0.0, 361, 40, 0, 0, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("bust"), 0.0, 361, 30, 0, 0, 14.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sleep"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
	}
	if is_excute(fighter) {
		AttackModule::set_size(module_accessor, 0, 16.0);
		AttackModule::set_size(module_accessor, 1, 16.0);
	}
	frame(lua_state, 126.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "purin", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn purin_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.5, 0.0, 5.2, 4.0, Some(0.0), Some(5.2), Some(9.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 0.8);
	}
	
}

#[acmd_script( agent = "purin", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn purin_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 2.8, 0.0, 5.2, 4.0, Some(0.0), Some(5.2), Some(12.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "purin", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn purin_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.5, 0.0, 5.2, -4.0, Some(0.0), Some(5.2), Some(-18.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "purin", script = "game_throwhi", category = ACMD_GAME, low_priority)]
unsafe fn purin_throwhi(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 90, -50, 0, 160, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
		ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
		AttackModule::set_add_reaction_frame(module_accessor, 0, 10.0, true);
	}
	frame(lua_state, 8.0);
	if is_excute(fighter) {
		CHECK_FINISH_CAMERA(fighter, -4, 9);
		let fighter_cutin_manager = *(FIGHTER_CUTIN_MANAGER_ADDR as *mut *mut app::FighterCutInManager);
		lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(fighter_cutin_manager, 1.2);
		lua_bind::FighterCutInManager::set_throw_finish_offset(fighter_cutin_manager, Vector3f{ x: 0.0, y: 2.0, z: 0.0 });
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
	}
	
}

#[acmd_script( agent = "purin", script = "game_finalstartl", category = ACMD_GAME, low_priority)]
unsafe fn purin_finalstartl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 7.0, 120.0);
    }
    if is_excute(fighter) {
        FT_SET_FINAL_FEAR_FACE(fighter, 60);
        REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstartr.nuanmb"), false);
        FT_START_CUTIN(fighter);
    }
    wait(lua_state, 30.0);
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter);
        WorkModule::on_flag(module_accessor, *FIGHTER_PURIN_STATUS_FINAL_FLAG_COMMON);
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 0.0, 0, 100, 75, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 0.0, 0, 100, 75, 0, 3.5, 0.0, -1.0, -1.0, Some(0.0), Some(-1.0), Some(1.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    frame(lua_state, 57.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 0.0, 0, 100, 75, 0, 4.4, 0.0, -1.0, 0.0, Some(0.0), Some(3.2), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    frame(lua_state, 74.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 0.0, 0, 100, 75, 0, 4.4, 0.0, 0.0, -1.0, Some(0.0), Some(0.0), Some(1.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    frame(lua_state, 92.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 0.0, 0, 100, 75, 0, 5.4, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    
}

#[acmd_script( agent = "purin", script = "game_finalstartr", category = ACMD_GAME, low_priority)]
unsafe fn purin_finalstartr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 7.0, 120.0);
    }
    if is_excute(fighter) {
        FT_SET_FINAL_FEAR_FACE(fighter, 60);
        REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstartr.nuanmb"), false);
        FT_START_CUTIN(fighter);
    }
    wait(lua_state, 30.0);
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter);
        WorkModule::on_flag(module_accessor, *FIGHTER_PURIN_STATUS_FINAL_FLAG_COMMON);
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 0.0, 0, 100, 75, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 0.0, 0, 100, 75, 0, 3.5, 0.0, -1.0, -1.0, Some(0.0), Some(-1.0), Some(1.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    frame(lua_state, 57.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 0.0, 0, 100, 75, 0, 4.4, 0.0, -1.0, 0.0, Some(0.0), Some(3.2), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    frame(lua_state, 74.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 0.0, 0, 100, 75, 0, 4.4, 0.0, 0.0, -1.0, Some(0.0), Some(0.0), Some(1.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    frame(lua_state, 92.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 0.0, 0, 100, 75, 0, 5.4, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    
}

#[acmd_script( agent = "purin", script = "game_finalairstartl", category = ACMD_GAME, low_priority)]
unsafe fn purin_finalairstartl(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 7.0, 120.0);
    }
    if is_excute(fighter) {
        FT_SET_FINAL_FEAR_FACE(fighter, 60);
        REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstartr.nuanmb"), false);
        FT_START_CUTIN(fighter);
    }
    wait(lua_state, 30.0);
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter);
        WorkModule::on_flag(module_accessor, *FIGHTER_PURIN_STATUS_FINAL_FLAG_COMMON);
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 0.0, 0, 100, 75, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 0.0, 0, 100, 75, 0, 3.5, 0.0, -1.0, -1.0, Some(0.0), Some(-1.0), Some(1.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    frame(lua_state, 57.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 0.0, 0, 100, 75, 0, 4.4, 0.0, -1.0, 0.0, Some(0.0), Some(3.2), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    frame(lua_state, 74.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 0.0, 0, 100, 75, 0, 4.4, 0.0, 0.0, -1.0, Some(0.0), Some(0.0), Some(1.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    frame(lua_state, 92.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 0.0, 0, 100, 75, 0, 5.4, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    
}

#[acmd_script( agent = "purin", script = "game_finalairstartr", category = ACMD_GAME, low_priority)]
unsafe fn purin_finalairstartr(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
        CHECK_VALID_FINAL_START_CAMERA(fighter, 0, 7, 20, 0, 0, 0);
        SLOW_OPPONENT(fighter, 7.0, 120.0);
    }
    if is_excute(fighter) {
        FT_SET_FINAL_FEAR_FACE(fighter, 60);
        REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04finalstartr.nuanmb"), false);
        FT_START_CUTIN(fighter);
    }
    wait(lua_state, 30.0);
    if is_excute(fighter) {
        CAM_ZOOM_OUT(fighter);
        WorkModule::on_flag(module_accessor, *FIGHTER_PURIN_STATUS_FINAL_FLAG_COMMON);
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 0.0, 0, 100, 75, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 0.0, 0, 100, 75, 0, 3.5, 0.0, -1.0, -1.0, Some(0.0), Some(-1.0), Some(1.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    frame(lua_state, 57.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 0.0, 0, 100, 75, 0, 4.4, 0.0, -1.0, 0.0, Some(0.0), Some(3.2), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    frame(lua_state, 74.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 0.0, 0, 100, 75, 0, 4.4, 0.0, 0.0, -1.0, Some(0.0), Some(0.0), Some(1.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    frame(lua_state, 92.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 0.0, 0, 100, 75, 0, 5.4, 0.0, 0.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 2, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    
}

#[acmd_script( agent = "purin", script = "game_final", category = ACMD_GAME, low_priority)]
unsafe fn purin_final(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 10.0, 60, 80, 10, 40, 4.5, 0.0, -1.0, -1.5, Some(0.0), Some(-1.0), Some(1.8), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.65);
    }
    frame(lua_state, 67.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 12.0);
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 25.0, 50, 65, 0, 60, 6.9, 0.0, 1.6, -2.0, Some(0.0), Some(1.6), Some(2.3), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(module_accessor, 0, true, false);
        AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        AttackModule::clear_all(module_accessor);
    }
    
}

#[acmd_script( agent = "purin", script = "game_finalair", category = ACMD_GAME, low_priority)]
unsafe fn purin_finalair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 10.0, 60, 80, 10, 40, 4.5, 0.0, -1.0, -1.5, Some(0.0), Some(-1.0), Some(1.8), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    wait(lua_state, 6.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
        WHOLE_HIT(fighter, *HIT_STATUS_XLU);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.65);
    }
    frame(lua_state, 67.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 12.0);
        ATTACK(fighter, 0, 0, Hash40::new("rot"), 25.0, 50, 65, 0, 60, 6.9, 0.0, 1.6, -2.0, Some(0.0), Some(1.6), Some(2.3), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(module_accessor, 0, true, false);
        AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
        AttackModule::set_invalid_xlu(module_accessor, 0, true);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
        AttackModule::clear_all(module_accessor);
    }
    
}

#[fighter_frame( agent = FIGHTER_KIND_PURIN )]
pub fn purin_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;

		if fighter_kind == FIGHTER_KIND_PURIN {
			if status_kind == *FIGHTER_PURIN_STATUS_KIND_SPECIAL_N_HOLD {
				let cat = fighter.global_table[CMD_CAT1].get_int() as i32;
				if jump_checker_buffer(module_accessor, cat) {
					if WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
						MotionModule::change_motion(module_accessor, Hash40{hash: hash40("jump_aerial_f2")}, 0.0, 1.0, false, 0.0, false, false);
					}
				}
			}
			if motion_kind == hash40("jump_aerial_f6") {
				MotionModule::change_motion(module_accessor, Hash40{hash: hash40("jump_aerial_f5")}, 0.0, 1.0, false, 0.0, false, false);
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		purin_attackairb,
		purin_specialnturn,
		purin_specialairnturn,
		purin_specialnhitend,
		purin_specialhil,
		purin_specialhir,
		purin_specialairhil,
		purin_specialairhir,
		purin_catch,
		purin_catchdash,
		purin_catchturn,
		purin_throwhi,
		purin_finalstartl,
        purin_finalstartr,
        purin_finalairstartl,
        purin_finalairstartr,
        purin_final,
        purin_finalair,
	);
	smashline::install_agent_frames!(purin_functions);
}

