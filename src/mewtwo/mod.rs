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
use crate::custom::B_CHECK;


#[acmd_script( agent = "mewtwo", script = "game_attackhi4", category = ACMD_GAME, low_priority)]
unsafe fn mewtwo_attackhi4(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 3.0);
	if is_excute(fighter) {
		SIZE1[get_player_number(module_accessor)] = 4.6;
		SIZE2[get_player_number(module_accessor)] = 4.6;
		SIZE3[get_player_number(module_accessor)] = 3.5;
		SIZE4[get_player_number(module_accessor)] = 4.0;
		SIZE5[get_player_number(module_accessor)] = 6.4;
		SIZE6[get_player_number(module_accessor)] = 6.4;
		SIZE7[get_player_number(module_accessor)] = 5.2;
		SIZE8[get_player_number(module_accessor)] = 4.0;
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
	}
	frame(lua_state, 9.0);
	if is_excute(fighter) {
		ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 368, 100, 90, 0, 3.0, 0.0, 11.7, -6.0, Some(0.0), Some(11.7), Some(7.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
			SIZE1[get_player_number(module_accessor)] += 2.0;
			SIZE2[get_player_number(module_accessor)] += 2.0;
			SIZE3[get_player_number(module_accessor)] += 2.0;
			SIZE4[get_player_number(module_accessor)] += 2.0;
			SIZE5[get_player_number(module_accessor)] += 2.0;
			SIZE6[get_player_number(module_accessor)] += 2.0;
			SIZE7[get_player_number(module_accessor)] += 2.0;
			SIZE8[get_player_number(module_accessor)] += 2.0;
		}
		let hitVec = Vector2f{ x: 0.0, y: 24.0 };
		app::lua_bind::AttackModule::set_vec_target_pos(module_accessor, 2, Hash40{ hash: hash40("top") }, &hitVec, 7.0 as u32, false);
	}
	frame(lua_state, 10.0);
	for _ in 0..3 {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 170, 100, 18, 0, SIZE1[get_player_number(module_accessor)], 0.0, 22.0, -5.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 170, 100, 18, 0, SIZE2[get_player_number(module_accessor)], 0.0, 22.0, 5.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 95, 100, 40, 0, SIZE3[get_player_number(module_accessor)], 0.0, 14.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
			ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 270, 100, 5, 0, SIZE4[get_player_number(module_accessor)], 0.0, 23.6, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				SIZE1[get_player_number(module_accessor)] += 2.0;
				SIZE2[get_player_number(module_accessor)] += 2.0;
				SIZE3[get_player_number(module_accessor)] += 2.0;
				SIZE4[get_player_number(module_accessor)] += 2.0;
				SIZE5[get_player_number(module_accessor)] += 2.0;
				SIZE6[get_player_number(module_accessor)] += 2.0;
				SIZE7[get_player_number(module_accessor)] += 2.0;
				SIZE8[get_player_number(module_accessor)] += 2.0;
			}
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		wait(lua_state, 2.0);
	}
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 80, 111, 0, 60, SIZE5[get_player_number(module_accessor)], 0.0, 22.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 80, 111, 0, 60, SIZE6[get_player_number(module_accessor)], 0.0, 22.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 2, 0, Hash40::new("top"), 10.0, 90, 111, 0, 60, SIZE7[get_player_number(module_accessor)], 0.0, 23.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		ATTACK(fighter, 3, 0, Hash40::new("top"), 10.0, 90, 111, 0, 60, SIZE8[get_player_number(module_accessor)], 0.0, 14.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
	}
	wait(lua_state, 4.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	
}

#[acmd_script( agent = "mewtwo", script = "game_attackairn", category = ACMD_GAME, low_priority)]
unsafe fn mewtwo_attackairn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
		SIZE1[get_player_number(module_accessor)] = 2.5;
		SIZE2[get_player_number(module_accessor)] = 2.5;
		SIZE3[get_player_number(module_accessor)] = 2.5;
		SIZE4[get_player_number(module_accessor)] = 2.5;
		SIZE5[get_player_number(module_accessor)] = 13.5;
	}
	frame(lua_state, 7.0);
	for _ in 0..5 {
		if is_excute(fighter) {
			ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 367, 100, 30, 20, SIZE1[get_player_number(module_accessor)], 0.0, 11.0, 4.1, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
			ATTACK(fighter, 1, 0, Hash40::new("top"), 1.6, 367, 100, 30, 20, SIZE2[get_player_number(module_accessor)], 0.0, 11.0, -6.1, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
			ATTACK(fighter, 2, 0, Hash40::new("top"), 1.6, 367, 100, 60, 20, SIZE3[get_player_number(module_accessor)], 0.0, 2.0, 4.1, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
			ATTACK(fighter, 3, 0, Hash40::new("top"), 1.6, 367, 100, 60, 20, SIZE4[get_player_number(module_accessor)], 0.0, 2.0, -6.1, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
			if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				SIZE1[get_player_number(module_accessor)] += 3.0;
				SIZE2[get_player_number(module_accessor)] += 3.0;
				SIZE3[get_player_number(module_accessor)] += 3.0;
				SIZE4[get_player_number(module_accessor)] += 3.0;
				SIZE5[get_player_number(module_accessor)] += 1.0;
			}
		}
		wait(lua_state, 2.0);
		if is_excute(fighter) {
			AttackModule::clear_all(module_accessor);
		}
		wait(lua_state, 2.0);
	}
	if is_excute(fighter) {
		ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 110, 0, 40, SIZE5[get_player_number(module_accessor)], 0.0, 7.0, -2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
	}
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		AttackModule::clear_all(module_accessor);
	}
	frame(lua_state, 46.0);
	if is_excute(fighter) {
		WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
	}
	
}

