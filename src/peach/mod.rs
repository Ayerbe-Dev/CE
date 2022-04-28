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
use crate::custom::estimate_frame;
use crate::FIGHTER_MANAGER_ADDR;
use crate::FIGHTER_CUTIN_MANAGER_ADDR;
use skyline::nn::ro::LookupSymbol;
use crate::custom::TOTAL_FIGHTER;
use crate::custom::SUB_METER;
use crate::custom::DRAIN_FULL_METER;
use std::mem;


#[acmd_script( agent = "peach", script = "game_attacklw3", category = ACMD_GAME, low_priority)]
unsafe fn peach_attacklw3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.8);
		FighterAreaModuleImpl::enable_fix_jostle_area(module_accessor, 1.0, 4.0);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		FighterAreaModuleImpl::enable_fix_jostle_area(module_accessor, 3.0, 6.0);
	}
	frame(lua_state, 12.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 275, 105, 0, 60, 3.8, 0.0, 3.2, 6.5, Some(0.0), Some(2.8), Some(10.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 80, 105, 0, 15, 3.8, 0.0, 3.2, 6.5, Some(0.0), Some(2.8), Some(10.2), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
		AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		MotionModule::set_rate(module_accessor, 0.7);
		FighterAreaModuleImpl::enable_fix_jostle_area(module_accessor, 3.0, 3.0);
	}
	
}

#[acmd_script( agent = "peach", script = "game_specialshitend", category = ACMD_GAME, low_priority)]
unsafe fn peach_specialshitend(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 361, 66, 0, 60, 7.7, 0.0, 5.0, 4.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 6, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HIP);
	}
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "peach", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn peach_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 8.5, 4.0, Some(0.0), Some(8.5), Some(9.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "peach", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn peach_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 2.6, 0.0, 8.5, 4.0, Some(0.0), Some(8.5), Some(10.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "peach", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn peach_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 8.5, -4.0, Some(0.0), Some(8.5), Some(-14.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "peach", script = "game_final", category = ACMD_GAME, low_priority)]
unsafe fn peach_final(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);

	frame(lua_state, 5.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), false);
		FT_START_CUTIN(fighter);
		SlowModule::set_whole(module_accessor, 2, 0);
	}
	frame(lua_state, 30.0);
	if is_excute (fighter) {
		SlowModule::clear_whole(module_accessor);
	}
	frame (lua_state, 141.0);
	if is_excute(fighter) {
		DamageModule::add_damage(module_accessor, -40.0, 0);
	}	
}

#[acmd_script( agent = "peach", script = "game_finalair", category = ACMD_GAME, low_priority)]
unsafe fn peach_finalair(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);

	frame(lua_state, 5.0);
	if is_excute(fighter) {
		FT_SET_FINAL_FEAR_FACE(fighter, 60);
		REQ_FINAL_START_CAMERA(fighter, Hash40::new("d04final.nuanmb"), false);
		FT_START_CUTIN(fighter);
		SlowModule::set_whole(module_accessor, 2, 0);
	}
	frame(lua_state, 30.0);
	if is_excute (fighter) {
		SlowModule::clear_whole(module_accessor);
	}
	frame(lua_state, 40.0);
	if is_excute (fighter) {
		sv_animcmd::CAM_ZOOM_OUT(lua_state);
	}
	frame (lua_state, 141.0);
	if is_excute(fighter) {
		DamageModule::add_damage(module_accessor, -40.0, 0);
	}
}

#[acmd_script( agent = "peach_kinopiospore", script = "game_shot", category = ACMD_GAME, low_priority)]
unsafe fn peach_kinopiospore_shot(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 45, 210, 0, 40, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
		AttackModule::set_force_reaction(module_accessor, 0, true, false);
	}
	
}

#[fighter_frame( agent = FIGHTER_KIND_PEACH )]
pub fn peach_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);

		if fighter_kind == FIGHTER_KIND_PEACH {
			if WorkModule::is_flag(module_accessor, *FIGHTER_PEACH_INSTANCE_WORK_ID_FLAG_UNIQ_FLOAT) {
				AttackModule::set_power_mul(module_accessor, 0.85);
			}
			if status_kind == *FIGHTER_STATUS_KIND_LANDING 
			|| status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR
			|| status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL
			|| status_kind == *FIGHTER_STATUS_KIND_LANDING_LIGHT
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_AIR
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FALL
			|| status_kind == *FIGHTER_STATUS_KIND_JUMP_AERIAL
			|| status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR
			|| status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N
			|| status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S
			|| status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI {
				AttackModule::set_power_mul(module_accessor, 1.0);
			}
			if motion_kind == hash40("special_lw") && estimate_frame(module_accessor, 0.0) {
				if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE) {
					DRAIN_FULL_METER[get_player_number(module_accessor)] = true;
					SUB_METER[get_player_number(module_accessor)] = 180.0;
				}
				else {
					SUB_METER[get_player_number(module_accessor)] = -20.0;
				}
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		peach_attacklw3,
		peach_specialshitend,
		peach_catch,
		peach_catchdash,
		peach_catchturn,
		peach_final,
		peach_finalair,
		peach_kinopiospore_shot,
	);
	smashline::install_agent_frames!(peach_functions);
}

