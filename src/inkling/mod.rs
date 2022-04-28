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
use crate::custom::BULLET_POS;
use crate::custom::INKED;
use crate::custom::ROLLER_POS;
use crate::globals::*;
use crate::custom::{get_player_number, get_attacker_number};
use crate::custom::get_boma;
use std::mem;


#[acmd_script( agent = "inkling", script = "game_attackairb", category = ACMD_GAME, low_priority)]
unsafe fn inkling_attackairb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 91, 0, 17, 5.0, 0.0, 3.0, -16.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 7.5, 361, 91, 0, 18, 3.5, 0.0, 4.7, -9.0, Some(0.0), Some(4.0), Some(-11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_add_reaction_frame(module_accessor, 0, 4.0, false);
        AttackModule::set_add_reaction_frame(module_accessor, 1, 4.0, false);
    }
    wait(lua_state, 5.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        WorkModule::off_flag(module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    
}

#[acmd_script( agent = "inkling", script = "game_specialsstart", category = ACMD_GAME, low_priority)]
unsafe fn inkling_specialsstart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(module_accessor, 3.0, 8.0);
        MotionModule::set_rate(module_accessor, 1.5);
    }
    
}

#[acmd_script( agent = "inkling", script = "game_specialairsstart", category = ACMD_GAME, low_priority)]
unsafe fn inkling_specialairsstart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(module_accessor, 3.0, 8.0);
        MotionModule::set_rate(module_accessor, 1.5);
    }
    
}

#[acmd_script( agent = "inkling", script = "sound_dash", category = ACMD_SOUND, low_priority)]
unsafe fn inkling_dash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if INKED[get_player_number(module_accessor)] != 2 {
            PLAY_SE(fighter, Hash40::new_raw(0x13db0c19a7u64));
            SET_PLAY_INHIVIT(fighter, Hash40::new_raw(0x13db0c19a7u64), 20);
        }
         else {
            PLAY_SE(fighter, Hash40::new_raw(0x158ed7de15u64));
            SET_PLAY_INHIVIT(fighter, Hash40::new_raw(0x158ed7de15u64), 20);
        }
    }
    
}

#[acmd_script( agent = "inkling", script = "sound_turndash", category = ACMD_SOUND, low_priority)]
unsafe fn inkling_turndash_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if INKED[get_player_number(module_accessor)] != 2 {
            PLAY_SE(fighter, Hash40::new_raw(0x13db0c19a7u64));
            SET_PLAY_INHIVIT(fighter, Hash40::new_raw(0x13db0c19a7u64), 20);
        }
         else {
            PLAY_SE(fighter, Hash40::new_raw(0x158ed7de15u64));
            SET_PLAY_INHIVIT(fighter, Hash40::new_raw(0x158ed7de15u64), 20);
        }
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if INKED[get_player_number(module_accessor)] == 2 {
            PLAY_SE(fighter, Hash40::new_raw(0x148fa0d39cu64));
        }
    }
    
}

#[acmd_script( agent = "inkling_roller", script = "game_specialsdash", category = ACMD_GAME, low_priority)]
unsafe fn inkling_roller_dash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("neck"), 8.0, 60, 80, 0, 60, 3.6, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("neck"), 8.0, 60, 80, 0, 60, 3.0, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(module_accessor, 0, 120.0);
        AttackModule::set_ink_value(module_accessor, 1, 120.0);
    }
    
}

#[acmd_script( agent = "inkling_roller", script = "game_specialsrun", category = ACMD_GAME, low_priority)]
unsafe fn inkling_roller_run(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("neck"), 11.0, 60, 40, 0, 60, 3.6, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ATTACK(fighter, 1, 0, Hash40::new("neck"), 11.0, 60, 40, 0, 60, 3.0, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 40, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(module_accessor, 0, 100.0);
        AttackModule::set_ink_value(module_accessor, 1, 100.0);
    }
    
}