#[acmd_script( agent = "mewtwo", script = "game_specialhi", category = ACMD_GAME, low_priority)]
unsafe fn mewtwo_upb(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		GroundModule::select_cliff_hangdata(module_accessor, *FIGHTER_MEWTWO_CLIFF_HANG_DATA_SPECIAL_HI as u32);
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		if ((PostureModule::lr(module_accessor))*(ControlModule::get_stick_x(module_accessor))) < 0.0 {
			PostureModule::reverse_lr(module_accessor);
			PostureModule::update_rot_y_lr(module_accessor);
		}
	}
	frame(lua_state, 16.0);
	if is_excute(fighter) {
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
	}
	
}

#[acmd_script( agent = "mewtwo", script = "game_specialairhi", category = ACMD_GAME, low_priority)]
unsafe fn mewtwo_air_upb(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		GroundModule::select_cliff_hangdata(module_accessor, *FIGHTER_MEWTWO_CLIFF_HANG_DATA_SPECIAL_HI as u32);
		notify_event_msc_cmd!(fighter, 0x2127e37c07u64, *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
	}
	frame(lua_state, 14.0);
	if is_excute(fighter) {
		if ((PostureModule::lr(module_accessor))*(ControlModule::get_stick_x(module_accessor))) < 0.0 {
			PostureModule::reverse_lr(module_accessor);
			PostureModule::update_rot_y_lr(module_accessor);
		}
	}
	frame(lua_state, 15.0);
	if is_excute(fighter) {
		StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
	}
	
}

#[acmd_script( agent = "mewtwo", script = "game_catch", category = ACMD_GAME, low_priority)]
unsafe fn mewtwo_catch(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 7.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 4.2, 0.0, 10.0, 4.5, Some(0.0), Some(10.0), Some(12.1), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "mewtwo", script = "game_catchdash", category = ACMD_GAME, low_priority)]
unsafe fn mewtwo_catchdash(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 10.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 3.3, 0.0, 8.5, 4.5, Some(0.0), Some(8.5), Some(16.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[acmd_script( agent = "mewtwo", script = "game_catchturn", category = ACMD_GAME, low_priority)]
unsafe fn mewtwo_catchturn(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	frame(lua_state, 1.0);
	if is_excute(fighter) {
		GrabModule::set_rebound(module_accessor, true);
	}
	frame(lua_state, 11.0);
	if is_excute(fighter) {
		CATCH(fighter, 0, Hash40::new("top"), 4.2, 0.0, 8.5, -4.5, Some(0.0), Some(8.5), Some(-17.299999), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	game_CaptureCutCommon(fighter);
	wait(lua_state, 2.0);
	if is_excute(fighter) {
		grab!(fighter, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
		GrabModule::set_rebound(module_accessor, false);
	}
	
}

#[fighter_frame( agent = FIGHTER_KIND_MEWTWO )]
pub fn mewtwo_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let mut globals = fighter.globals_mut().clone();

		if fighter_kind == FIGHTER_KIND_MEWTWO {
			if let L2CValueType::Void = globals["whiffed_aerial"].val_type {
				globals["whiffed_aerial"] = false.into();
			}
			if motion_kind == hash40("escape_air") {
				if MotionModule::frame(module_accessor) == 13.0 {
					KineticModule::clear_speed_all(module_accessor);
					KineticModule::unable_energy_all(module_accessor);
				}
				if MotionModule::frame(module_accessor) == 37.0 {
					KineticModule::change_kinetic(module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
					notify_event_msc_cmd!(fighter, 0x2127e37c07u64, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
				}
			}
			if B_CHECK[get_player_number(module_accessor)] {
				if motion_kind == hash40("attack_air_n") {
					if MotionModule::frame(module_accessor) == 10.0 || MotionModule::frame(module_accessor) == 14.0 || MotionModule::frame(module_accessor) == 18.0 || MotionModule::frame(module_accessor) == 22.0 || MotionModule::frame(module_accessor) == 26.0 {
						if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
							MotionModule::set_rate(module_accessor, 1.0);
							globals["whiffed_aerial"] = false.into();
						}
						else {
							DamageModule::add_damage(module_accessor, 0.6, 0);
							globals["whiffed_aerial"] = true.into();
						}
					}
					if globals["whiffed_aerial"].get_bool() {
						if MotionModule::frame(module_accessor) == 28.0 {
							MotionModule::set_rate(module_accessor, 0.5);
							globals["whiffed_aerial"] = false.into();
						}
					}
				}
				if motion_kind == hash40("attack_air_f") {
					if MotionModule::frame(module_accessor) == 8.0 {
						if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) == false {
							MotionModule::set_rate(module_accessor, 0.5);
						}
					}
				}
				if motion_kind == hash40("attack_air_b") {
					if MotionModule::frame(module_accessor) == 17.0 {
						if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) == false {
							MotionModule::set_rate(module_accessor, 0.33);
						}
					}
				}
				if motion_kind == hash40("attack_air_hi") {
					if MotionModule::frame(module_accessor) == 14.0 {
						if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) == false {
							MotionModule::set_rate(module_accessor, 0.5);
						}
					}
				}
				if motion_kind == hash40("attack_air_lw") {
					if MotionModule::frame(module_accessor) >= 49.0 {
						if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) == false {
							StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, false);
						}
					}
				}
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		mewtwo_attackhi4,
		mewtwo_attackairn,
		mewtwo_upb,
		mewtwo_air_upb,
		mewtwo_catch,
		mewtwo_catchdash,
		mewtwo_catchturn
	);
	smashline::install_agent_frames!(mewtwo_functions);
}

