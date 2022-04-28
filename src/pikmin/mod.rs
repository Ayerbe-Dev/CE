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
use crate::custom::FIGHTER_BOOL_1;
use crate::custom::FIGHTER_BOOL_2;

#[acmd_script( agent = "pikmin_dolfin", script = "game_finalstart", category = ACMD_GAME, low_priority)]
unsafe fn pikmin_dolfin_finalstart(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 12.0);
	}
	frame(lua_state, 36.0);
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
		ATTACK(fighter, 0, 0, Hash40::new("body"), 10.0, 90, 100, 100, 0, 20.0, 0.0, -15.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("body"), 10.0, 270, 100, 868, 0, 20.0, 0.0, -15.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 1, true, true, -1.0, false);
		MotionModule::set_rate(module_accessor, 1.0);
	}
	frame(lua_state, 40.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 0.2);
	}
	frame(lua_state, 45.0);
	if is_excute(fighter) {
		MotionModule::set_rate(module_accessor, 1.0);
	}
	
}

#[acmd_script( agent = "pikmin_dolfin", script = "game_final", category = ACMD_GAME, low_priority)]
unsafe fn pikmin_dolfin_final(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
		ATTACK(fighter, 0, 0, Hash40::new("body"), 10.0, 250, 100, 0, 80, 17.0, 0.0, 15.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("body"), 10.0, 250, 100, 1, 0, 17.0, 0.0, -15.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
		AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 1, true, true, -1.0, false);
	}
	frame(lua_state, 60.0);
	if is_excute(fighter) {
		WorkModule::on_flag(module_accessor, *WEAPON_PIKMIN_DOLFIN_STATUS_FINAL_FLY_WORK_FLAG_FINAL_STATUS_ON);
	}
	
}

#[acmd_script( agent = "pikmin_dolfin", script = "game_finalend", category = ACMD_GAME, low_priority)]
unsafe fn pikmin_dolfin_finalend(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
		ATTACK(fighter, 0, 0, Hash40::new("body"), 10.0, 280, 105, 0, 70, 20.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
	}
	
}

#[acmd_script( agent = "pikmin_dolfin", script = "game_finalexplosion", category = ACMD_GAME, low_priority)]
unsafe fn pikmin_dolfin_finalexplosion(fighter: &mut L2CAgentBase) {
	let lua_state = fighter.lua_state_agent;
	let module_accessor = sv_system::battle_object_module_accessor(lua_state);
	if is_excute(fighter) {
		WHOLE_HIT(fighter, *HIT_STATUS_INVINCIBLE);
		ATTACK(fighter, 0, 0, Hash40::new("body"), 14.0, 361, 120, 0, 65, 35.0, 0.0, 10.0, 0.0, Some(0.0), Some(-10.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
		ATTACK(fighter, 1, 0, Hash40::new("body"), 12.0, 361, 105, 0, 85, 60.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
		AttackModule::set_force_reaction(module_accessor, 0, true, false);
		AttackModule::set_force_reaction(module_accessor, 1, true, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
		AttackModule::set_final_finish_cut_in(module_accessor, 1, true, true, -1.0, false);
	}
	wait(lua_state, 3.0);
	if is_excute(fighter) {
		VisibilityModule::set_whole(module_accessor, false);
	}
	
}

#[fighter_frame( agent = FIGHTER_KIND_PIKMIN )]
pub fn pikmin_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let _fighter_module_accessor = mem::transmute::<&mut app::BattleObjectModuleAccessor, &mut app::FighterModuleAccessor>(module_accessor);
		let fighter_kind = app::utility::get_kind(module_accessor);
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let OLIMAR_PARRY = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
		
		if fighter_kind == FIGHTER_KIND_PIKMIN {
			if motion_kind == hash40("just_shield_off") {
				*OLIMAR_PARRY = true;
			}
			else {
				if StatusModule::prev_status_kind(module_accessor, 0) != *FIGHTER_STATUS_KIND_GUARD_OFF {
					*OLIMAR_PARRY = false;
				}
			}
		}
	}
}

#[weapon_frame( agent = WEAPON_KIND_PIKMIN_PIKMIN )]
fn pikmin_pikmin_functions(fighter: &mut L2CFighterBase) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		let weapon_kind = app::utility::get_kind(module_accessor) as i32;
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let owner_status_kind = StatusModule::status_kind(owner_module_accessor);
		let mut globals = fighter.globals_mut().clone();
		let OLIMAR_PARRY = &mut FIGHTER_BOOL_1[get_player_number(owner_module_accessor)];
		let PIKMIN_XLU = &mut FIGHTER_BOOL_2[get_player_number(owner_module_accessor)];

		if weapon_kind == WEAPON_KIND_PIKMIN_PIKMIN {
			if let L2CValueType::Void = globals["pikmin_hp"].val_type {
				globals["pikmin_hp"] = 1.0.into();
			}
			if let L2CValueType::Void = globals["pikmin_old_hp"].val_type {
				globals["pikmin_old_hp"] = 1.0.into();
			}

			if status_kind == *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_PULL_OUT {
				WorkModule::set_int(module_accessor, 999999, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_HP);
				globals["pikmin_old_hp"] = (WorkModule::get_int(module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_HP) as f32).into();
				if WorkModule::get_int(module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_RED {
					globals["pikmin_hp"] = 6.0.into();
				}
				if WorkModule::get_int(module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_BLUE {
					globals["pikmin_hp"] = 8.0.into();
				}
				if WorkModule::get_int(module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_YELLOW {
					globals["pikmin_hp"] = 6.0.into();
				}
				if WorkModule::get_int(module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_WHITE {
					globals["pikmin_hp"] = 5.0.into();
				}
				if WorkModule::get_int(module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_VIOLET {
					globals["pikmin_hp"] = 11.0.into();
				}
			}
			else {
				if globals["pikmin_old_hp"].get_num() > WorkModule::get_int(module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_HP) as f32 {
					if *PIKMIN_XLU == false {
						globals["pikmin_hp"] = (globals["pikmin_hp"].get_num() - (globals["pikmin_old_hp"].get_num() - WorkModule::get_int(module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_HP) as f32)).into();
					}
					globals["pikmin_old_hp"] = (WorkModule::get_int(module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_HP) as f32).into();
				}
				if globals["pikmin_hp"].get_num() <= 0.0 {
					WorkModule::set_int(module_accessor, -20, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_HP);
				}
				if WorkModule::get_int(module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_INT_HP) < 0 && (
					status_kind == *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_DAMAGE_LANDING
					|| status_kind == *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_WAIT
					|| status_kind == *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_LANDING
					|| status_kind == *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_GROUND_FOLLOW
				) {
					StatusModule::change_status_request_from_script(module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_KIND_DEATH, true);
				}
			}
			if StatusModule::prev_status_kind(owner_module_accessor, 0) == *FIGHTER_STATUS_KIND_GUARD_DAMAGE 
			|| StatusModule::prev_status_kind(owner_module_accessor, 1) == *FIGHTER_STATUS_KIND_GUARD_DAMAGE 
			|| *OLIMAR_PARRY 
			|| owner_status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
			|| owner_status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY {
				*PIKMIN_XLU = true;
			}
			else {
				*PIKMIN_XLU = false;
			}
		}
	}
}

pub fn install() {
	install_acmd_scripts!(
		pikmin_dolfin_finalstart,
		pikmin_dolfin_final,
		pikmin_dolfin_finalend,
		pikmin_dolfin_finalexplosion
	);
	smashline::install_agent_frames!(pikmin_functions);
	smashline::install_agent_frames!(pikmin_pikmin_functions);
}