#[acmd_script( agent = "inkling_megaphonelaser", script = "game_finalfire", category = ACMD_GAME, low_priority)]
unsafe fn inkling_megaphonelaser_finalfire(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 60, 100, 13, 0, 18.0, 0.0, 10.0, 28.0, Some(0.0), Some(10.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_ink_value(module_accessor, 0, 30.0);
        AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
    }
    for _ in 0..21 {
        if is_excute(fighter) {
            ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x1470762b3du64), 0, false, 0);
        }
        wait(lua_state, 10.0);
    }
    
}

#[acmd_script( agent = "inkling_megaphonelaser", script = "game_finalend", category = ACMD_GAME, low_priority)]
unsafe fn inkling_megaphonelaser_finalend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let module_accessor = sv_system::battle_object_module_accessor(lua_state);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 60, 100, 30, 0, 18.0, 0.0, 10.0, 28.0, Some(0.0), Some(10.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 7, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
    }
    wait(lua_state, 8.0);
    if is_excute(fighter) {
        ControlModule::set_rumble(module_accessor, Hash40::new("rbkind_erase"), 0, false, 0);
    }
    wait(lua_state, 1.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 105, 0, 80, 24.0, 0.0, 10.0, 33.0, Some(0.0), Some(10.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_force_reaction(module_accessor, 0, true, false);
        AttackModule::set_ink_value(module_accessor, 0, 200.0);
        AttackModule::set_final_finish_cut_in(module_accessor, 0, true, true, -1.0, false);
        ControlModule::set_rumble(module_accessor, Hash40::new_raw(0x1407711babu64), 0, false, 0);
    }
    wait(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear_all(module_accessor);
    }
    
}

