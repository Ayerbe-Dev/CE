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
use crate::custom::FIGHTER_VEC2F_1;
use crate::custom::FINAL_TRANSFORM;


#[acmd_script( agent = "zelda", script = "game_attacklw3", category = ACMD_GAME, low_priority)]
unsafe fn zelda_attacklw3(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 5.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 80, 120, 0, 20, 2.6, 0.0, 2.0, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 80, 120, 0, 20, 2.6, 0.0, 1.7, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 5.5, 80, 120, 0, 20, 2.6, 0.0, 1.5, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
		AttackModule::set_attack_height_all(module_accessor, app::AttackHeight(*ATTACK_HEIGHT_LOW), false);
	}
	wait(lua_state, 7.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "zelda", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn zelda_attackairn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 4.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 6.0);
	for _ in 0..4 {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 70, 80, 0, 50, 5.0, 0.0, 10.0, 7.0, None, None, None, 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 367, 10, 0, 30, 1.5, 0.0, 6.5, 5.0, Some(0.0), Some(6.5), Some(10.0), 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 367, 10, 0, 15, 1.5, 0.0, 13.0, 5.0, Some(0.0), Some(13.0), Some(10.0), 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
			ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 70, 80, 0, 50, 5.0, 0.0, 13.0, -5.0, None, None, None, 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
			ATTACK(fighter, 4, 0, Hash40::new("top"), 1.5, 367, 10, 0, 30, 1.5, 0.0, 9.5, -3.0, Some(0.0), Some(9.5), Some(-8.0), 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
			ATTACK(fighter, 5, 0, Hash40::new("top"), 1.5, 367, 10, 0, 15, 1.5, 0.0, 16.0, -3.0, Some(0.0), Some(16.0), Some(-8.0), 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		wait(lua_state, 2.0);
	}
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 130, 0, 35, 5.0, 0.0, 13.5, -7.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 361, 130, 0, 35, 5.0, 0.0, 9.0, 8.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 361, 110, 0, 35, 6.0, 0.0, 7.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		ATTACK(fighter, 3, 0, Hash40::new("head"), 5.0, 361, 110, 0, 35, 6.0, 0.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 38.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "zelda", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn zelda_attackairb(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	frame(lua_state, 6.0);
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 361, 96, 0, 30, 2.2, -1.0, 4.4, -11.1, Some(1.0), Some(4.4), Some(-11.1), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_magic"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_MAGIC);
		ATTACK(fighter, 1, 0, Hash40::new("kneer"), 4.0, 361, 80, 0, 5, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
		ATTACK(fighter, 2, 0, Hash40::new("hip"), 4.0, 361, 80, 0, 5, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
		ATTACK(fighter, 3, 0, Hash40::new("toer"), 4.0, 361, 96, 0, 5, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
		AttackModule::set_optional_hit_effect(module_accessor, 0, Hash40{ hash: 0x1484fa7486 });
	}
	wait(lua_state, 1.0);
	if is_excute(fighter) {
		AttackModule::clear(module_accessor, 0, false);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
		MotionModule::set_rate(module_accessor, 1.09);
	}
	frame(lua_state, 38.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "zelda", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn zelda_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.8, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(11.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "zelda", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn zelda_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 13.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.0, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(13.3), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "zelda", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn zelda_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.8, 0.0, 9.0, -4.0, Some(0.0), Some(9.0), Some(-19.200001), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "zelda", script = "game_finalloop", category = ACMD_GAME, low_priority)]
unsafe fn zelda_finalloop(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		FINAL_TRANSFORM[get_player_number(module_accessor)] = 240;
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
	}
}

#[acmd_script( agent = "zelda", script = "game_finalairloop", category = ACMD_GAME, low_priority)]
unsafe fn zelda_finalairloop(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		FINAL_TRANSFORM[get_player_number(module_accessor)] = 240;
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
	}
}

#[acmd_script( agent = "zelda", script = "game_finalfinish", category = ACMD_GAME, low_priority)]
unsafe fn zelda_finalfinish(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_ZELDA_STATUS_FINAL_FLAG_FINISH);
	}
	
}

#[acmd_script( agent = "zelda", script = "game_finalairfinish", category = ACMD_GAME, low_priority)]
unsafe fn zelda_finalairfinish(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 20.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_ZELDA_STATUS_FINAL_FLAG_FINISH);
	}
	
}