#[fighter_frame( agent = FIGHTER_KIND_INKLING )]
pub fn inkling_functions(fighter: &mut L2CFighterCommon) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let fighter_kind = app::utility::get_kind(module_accessor) as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let status_kind = fighter.global_table[STATUS_KIND].get_int() as i32;
		let situation_kind = StatusModule::situation_kind(module_accessor);
		let battle_object = app::sv_system::battle_object(fighter.lua_state_agent);
		let instance = mem::transmute::<&mut app::BattleObject, &mut app::Fighter>(battle_object);
		let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
		let mut _globals = fighter.globals_mut().clone();

		if app::utility::get_category(module_accessor) == BATTLE_OBJECT_CATEGORY_FIGHTER {
			if fighter_kind != *FIGHTER_KIND_INKLING || (motion_kind != hash40("dash") && motion_kind != hash40("turn_dash")) {
				INKED[get_player_number(module_accessor)] = 0;
			}
			else if INKED[get_player_number(module_accessor)] == 2 {
				HitModule::set_status_joint(module_accessor, Hash40{hash: hash40("head")}, app::HitStatus(*HIT_STATUS_XLU), 0);
				HitModule::set_status_joint(module_accessor, Hash40{hash: hash40("tank")}, app::HitStatus(*HIT_STATUS_XLU), 0);
				HitModule::set_status_joint(module_accessor, Hash40{hash: hash40("waist")}, app::HitStatus(*HIT_STATUS_XLU), 0);
				HitModule::set_status_joint(module_accessor, Hash40{hash: hash40("bust")}, app::HitStatus(*HIT_STATUS_XLU), 0);
				HitModule::set_status_joint(module_accessor, Hash40{hash: hash40("shoulderl")}, app::HitStatus(*HIT_STATUS_XLU), 0);
				HitModule::set_status_joint(module_accessor, Hash40{hash: hash40("shoulderr")}, app::HitStatus(*HIT_STATUS_XLU), 0);
				HitModule::set_status_joint(module_accessor, Hash40{hash: hash40("arml")}, app::HitStatus(*HIT_STATUS_XLU), 0);
				HitModule::set_status_joint(module_accessor, Hash40{hash: hash40("armr")}, app::HitStatus(*HIT_STATUS_XLU), 0);
			}
			if INKED[get_player_number(module_accessor)] == 2 && fighter_kind == FIGHTER_KIND_INKLING {
				if status_kind == FIGHTER_STATUS_KIND_RUN {
					lua_bind::FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, 1.3);
				}
			}
			if (StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_DASH || StatusModule::prev_status_kind(module_accessor, 0) == *FIGHTER_STATUS_KIND_TURN_DASH) 
			&& status_kind != *FIGHTER_STATUS_KIND_DASH && status_kind != *FIGHTER_STATUS_KIND_TURN_DASH {
				HitModule::set_status_all(module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
			}
			for i in 0..8 {
				for o in 0..9 {
					if BULLET_POS[i][o].x < PostureModule::pos_x(module_accessor) && BULLET_POS[i][o].y > PostureModule::pos_x(module_accessor) && situation_kind == *SITUATION_KIND_GROUND {
						if (BULLET_POS[i][o].z - PostureModule::pos_y(module_accessor)).abs() < 1.0 {
							if o as usize != get_player_number(module_accessor) {
								if INKED[get_player_number(module_accessor)] != 2 {
									INKED[get_player_number(module_accessor)] = 1;
								}
							}
							else if fighter_kind == FIGHTER_KIND_INKLING {
								INKED[get_player_number(module_accessor)] = 2;
							}
						}
					}
				}
			}
		}

		if fighter_kind == FIGHTER_KIND_INKLING {
			if motion_kind == hash40("special_s_jump_end") || motion_kind == hash40("special_air_s_jump_end") {
				MotionModule::set_rate(module_accessor, 2.0);
				if MotionModule::frame(module_accessor) >= 15.0 {
					ArticleModule::remove_exist(module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_ROLLER, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
				}
			}
			for i in 0 .. 10 {
				let ink_range = Vector2f{x: ROLLER_POS[i][get_player_number(module_accessor)].x + 6.1, y: ROLLER_POS[i][get_player_number(module_accessor)].x - 6.1};
				let ink_v_range = Vector2f{x: ROLLER_POS[i][get_player_number(module_accessor)].y + 1.0, y: ROLLER_POS[i][get_player_number(module_accessor)].y - 1.0};
				ROLLER_POS[i][get_player_number(module_accessor)].x = 0.0;
				ROLLER_POS[i][get_player_number(module_accessor)].y = 999.0;
				ROLLER_POS[i][get_player_number(module_accessor)].z = 0.0;
				if PostureModule::pos_x(module_accessor) > ink_range.y 
				&& PostureModule::pos_x(module_accessor) < ink_range.x 
				&& PostureModule::pos_y(module_accessor) > ink_v_range.y 
				&& PostureModule::pos_y(module_accessor) < ink_v_range.x
				&& situation_kind == *SITUATION_KIND_GROUND  {
					INKED[get_player_number(module_accessor)] = 2;
					break;
				}
			}
			for i in 0..8 {
				let ink_bullet_pos = Vector3f{x: BULLET_POS[i][get_player_number(module_accessor)].x + 8.0, y: BULLET_POS[i][get_player_number(module_accessor)].z, z: 0.0};
				let ink_bullet_rot = Vector3f{x: 0.0, y: 90.0, z: 0.0};
				if 	BULLET_POS[i][get_player_number(module_accessor)].w > 0.0 {
					BULLET_POS[i][get_player_number(module_accessor)].w -= 1.0;
					if BULLET_POS[i][get_player_number(module_accessor)].w >= 139.0 {
						if i == 1 || i == 3 || i == 5 || i == 7 {
							EffectModule::req_time(module_accessor, Hash40{hash: hash40("inkling_squid_change")}, 480, &ink_bullet_pos, &ink_bullet_rot, 1.0, *EFFECT_HANDLE_NULL as u32, false, false);
							EffectModule::set_rgb_partial_last(module_accessor, WorkModule::get_float(module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R), WorkModule::get_float(module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G), WorkModule::get_float(module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B));
						}
						else {
							EffectModule::req_time(module_accessor, Hash40{hash: hash40("inkling_squid_splash")}, 480, &ink_bullet_pos, &ink_bullet_rot, 1.0, *EFFECT_HANDLE_NULL as u32, false, false);	
							EffectModule::set_rgb_partial_last(module_accessor, WorkModule::get_float(module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R), WorkModule::get_float(module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G), WorkModule::get_float(module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B));
						}
					}
				}
				else {
					BULLET_POS[i][get_player_number(module_accessor)].x = 0.0;
					BULLET_POS[i][get_player_number(module_accessor)].y = 0.0;
					BULLET_POS[i][get_player_number(module_accessor)].z = 999.0;
				}
				if sv_information::is_ready_go() == false {
					BULLET_POS[i][get_player_number(module_accessor)].x = 0.0;
					BULLET_POS[i][get_player_number(module_accessor)].y = 0.0;
					BULLET_POS[i][get_player_number(module_accessor)].z = 999.0;
					BULLET_POS[i][get_player_number(module_accessor)].w = 0.0;
				}
			}
		}
	}
}

pub unsafe fn get_slot(player_number: usize) -> usize {
	for i in 0..8 {
		if BULLET_POS[i][player_number].w == 0.0 {
			return i as usize;
		}
		else if i == 7 {
			BULLET_POS[0][player_number] = BULLET_POS[1][player_number];
			BULLET_POS[1][player_number] = BULLET_POS[2][player_number];
			BULLET_POS[2][player_number] = BULLET_POS[3][player_number];
			BULLET_POS[3][player_number] = BULLET_POS[4][player_number];
			BULLET_POS[4][player_number] = BULLET_POS[5][player_number];
			BULLET_POS[5][player_number] = BULLET_POS[6][player_number];
			BULLET_POS[6][player_number] = BULLET_POS[7][player_number];
			return i as usize; 	
		}
	}
	return 7 as usize;
}

#[weapon_frame( agent = WEAPON_KIND_INKLING_ROLLERINK )]
fn inkling_rollerink_functions(fighter: &mut L2CFighterBase) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let _instance = app::sv_system::battle_object(fighter.lua_state_agent);
		let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		let weapon_kind = app::utility::get_kind(module_accessor) as i32;
		let _motion_kind = MotionModule::motion_kind(module_accessor);
		let status_kind = StatusModule::status_kind(module_accessor);


		if weapon_kind == WEAPON_KIND_INKLING_ROLLERINK {
			println!("Roller Pos X: {}", WorkModule::get_float(module_accessor, 3));
			println!("Roller Pos Y: {}", WorkModule::get_float(module_accessor, 4));
			println!("Roller Ground Line ID: {}", WorkModule::get_int(module_accessor, 0x10000009 as i32));
			println!("Rollerink Process: {}", WorkModule::is_flag(module_accessor, 0x20000006 as i32));
			println!("Rollerink Disconnect: {}", WorkModule::is_flag(module_accessor, 0x20000009 as i32));
			for i in 0 .. 10 {
				if ROLLER_POS[i][get_player_number(owner_module_accessor)].z == 0.0 {
					ROLLER_POS[i][get_player_number(owner_module_accessor)].x = PostureModule::pos_x(module_accessor);
					ROLLER_POS[i][get_player_number(owner_module_accessor)].y = PostureModule::pos_y(module_accessor);
					ROLLER_POS[i][get_player_number(owner_module_accessor)].z = 1.0;
					break;
				}
			}
		}
	}
}

#[weapon_frame( agent = WEAPON_KIND_INKLING_ROLLER )]
fn inkling_roller_functions(fighter: &mut L2CFighterBase) {
	unsafe {
		let module_accessor = app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
		let weapon_kind = app::utility::get_kind(module_accessor) as i32;
		let motion_kind = MotionModule::motion_kind(module_accessor);
		
		if weapon_kind == *WEAPON_KIND_INKLING_ROLLER {
			if motion_kind == hash40("special_s_jump_end") || motion_kind == hash40("special_air_s_jump_end") {
				MotionModule::set_rate(module_accessor, 2.0);
			}
		}
	}
}

pub fn install() {
    install_acmd_scripts!(
        inkling_attackairb,
        inkling_specialsstart,
        inkling_specialairsstart,
        inkling_dash_sound,
        inkling_turndash_sound,
        inkling_roller_dash,
        inkling_roller_run,
		inkling_megaphonelaser_finalfire,
		inkling_megaphonelaser_finalend
    );
    smashline::install_agent_frames!(inkling_functions);
    smashline::install_agent_frames!(
        inkling_rollerink_functions,
        inkling_roller_functions
    );
}