#[acmd_script( agent = "zelda_triforce", script = "game_inhale", category = ACMD_GAME, low_priority)]
unsafe fn zelda_triforce_inhale(fighter: &mut L2CAgentBase) {
	if is_excute(fighter) {
        AREA_WIND_2ND_RAD_arg9(fighter, 0, 4, -0.1, 1000, 1, 0, 0, 70, 80);
	}
}

#[acmd_script( agent = "zelda_triforce", script = "game_start", category = ACMD_GAME, low_priority)]
unsafe fn zelda_triforce_start(fighter: &mut L2CAgentBase) {}


#[fighter_frame( agent = FIGHTER_KIND_ZELDA )]
pub fn zelda_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let situation_kind = StatusModule::situation_kind(module_accessor);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;

		if fighter_kind == FIGHTER_KIND_ZELDA {
			if (motion_kind == hash40("special_s_loop") || motion_kind == hash40("special_air_s_loop")) && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
				if situation_kind == *SITUATION_KIND_GROUND {
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);                        
				}
				else {
					StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
				}
			}
			if (motion_kind == hash40("special_lw") || motion_kind == hash40("special_air_lw")) && MotionModule::frame(module_accessor) >= 10.0 {
				if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
					if situation_kind == *SITUATION_KIND_GROUND {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, true);
					}
					else {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, true);
					}
				}
			}
			if FINAL_TRANSFORM[get_player_number(module_accessor)] != 0 {
				if FINAL_TRANSFORM[get_player_number(module_accessor)] == 1 {
					if WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI) {
						StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_ZELDA_STATUS_KIND_FINAL_END, true);
						notify_event_msc_cmd!(fighter, 0x1e0aba2d68u64, 7);
						FINAL_TRANSFORM[get_player_number(module_accessor)] -= 1;
					} 
				}
				else {
					FINAL_TRANSFORM[get_player_number(module_accessor)] -= 1;
				}
			}
		}
	}
}

#[weapon_frame( agent = WEAPON_KIND_ZELDA_DEIN )]
fn zelda_dein_functions(fighter: &mut L2CFighterBase) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		let weapon_kind = app::utility::get_kind(module_accessor) as i32;
		let owner_status_kind = StatusModule::status_kind(owner_module_accessor);
		let DEIN_POS = &mut FIGHTER_VEC2F_1[get_player_number(owner_module_accessor)];

		if weapon_kind == *WEAPON_KIND_ZELDA_DEIN || weapon_kind == *WEAPON_KIND_ZELDA_DEIN_S {
			if owner_status_kind == FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_LOOP || owner_status_kind == FIGHTER_STATUS_KIND_SPECIAL_S {
				*DEIN_POS = Vector2f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor)};
			}
			else {
				let correct_kind = app::GroundCorrectKind(GroundModule::get_correct(module_accessor));
				GroundModule::set_correct(module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
				PostureModule::set_pos_2d(module_accessor, &*DEIN_POS);
				GroundModule::set_correct(module_accessor, correct_kind);
			}
		}
	}
}

#[weapon_frame( agent = WEAPON_KIND_ZELDA_DEIN_S )]
fn zelda_dein_s_functions(fighter: &mut L2CFighterBase) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		let weapon_kind = app::utility::get_kind(module_accessor) as i32;
		let owner_status_kind = StatusModule::status_kind(owner_module_accessor);
		let DEIN_POS = &mut FIGHTER_VEC2F_1[get_player_number(owner_module_accessor)];

		if weapon_kind == *WEAPON_KIND_ZELDA_DEIN || weapon_kind == *WEAPON_KIND_ZELDA_DEIN_S {
			if owner_status_kind == FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_LOOP || owner_status_kind == FIGHTER_STATUS_KIND_SPECIAL_S {
				*DEIN_POS = Vector2f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor)};
			}
			else {
				let correct_kind = app::GroundCorrectKind(GroundModule::get_correct(module_accessor));
				GroundModule::set_correct(module_accessor, app::GroundCorrectKind(*GROUND_CORRECT_KIND_NONE));
				PostureModule::set_pos_2d(module_accessor, &*DEIN_POS);
				GroundModule::set_correct(module_accessor, correct_kind);
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		zelda_attacklw3,
		zelda_attackairn,
		zelda_attackairb,
		zelda_catch,
		zelda_catchdash,
		zelda_catchturn,
		zelda_finalloop,
		zelda_finalairloop,
		zelda_finalfinish,
		zelda_finalairfinish,
		zelda_triforce_inhale,
		zelda_triforce_start,
	);
	smashline::install_agent_frames!(zelda_functions);
	smashline::install_agent_frames!(
		zelda_dein_functions,
		zelda_dein_s_functions
	);
}

