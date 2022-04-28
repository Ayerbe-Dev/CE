#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(unused_imports)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![feature(c_variadic)]
#![feature(asm)]
use smash::app::{self, lua_bind::*, sv_battle_object::*, BattleObjectModuleAccessor, smashball};
use smash::app::sv_animcmd::{self};
use smash::app::{sv_information, sv_math};
use smash::phx::*;
use smash::{hash40, lua_State};
use smash::lib::{self, L2CAgent, L2CValue, lua_const::*};
use smash::lua2cpp::{L2CFighterBase, L2CFighterCommon};
use skyline::from_c_str;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
use skyline::hooks::InlineCtx;
use skyline::nro::NroInfo;
use skyline::patching::patch_str;
use crate::globals::*;
use crate::custom::{get_player_number, get_attacker_number};
use crate::custom::get_boma;
use crate::inkling::get_slot;
use crate::brave::spell_ok;
use crate::brave::spell_check;
use crate::custom::EXTRA_LAG;
use crate::custom::PARRIED_COUNT;
use crate::custom::GIGA_GRAB;
use crate::custom::GIGA_GRABBED;
use crate::custom::REWOUND;
use crate::custom::MONADO_TIMER;
use crate::custom::MONADO_STATUS;
use crate::custom::REWIND_ENDING;
use crate::custom::B_CHECK;
use crate::custom::DEMON_DEATH_TIMER;
use crate::custom::FIGHTER_U64_1;
use crate::custom::FIGHTER_U8_1;
use crate::custom::FIGHTER_U8_2;
use crate::custom::FIGHTER_U8_3;
use crate::custom::FIGHTER_U8_4;
use crate::custom::FIGHTER_INT_1;
use crate::custom::FIGHTER_INT_2;
use crate::custom::FIGHTER_INT_3;
use crate::custom::FIGHTER_INT_4;
use crate::custom::FIGHTER_INT_5;
use crate::custom::FIGHTER_INT_6;
use crate::custom::FIGHTER_INT_7;
use crate::custom::FIGHTER_INT_8;
use crate::custom::FIGHTER_INT_9;
use crate::custom::FIGHTER_INT_11;
use crate::custom::FIGHTER_FLOAT_1;
use crate::custom::FIGHTER_FLOAT_2;
use crate::custom::FIGHTER_FLOAT_3;
use crate::custom::FIGHTER_VEC3F_1;
use crate::custom::FIGHTER_VEC2F_1;
use crate::custom::FIGHTER_BOOL_1;
use crate::custom::FIGHTER_BOOL_2;
use crate::custom::FIGHTER_BOOL_3;
use crate::custom::FIGHTER_BOOL_4;
use crate::custom::FIGHTER_BOOL_5;
use crate::custom::FIGHTER_BOOL_6;
use crate::custom::FIGHTER_BOOL_7;
use crate::custom::FIGHTER_BOOL_8;
use crate::custom::FIGHTER_BOOL_9;
use crate::custom::TRAINING_SPAWN;
use crate::custom::AERIAL_KIND;
use crate::custom::ATTACK_CANCELED;
use crate::custom::INKED;
use crate::custom::LAST_DAMAGE;
use crate::custom::COUNTERHIT_CHECK;
use crate::custom::COUNTERHIT_SUCCESS;
use crate::custom::BULLET_POS;
use crate::custom::TOTAL_FIGHTER;
use crate::custom::INKLING_PRESENT;
use crate::custom::MODEL_DATA_POS;
use crate::custom::FIGHTER_NAME;
use crate::custom::READY_GO_TIMER;
use crate::custom::SPECIAL_SMASH_SIZE;
use crate::custom::SPECIAL_SMASH_HEAD;
use crate::custom::SPECIAL_SMASH_STATUS;
use crate::custom::SPECIAL_SMASH_GRAVITY;
use crate::custom::SPECIAL_SMASH_BODY;
use crate::custom::HIGH_SPAWN_POS;
use crate::custom::LOW_SPAWN_POS;
use crate::custom::MAJIN_GRAB;
use crate::custom::is_majin;
use crate::custom::read_tag;
use crate::custom::get_meter_gain;
use crate::custom::get_meter_gain_attack;
use crate::custom::get_meter_gain_damage;
use crate::custom::estimate_frame;
use crate::custom::SUB_METER;
use crate::custom::DRAIN_FULL_METER;
use crate::custom::NERD_GOT_CLIPPED;
use crate::custom::SUCCESSFUL_CLIP;
use crate::custom::GET_CLIPPED_NERD;
use crate::custom::BALL_BOUNCED;
use crate::custom::ALREADY_BOUNCED;
use crate::custom::SPAWN_SIDE;
use crate::custom::READY_GO;
use crate::custom::FINAL_WAIT;
use crate::custom::FINAL_TRANSFORM;
use crate::custom::FINAL_HIT;
use crate::custom::LUIGI_FINAL_HIT;
use crate::custom::FIRST_BOUNCE;
use crate::custom::LAST_TO_HIT_BALL;
use crate::custom::METER_ENABLED;
use crate::custom::FS_METER;
use crate::custom::FS_METER_3;
use crate::custom::START_FS;
use crate::custom::REMAINING_METER;
use crate::custom::GIGA_GRAB_TARGET;
use crate::custom::MAJIN_DEMON_TARGET;
use crate::custom::FRAME_COUNTER;
pub static mut FIGHTER_MANAGER_ADDR: usize = 0;
pub static mut FIGHTER_CUTIN_MANAGER_ADDR: usize = 0;
pub static mut ITEM_MANAGER_ADDR: usize = 0;
static MAJIN_SE: [&'static str;55] = ["appeal01", "appeal02", "appeal03", "attack01", "attack02", "attack03", "attack04", "attack05", "attack06", "attack07", "attack08", "attack09", "cliffcatch", "damage01", "damage02",
"damage03", "damagefly01", "damagefly02", "damage_twinkle", "final01", "final01_02", "final02", "final03", "final04", "final04_02", "furafura", "furasleep", "heavyget", "jump01", "missfoot01", "missfoot02", 
"ottotto", "passive", "smash_s01", "special_h01", "special_h01_command", "special_l01", "special_l02", "special_l03", "special_l04", "special_n01", "special_n01_command", "special_n02_command", "special_n03",
"special_s01", "special_s01_command", "special_s02", "special_s02_command", "swimup", "throw_l01", "wakeup", "win01", "win02", "win03", "knockout"];
static MAJIN_SEQ: [&'static str;8] = ["attack_s", "attack_m", "attack_l", "attack_ll", "futtobi01", "futtobi02", "jump", "ottotto"];
static MAJIN_MESH: [&'static str;27] = ["dogiup1", "dogiup2", "dogiup3", "dogiup4", "dogiup5", "dogiup6", "eye", "eye2", "openblink", "halfblink", "blink", "facen_mouth", "heavyattack_mouth", "escape_mouth",
 "ouch_mouth", "capture_mouth", "down_mouth", "voicec_mouth", "voiceb_mouth", "patternb_mouth", "result_mouth", "facen_brow", "talk_brow", "heavyattack_brow", 
 "capture_brow", "ouch_brow", "heavyouch_brow"];
static STATUS_OFFSET: usize = 0x23670;
static mut STATUS_ADDR: usize = 0;

pub mod globals {
    pub const UNK1: 				 i32 = 0x0; //void
    pub const UNK2:                  i32 = 0x1; //i32
    pub const FIGHTER_KIND:          i32 = 0x2; //i32
    pub const OBJECT_ID:             i32 = 0x3; //i32
    pub const UNK3:                  i32 = 0x4; //ptr, very close in value to UNK6 and the last 5 digits of both values don't change on reboot, does NOT change by player number
    pub const UNK4:                  i32 = 0x5; //ptr
    pub const UNK5:                  i32 = 0x6; //void
    pub const INIT_STATUS_FUNC:      i32 = 0x7; //ptr
    pub const IN_HITLAG:             i32 = 0x8; //bool
    pub const STATUS_KIND_INTERRUPT: i32 = 0x9; //i32
    pub const PREV_STATUS_KIND:      i32 = 0xA; //i32
    pub const STATUS_KIND:           i32 = 0xB; //i32
    pub const UNK6:                  i32 = 0xC; //i32, varies by fighter_kind but idk what the pattern is. Prints 480 for Falcon, 479 for Giga Bowser, 502 for Ryu
    pub const UNK7:                  i32 = 0xD; //bool
    pub const MOTION_FRAME:          i32 = 0xE; //f32
    pub const MOTION_FRAME_NO_INTERP:i32 = 0xF; //f32
    pub const UNK8:                  i32 = 0x10; //ptr, pointer to where the status_kind's function is located?
    pub const UNK9:                  i32 = 0x11; //ptr, equal to UNK10, does NOT change by player number
    pub const UNK10:                 i32 = 0x12; //ptr
    pub const UNK11:                 i32 = 0x13; //this bitch changes types (i32/ptr)
    pub const PREV_SUB_STATUS:       i32 = 0x14; //i32
    pub const SUB_STATUS:            i32 = 0x15; //i32
    pub const SITUATION_KIND:        i32 = 0x16; //i32
    pub const PREV_SITUATION_KIND:   i32 = 0x17; //i32
    pub const UNK12:				 i32 = 0x18; //f32, status kind related, occasionally matches StatusModule::status_kind_next
    pub const UNK13:                 i32 = 0x19; //i32
    pub const STICK_X:               i32 = 0x1A; //f32
    pub const STICK_Y:               i32 = 0x1B; //f32
    pub const FLICK_X:               i32 = 0x1C; //i32
    pub const FLICK_Y:               i32 = 0x1D; //i32
    pub const FLICK_Y_DIR:           i32 = 0x1E; //f32
    pub const PAD_FLAG:              i32 = 0x1F; //u64
    pub const CMD_CAT1:              i32 = 0x20; //u64
    pub const CMD_CAT2:              i32 = 0x21; //u64
    pub const CMD_CAT3:              i32 = 0x22; //u64
    pub const CMD_CAT4:              i32 = 0x23; //u64
}

mod mario;
mod donkey;
mod link;
mod samus;
mod samusd;
mod yoshi;
mod kirby;
mod fox;
mod pikachu;
mod luigi;
mod ness;
mod captain;
mod purin;
mod peach;
mod daisy;
mod koopa;
mod popo_nana;
mod sheik;
mod zelda;
mod mariod;
mod pichu;
mod falco;
mod marth;
mod lucina;
mod younglink;
mod ganon;
mod mewtwo;
mod koopag;
mod roy;
mod chrom;
mod gamewatch;
mod metaknight;
mod pit;
mod pitb;
mod szerosuit;
mod wario;
mod snake;
mod ike;
mod ptrainer;
mod diddy;
mod lucas;
mod sonic;
mod dedede;
mod pikmin;
mod lucario;
mod robot;
mod toonlink;
mod wolf;
mod murabito;
mod rockman;
mod wiifit;
mod rosetta;
mod littlemac;
mod gekkouga;
mod miifighter;
mod miiswordsman;
mod miigunner;
mod palutena;
mod pacman;
mod reflet;
mod shulk;
mod koopajr;
mod duckhunt;
mod ryu;
mod ken;
mod cloud;
mod kamui;
mod bayonetta;
mod inkling;
mod ridley;
mod simon;
mod richter;
mod krool;
mod shizue;
mod gaogaen;
mod packun;
mod jack;
mod brave;
mod buddy;
mod dolly;
mod master;
mod tantan;
mod pickel;
mod edge;
mod eflame;
mod elight;
mod demon;
mod custom;

#[skyline::hook(offset=0x3f0028, inline)]
unsafe fn offset_dump(ctx: &InlineCtx) {
	let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
	println!("Function Offset: {:#X}", ctx.registers[8].x.as_ref() - text);
}

//level 1: 48F710

extern "C" {
	#[link_name = "\u{1}_ZN3app5stage12get_stage_idEv"]
	fn get_stage_id() -> i32;
}

extern "C" {
	#[link_name = "\u{1}_ZN3app2ai7cp_typeEP9lua_State"]
	fn ai__cp_type(lua_state: u64) -> i32;
}

extern "C" {
	#[link_name = "\u{1}_ZN3app8lua_bind28PostureModule__rot_y_lr_implEPNS_26BattleObjectModuleAccessorE"]
	fn imported_rot_y_lr(boma: &mut BattleObjectModuleAccessor) -> f32;
}

#[skyline::hook(replace=app::FighterUtil::is_valid_just_shield)]
unsafe fn is_valid_just_shield_replace(module_accessor: &mut BattleObjectModuleAccessor) -> bool {
	if SPECIAL_SMASH_STATUS == 1 {
		return false;
	}
	else {
		original!()(module_accessor)
	}
}

#[skyline::hook(replace=app::FighterUtil::is_valid_just_shield_reflector)]
unsafe fn is_valid_just_shield_reflector_replace(module_accessor: &mut BattleObjectModuleAccessor) -> bool {
	let fighter_kind = app::utility::get_kind(module_accessor);
	if fighter_kind == *FIGHTER_KIND_GANON || SPECIAL_SMASH_STATUS == 2 {
		return true;
	}
	else {
		original!()(module_accessor)
	}
}

extern "C" {
	#[link_name = "\u{1}_ZN3app17sv_camera_manager10dead_rangeEP9lua_State"]
	fn dead_range(lua_state: u64) -> Vector4f;
}

extern "C" {
	#[link_name = "\u{1}_ZN3app17sv_camera_manager12camera_rangeEv"]
	fn camera_range() -> Vector4f;
}

extern "C" {
	#[link_name = "\u{1}_ZN3app9curryshot15is_preview_modeEv"]
	fn is_preview_mode() -> bool;
}

extern "C" {
	#[link_name = "\u{1}_ZN3app22FighterSpecializer_Ryu25req_shinsyoryu_hit_effectERNS_26BattleObjectModuleAccessorERKN3phx8Vector3fES6_bb"]
	pub fn req_shinsyoryu_hit_effect(
		module_accessor: *mut BattleObjectModuleAccessor,
		arg2: *const Vector3f,
		arg3: *const Vector3f,
		arg4: bool,
		arg5: bool
	) -> u64;
}

#[skyline::hook(replace=smash::app::lua_bind::FighterInformation::gravity)]
unsafe fn gravity_replace(fighter_information: &mut app::FighterInformation) -> f32 {
	let ret = original!()(fighter_information);

	if ret == 1.33 {
		SPECIAL_SMASH_GRAVITY = 1;
	}
	else if ret == 0.66 {
		SPECIAL_SMASH_GRAVITY = 2;
	}
	else {
		SPECIAL_SMASH_GRAVITY = 0;
	}
	return 1.0;
}

#[skyline::hook(offset=0x67a790)]
pub unsafe fn notify_log_event_collision_hit_replace(
fighter_manager: u64,
attacker_object_id: u32,
defender_object_id: u32, 
move_type: u64,
arg5: u64,
move_type_again: u64) -> u64 {
	let attacker_boma = &mut *app::sv_battle_object::module_accessor(attacker_object_id);
	let defender_boma = &mut *app::sv_battle_object::module_accessor(defender_object_id);
	let attacker_kind = app::utility::get_kind(attacker_boma);
	let defender_kind = app::utility::get_kind(defender_boma);
	let attacker_motion_kind = MotionModule::motion_kind(attacker_boma);
	let attacker_status_kind = StatusModule::status_kind(attacker_boma);
	let _defender_status_kind = StatusModule::status_kind(defender_boma);
	let defender_situation_kind = StatusModule::situation_kind(defender_boma);
	let PREV_MOTION = &mut FIGHTER_U64_1[get_player_number(attacker_boma)];

	//Turbo

	if SPECIAL_SMASH_HEAD == 1 {
		CancelModule::enable_cancel(attacker_boma);
	}

	//Ball

	if attacker_kind == *ITEM_KIND_SOCCERBALL {
		LAST_TO_HIT_BALL = get_player_number(defender_boma); //If the ball hits someone and then goes out of bounds, the team that got hit loses the stock
	}
	if defender_kind == *ITEM_KIND_SOCCERBALL {
		LAST_TO_HIT_BALL = get_player_number(attacker_boma);
		ALREADY_BOUNCED = false;
	}

	//FS Meter

	if (app::utility::get_category(attacker_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER || attacker_kind == *WEAPON_KIND_DOLLY_WAVE) && METER_ENABLED && FS_METER[get_player_number(attacker_boma)] < 200.0 {
		if attacker_kind == *FIGHTER_KIND_DOLLY || attacker_kind == *FIGHTER_KIND_PZENIGAME || attacker_kind == *FIGHTER_KIND_EFLAME || attacker_kind == *FIGHTER_KIND_ELIGHT || attacker_kind == *WEAPON_KIND_DOLLY_WAVE {
			let dealt_damage = DamageModule::damage(defender_boma, 0) - LAST_DAMAGE[get_player_number(defender_boma)];
			if attacker_kind == *FIGHTER_KIND_PZENIGAME && FS_METER[get_player_number(attacker_boma)] < 68.0 {
				FS_METER[get_player_number(attacker_boma)] += dealt_damage / 1.1;
			}
			else if attacker_kind == *FIGHTER_KIND_DOLLY || attacker_kind == *WEAPON_KIND_DOLLY_WAVE {
				FS_METER[get_player_number(attacker_boma)] += dealt_damage / 0.85;
			}
			else {
				FS_METER[get_player_number(attacker_boma)] += dealt_damage / 1.1;
			}
		}
	}

	//Zoom in effects

	if app::utility::get_category(attacker_boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER && GET_CLIPPED_NERD[get_player_number(defender_boma)] {
		if attacker_kind == *FIGHTER_KIND_CAPTAIN && attacker_motion_kind == hash40("attack_air_f") && MotionModule::frame(attacker_boma) < 15.0 {
			NERD_GOT_CLIPPED[get_player_number(attacker_boma)] = Vector4f{x: 10.0, y: 1.2, z: -0.3, w: 0.8 * PostureModule::lr(attacker_boma)};
			SlowModule::set_whole(attacker_boma, 2, 0);
			SUCCESSFUL_CLIP[get_player_number(attacker_boma)] = 31;			
		}
		GET_CLIPPED_NERD[get_player_number(defender_boma)] = false;
	}
	if attacker_kind == *FIGHTER_KIND_MEWTWO && attacker_motion_kind == hash40("attack_air_lw") { //If Mewtwo hits an airborne opponent with Dair after using teleport, zoom in
		for i in 0..3 {
			if B_CHECK[get_player_number(attacker_boma)] {
				if defender_situation_kind == *SITUATION_KIND_AIR {
					NERD_GOT_CLIPPED[get_player_number(attacker_boma)] = Vector4f{x: 5.0, y: 1.2, z: -0.3, w: 0.8 * PostureModule::lr(attacker_boma)};
					SlowModule::set_whole(attacker_boma, 4, 0);
					SUCCESSFUL_CLIP[get_player_number(attacker_boma)] = 31;
					B_CHECK[get_player_number(attacker_boma)] = false;				
				}
				break;
			}
		}
	}
	if attacker_kind == *FIGHTER_KIND_GANON && attacker_motion_kind == hash40("attack_hi3") && StatusModule::prev_status_kind(attacker_boma, 0) == *FIGHTER_STATUS_KIND_DASH && StatusModule::prev_status_kind(attacker_boma, 1) == *FIGHTER_STATUS_KIND_ATTACK { //If Ganondorf hits an Up Tilt after Jab > Dash, zoom in
		NERD_GOT_CLIPPED[get_player_number(attacker_boma)] = Vector4f{x: 5.0, y: 1.2, z: -0.3, w: 0.8 * PostureModule::lr(attacker_boma)};
		SlowModule::set_whole(attacker_boma, 4, 0);
		SUCCESSFUL_CLIP[get_player_number(attacker_boma)] = 31;
	}
	if attacker_kind == *FIGHTER_KIND_ROCKMAN && attacker_motion_kind == hash40("attack_hi3") && (*PREV_MOTION == hash40("appeal_lw_r") || *PREV_MOTION == hash40("appeal_lw_l")) && estimate_frame(attacker_boma, 5.0) { //If Megaman hits Down Taunt > Up Tilt, zoom in
		NERD_GOT_CLIPPED[get_player_number(attacker_boma)] = Vector4f{x: 5.0, y: 1.2, z: -0.3, w: 0.8 * PostureModule::lr(attacker_boma)};
		SlowModule::set_whole(attacker_boma, 4, 0);
		SUCCESSFUL_CLIP[get_player_number(attacker_boma)] = 31;
	}
	if attacker_kind == *FIGHTER_KIND_KEN && (attacker_motion_kind == hash40("special_air_hi") || attacker_motion_kind == hash40("special_air_hi_command")) && WorkModule::get_int(attacker_boma, *FIGHTER_RYU_STATUS_WORK_ID_SPECIAL_COMMON_INT_STRENGTH) == *FIGHTER_RYU_STRENGTH_M { //If Ken gets a double hit air medium, zoom in
		if MotionModule::frame(attacker_boma) < 8.0 {
			SUCCESSFUL_CLIP[get_player_number(attacker_boma)] = 30;
		}
		else if SUCCESSFUL_CLIP[get_player_number(attacker_boma)] < 12 && SUCCESSFUL_CLIP[get_player_number(attacker_boma)] > 0 { 
			NERD_GOT_CLIPPED[get_player_number(defender_boma)] = Vector4f{x: 5.0, y: 1.2, z: -0.5, w: 0.8 * PostureModule::lr(attacker_boma)};
			SlowModule::set_whole(attacker_boma, 4, 0);
			SUCCESSFUL_CLIP[get_player_number(attacker_boma)] = 13;
		}
	}

	//Character specific

	if defender_kind == *FIGHTER_KIND_SAMUS {
		let DESTROY_MISSILES = &mut FIGHTER_BOOL_1[get_player_number(defender_boma)];
		*DESTROY_MISSILES = true;
	}
	if attacker_kind == *FIGHTER_KIND_LITTLEMAC && attacker_status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_LW_HIT {
		if defender_situation_kind == *SITUATION_KIND_GROUND {
			KineticModule::unable_energy_all(attacker_boma); //If Mac counters a grounded opponent, stop his momentum
		}
		else {
			WorkModule::enable_transition_term(attacker_boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S); //If Mac counters an aerial opponent, let him cancel into Side B
		}
	}
	if attacker_kind == *FIGHTER_KIND_LUIGI && attacker_status_kind == *FIGHTER_STATUS_KIND_FINAL {
		LUIGI_FINAL_HIT[get_player_number(defender_boma)] = true;
	}
	if attacker_kind == *FIGHTER_KIND_KOOPAG {
		if attacker_motion_kind == hash40("catch") {
			GIGA_GRAB_TARGET[get_player_number(attacker_boma)] = get_player_number(defender_boma);
		}
	}
	if is_majin(attacker_boma, attacker_kind) {
		let DEMON_GRAB_TARGET = &mut FIGHTER_INT_1[get_player_number(attacker_boma)];
		let RD_OVER_100 = &mut FIGHTER_BOOL_8[get_player_number(attacker_boma)];
		if attacker_motion_kind == hash40("special_lw") { //If Majin hits someone with raging demon and they were over 100% when the attack started, kill them in 35 frames
			if DamageModule::damage(defender_boma, 0) >= 140.0 {
				*RD_OVER_100 = true;
				if estimate_frame(attacker_boma, 179.0) {
					DEMON_DEATH_TIMER[get_player_number(defender_boma)] = 35;
				} 
			}
			else {
				*RD_OVER_100 = false;
			}
		}
		if attacker_motion_kind == hash40("special_air_lw_turn") {
			*DEMON_GRAB_TARGET = get_player_number(defender_boma) as i32;
			MAJIN_DEMON_TARGET[get_player_number(attacker_boma)] = get_player_number(defender_boma);
			let pos = Vector3f{x: PostureModule::pos_x(defender_boma) - 8.0 * PostureModule::lr(attacker_boma), y: PostureModule::pos_y(defender_boma) + 6.0, z: PostureModule::pos_z(attacker_boma)};
			PostureModule::set_pos(attacker_boma, &pos);
		}
		if attacker_motion_kind == hash40("special_air_lw_step_f") || attacker_motion_kind == hash40("special_lw_turn") {
			*DEMON_GRAB_TARGET = 8;
		}
		if attacker_motion_kind == hash40("special_lw_step_b") {
			MAJIN_DEMON_TARGET[get_player_number(attacker_boma)] = get_player_number(defender_boma);
		}
	}
	if is_majin(defender_boma, defender_kind) {
		let MAJIN_TELEPORT = &mut FIGHTER_FLOAT_2[get_player_number(defender_boma)];
		*MAJIN_TELEPORT = 0.0;
	}
	if attacker_kind == *FIGHTER_KIND_GAOGAEN && COUNTERHIT_CHECK[get_player_number(defender_boma)] { //Incineroar counterhit detection
		COUNTERHIT_SUCCESS[get_player_number(attacker_boma)] = true;
		COUNTERHIT_CHECK[get_player_number(defender_boma)] = false;
	}

	LAST_DAMAGE[get_player_number(defender_boma)] = DamageModule::damage(defender_boma, 0);
	GET_CLIPPED_NERD[get_player_number(defender_boma)] = false;
	
	original!()(fighter_manager, attacker_object_id, defender_object_id, move_type, arg5, move_type_again)
}

#[skyline::hook(offset=0x4e5380)]
pub unsafe fn get_param_int_lvl_1_replace(
boma: u64,
param_type: u64,
param_hash: u64) -> i32 {
	let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
	let fighter_kind = app::utility::get_kind(module_accessor);
	let status_kind = StatusModule::status_kind(module_accessor);

	if param_hash == hash40("charge_final_decrease_start_frame") || param_hash == hash40("charge_final_remove_frame") {
		if DRAIN_FULL_METER[get_player_number(module_accessor)] {
			if param_hash == hash40("charge_final_remove_frame") {
				DRAIN_FULL_METER[get_player_number(module_accessor)] = false;
			}
			return 0;
		}
		else {
			if get_meter_gain(module_accessor, fighter_kind) != 0.0 && get_meter_gain_damage(module_accessor, fighter_kind) == 0.0 {
				return 1200;
			}
			else {
				return 9999999;
			}
		}
	}
	else if fighter_kind == FIGHTER_KIND_SHULK {
		if param_type == hash40("param_special_n") && param_hash == hash40("circle_menu_release_after_interval_frame") {
			if (status_kind == *FIGHTER_STATUS_KIND_DAMAGE
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_AIR
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D
			|| status_kind == *FIGHTER_STATUS_KIND_DAMAGE_FALL 
			|| status_kind == *FIGHTER_STATUS_KIND_TREAD_DAMAGE) && WorkModule::is_enable_transition_term(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR) == false {
				let hitstun = (WorkModule::get_int(module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_FRAME) - WorkModule::get_int(module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_INT_HIT_STOP_FRAME)) as f32;
				if WorkModule::get_float(module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_REACTION_FRAME) + hitstun < 40.0 {
					return WorkModule::get_float(module_accessor, *FIGHTER_STATUS_DAMAGE_WORK_FLOAT_REACTION_FRAME) as i32;
				}
				else {
					return (40.0 - hitstun) as i32;
				}
			}
			else {
				original!()(boma, param_type, param_hash)
			}
		}
		else {
			original!()(boma, param_type, param_hash)
		}
	}
	else if fighter_kind == *FIGHTER_KIND_BRAVE {
		if param_hash == hash40("lot_prob") {
			println!("Param Type: {}", param_type);
			original!()(boma, param_type, param_hash)
		}
		else {
			original!()(boma, param_type, param_hash)
		}
	}
	else if fighter_kind == *WEAPON_KIND_RYU_SHINKUHADOKEN && is_majin(module_accessor, fighter_kind) {
		if param_hash == hash40("life") {
			return 38;
		}
		else {
			original!()(boma, param_type, param_hash)
		}
	}
	else {
		original!()(boma, param_type, param_hash)
	}
}

#[skyline::hook(replace=smash::app::lua_bind::WorkModule::get_param_int)]
pub unsafe fn get_param_int_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
param_type: u64,
param_hash: u64) -> i32 {
	let fighter_kind = app::utility::get_kind(module_accessor);

	if is_majin(module_accessor, fighter_kind) {
		let G_DEMON_FLIP = &mut FIGHTER_BOOL_3[get_player_number(module_accessor)];
		if param_type == hash40("param_special_hi") {
			if param_hash == hash40("landing_frame_w") {
				return 12;
			}
			else if param_hash == hash40("landing_frame_m") || param_hash == hash40("landing_frame_s") {
				return 30;
			}
			else {
				original!()(module_accessor, param_type, param_hash)
			}
		}
		else if *G_DEMON_FLIP && param_hash == 0 && param_type == hash40("jump_squat_frame") {
			return 6;
		}
		else {
			original!()(module_accessor, param_type, param_hash)
		}
	}
	else {
		original!()(module_accessor, param_type, param_hash)
	}
}

#[skyline::hook(replace=smash::app::lua_bind::WorkModule::get_int)]
pub unsafe fn get_int_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
int: i32) -> u64 {
	let fighter_kind = app::utility::get_kind(module_accessor);
	if SPECIAL_SMASH_BODY == 3 && fighter_kind == *ITEM_KIND_SOCCERBALL {
		let situation_kind = StatusModule::situation_kind(module_accessor);
		let motion_kind = MotionModule::motion_kind(module_accessor);
		let mut pos = Vector3f{x: PostureModule::pos_x(module_accessor), y: PostureModule::pos_y(module_accessor), z: PostureModule::pos_z(module_accessor)};
		if pos.x < camera_range().x + 10.0 || pos.x > camera_range().y - 10.0 || pos.y < camera_range().w + 10.0 { 
			//If we do know who it was, trigger the ball KO sequence
			if ALREADY_BOUNCED {
				BALL_BOUNCED = Vector3f{x: pos.x, y: 0.0, z: 0.0};
			}
			else {
				BALL_BOUNCED = Vector3f{x: pos.x, y: 1.0, z: 0.0};
			}
		}

		if GroundModule::get_touch_flag(module_accessor) == *GROUND_TOUCH_FLAG_DOWN as u64 {
			if ALREADY_BOUNCED || (FIRST_BOUNCE && ((SPAWN_SIDE[LAST_TO_HIT_BALL] && pos.x > 3.0) || (!SPAWN_SIDE[LAST_TO_HIT_BALL] && pos.x < -3.0))) { //If either we already bounced, or we hit the ball but it was still on our side, KO
				BALL_BOUNCED = Vector3f{x: pos.x, y: 0.0, z: 0.0};
				ALREADY_BOUNCED = false;
			}
			else { //Otherwise, just record that we already bounced
				ALREADY_BOUNCED = true;
			}	
			FIRST_BOUNCE = true;
		}
	}
	original!()(module_accessor, int)
}

#[skyline::hook(replace=smash::app::lua_bind::WorkModule::get_float)]
pub unsafe fn get_float_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
float: i32) -> f32 {
	let fighter_kind = app::utility::get_kind(module_accessor);
	if fighter_kind == *FIGHTER_KIND_LUCARIO {
		if float == *FIGHTER_LUCARIO_INSTANCE_WORK_ID_FLOAT_CURR_AURAPOWER {
			if FINAL_TRANSFORM[get_player_number(module_accessor)] > 1 {
				return 1.67;
			}
			else {
				original!()(module_accessor, float)				
			}
		}
		else {
			original!()(module_accessor, float)
		}
	}
	else {
		original!()(module_accessor, float)
	}
}

#[skyline::hook(offset=0x4e53c0)]
pub unsafe fn get_param_float_lvl_1_replace(
boma: u64,
param_type: u64,
param_hash: u64) -> f32 {
	let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
	let owner_module_accessor = &mut *app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	let fighter_kind = app::utility::get_kind(module_accessor);
	let status_kind = StatusModule::status_kind(module_accessor);
	let situation_kind = StatusModule::situation_kind(module_accessor);
	let mut SHELL_TIMER = &mut FIGHTER_INT_2[0];
	if app::utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
		SHELL_TIMER = &mut FIGHTER_INT_2[get_player_number(module_accessor)];
	}
	
	
	if param_hash == 0 {
		if INKLING_PRESENT {
			if param_type == hash40("dash_speed")
			|| param_type == hash40("air_speed_x_stable") {
				if INKED[get_player_number(module_accessor)] != 0 && param_type == hash40("dash_speed") {
					if fighter_kind == FIGHTER_KIND_INKLING && INKED[get_player_number(module_accessor)] == 2 {
						return 2.36;
					}
					else {
						return original!()(boma, param_type, param_hash) * 0.7;
					}
				}
				else {
					let total_ink = (InkPaintModule::ink(module_accessor, app::Fighter::get_id_from_entry_id(0)) 
					+ InkPaintModule::ink(module_accessor, app::Fighter::get_id_from_entry_id(1)) 
					+ InkPaintModule::ink(module_accessor, app::Fighter::get_id_from_entry_id(2))
					+ InkPaintModule::ink(module_accessor, app::Fighter::get_id_from_entry_id(3))
					+ InkPaintModule::ink(module_accessor, app::Fighter::get_id_from_entry_id(4)) 
					+ InkPaintModule::ink(module_accessor, app::Fighter::get_id_from_entry_id(5))
					+ InkPaintModule::ink(module_accessor, app::Fighter::get_id_from_entry_id(6))
					+ InkPaintModule::ink(module_accessor, app::Fighter::get_id_from_entry_id(7))) / ((9 - TOTAL_FIGHTER) as f32);
					if total_ink > 180.0 {
						return original!()(boma, param_type, param_hash) * 0.9;
					}
					else {
						return original!()(boma, param_type, param_hash) * (1.0 - (total_ink / 1800.0));
					}
				}
			}
		}

		if REWOUND[get_player_number(module_accessor)] != -1 {
			if param_type == hash40("air_accel_y") {
				return 0.0;
			}
			else {
				original!()(boma, param_type, param_hash)
			}
		}
		else if fighter_kind == FIGHTER_KIND_PZENIGAME {
			if param_type == hash40("weight") && *SHELL_TIMER != 0 {
				return original!()(boma, param_type, param_hash) / 2.0;
			}
			original!()(boma, param_type, param_hash)
		}
		else if fighter_kind == FIGHTER_KIND_MIIFIGHTER {
			let SMALL_STATE = &mut FIGHTER_INT_1[get_player_number(module_accessor)];
			if *SMALL_STATE != 0 {
				if *SMALL_STATE == 1 {
					if param_type == hash40("walk_speed_max") 
					|| param_type == hash40("dash_speed")
					|| param_type == hash40("run_speed_max")
					|| param_type == hash40("run_accel_add") {
						return original!()(boma, param_type, param_hash) * 1.2208;
					}
					else if param_type == hash40("ground_brake") {
						return original!()(boma, param_type, param_hash) * 0.9;
					}
					else if param_type == hash40("jump_speed_x") {
						return original!()(boma, param_type, param_hash) * 1.0812;
					}
					else if param_type == hash40("jump_y")
					|| param_type == hash40("mini_jump_y")
					|| param_type == hash40("jump_aerial_y") {
						return original!()(boma, param_type, param_hash) * 1.1856;
					}
					else if param_type == hash40("air_accel_x_add")
					|| param_type == hash40("air_speed_x_stable")
					|| param_type == hash40("air_brake_x") {
						return original!()(boma, param_type, param_hash) * 1.176;
					}
					else if param_type == hash40("weight") {
						return original!()(boma, param_type, param_hash) * 0.97;
					}
					else if param_type == hash40("shield_radius") {
						return original!()(boma, param_type, param_hash) * 0.9;
					}
					else if param_type == hash40("passive_wall_jump_x_speed")
					|| param_type == hash40("passive_wall_jump_y_speed")
					|| param_type == hash40("cliff_jump_x_speed") {
						return original!()(boma, param_type, param_hash) * 1.092;
					}
				}
				else {
					if param_type == hash40("walk_speed_max") 
					|| param_type == hash40("dash_speed")
					|| param_type == hash40("run_speed_max")
					|| param_type == hash40("run_accel_add") {
						return original!()(boma, param_type, param_hash) * 0.817;
					}
					else if param_type == hash40("ground_brake") {
						return original!()(boma, param_type, param_hash) * 1.2;
					}
					else if param_type == hash40("jump_speed_x") {
						return original!()(boma, param_type, param_hash) * 0.8536;
					}
					else if param_type == hash40("jump_y")
					|| param_type == hash40("mini_jump_y")
					|| param_type == hash40("jump_aerial_y")
					|| param_type == hash40("air_accel_x_add")
					|| param_type == hash40("air_speed_x_stable")
					|| param_type == hash40("air_brake_x") {
						return original!()(boma, param_type, param_hash) * 0.8455;
					}
					else if param_type == hash40("weight") {
						return original!()(boma, param_type, param_hash) * 1.02;
					}
					else if param_type == hash40("shield_radius") {
						return original!()(boma, param_type, param_hash) * 1.1;
					}
					else if param_type == hash40("passive_wall_jump_x_speed")
					|| param_type == hash40("cliff_jump_x_speed") {
						return original!()(boma, param_type, param_hash) * 0.8633;
					}
					else if param_type == hash40("passive_wall_jump_y_speed") {
						return original!()(boma, param_type, param_hash) * 0.817;
					}
				}
			}
			original!()(boma, param_type, param_hash)
		}
		else if is_majin(module_accessor, fighter_kind) { 
			if param_type == hash40("air_speed_y_stable") {
				return 1.76;
			}
			else {
				original!()(boma, param_type, param_hash)
			}
		}
		else {
			original!()(boma, param_type, param_hash)
		}
	}
	else {
		if param_hash == hash40("charge_final_add_gauge_by_time") {
			METER_ENABLED = true;
			if SUB_METER[get_player_number(module_accessor)] != 0.0 {
				let ret = SUB_METER[get_player_number(module_accessor)];
				SUB_METER[get_player_number(module_accessor)] = 0.0;
				return ret;
			}
			else {
				return get_meter_gain(module_accessor, fighter_kind);
			}
		}
		else if param_hash == hash40("charge_final_add_gauge_by_attack") {
			METER_ENABLED = true;
			return get_meter_gain_attack(module_accessor, fighter_kind);
		}
		else if param_hash == 0x2d073b8b11u64 { //Meter Gain through damage multiplier 
			METER_ENABLED = true;
			return get_meter_gain_damage(module_accessor, fighter_kind);
		}
		else if param_hash == 0x2212ae242au64 || param_hash == 0x222ea31b73u64 { //Remaining Meter when KO'd and no disadvantage
			let gain = get_meter_gain_damage(module_accessor, fighter_kind);
			if gain > 0.75 {
				FS_METER_3[get_player_number(module_accessor)] = 0.0;
				return 0.0;
			}
			else if gain < 0.0 || fighter_kind == *FIGHTER_KIND_DOLLY {
				return 100.0;
			}
			else {
				return REMAINING_METER[get_player_number(module_accessor)] as f32;
			}
		}
		else {
			if is_majin(module_accessor, fighter_kind) {
				if param_type == hash40("param_special_n") {
					if param_hash == hash40("hop_speed_y") {
						return 0.9;
					}
					else if param_hash == hash40("shoot_x") {
						if situation_kind == *SITUATION_KIND_AIR {
							return 10.0;
						}
						else {
							original!()(boma, param_type, param_hash)
						}
					}
					else if param_hash == hash40("shoot_y") { 
						if situation_kind == *SITUATION_KIND_AIR {
							return 3.5;
						}
						else {
							original!()(boma, param_type, param_hash)
						}
					}
					else {
						original!()(boma, param_type, param_hash)
					}
				}
				else if param_type == hash40("param_private") {
					if param_hash == hash40("final_search_radius") {
						if situation_kind == *SITUATION_KIND_GROUND {
							return 999999.0;
						}
						else {
							return -1.0;
						}
					}
					else if param_hash == hash40("final2_shoot_x") {
						return 10.0;
					}
					else if param_hash == hash40("final2_shoot_y") {
						return 3.5;
					}
					else {
						original!()(boma, param_type, param_hash)
					}
				}
				else {
					original!()(boma, param_type, param_hash)
				}
			}
			else if is_majin(owner_module_accessor, fighter_kind) {
				if param_type == hash40("param_hadoken") {
					if param_hash == hash40("speed_w") {
						return 0.9;
					}
					else if param_hash == hash40("speed_m") {
						return 1.2;
					}
					else if param_hash == hash40("speed_s") {
						return 1.5;
					}
					else if param_hash == hash40("speed_w_sp") {
						return 0.9;
					}
					else if param_hash == hash40("speed_m_sp") {
						return 1.2;
					}
					else if param_hash == hash40("speed_s_sp") {
						return 1.5;
					}
					else {
						original!()(boma, param_type, param_hash)
					}
				}
				else {
					original!()(boma, param_type, param_hash)
				}
			}
			else if fighter_kind == FIGHTER_KIND_LITTLEMAC {
				if param_type == hash40("param_special_n") && (param_hash == hash40("special_n_hit_damage_mul_") || param_hash == hash40("special_n_atk_damage_mul_")) {
					if param_hash == hash40("special_n_hit_damage_mul_") {
						let MAC_HITSTUN = &mut FIGHTER_INT_1[get_player_number(module_accessor)];
						if *MAC_HITSTUN == 0 {
							let taken_damage = DamageModule::damage(module_accessor, 0) - LAST_DAMAGE[get_player_number(module_accessor)];
							return -34.0 / taken_damage;
						}
					}
					else {
						for i in 0..TOTAL_FIGHTER {
							if COUNTERHIT_CHECK[get_player_number(&mut *get_boma(i))] && get_attacker_number(&mut *get_boma(i)) == get_player_number(module_accessor)
							&& (status_kind == *FIGHTER_STATUS_KIND_ATTACK
							|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3
							|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI3
							|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_LW3
							|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_DASH) {
								COUNTERHIT_CHECK[get_player_number(&mut *get_boma(i))] = false;
								let mut power = ((*AttackModule::attack_data(module_accessor, 0, false)).power) * AttackModule::get_attack_power_mul_pattern(module_accessor);
								if TOTAL_FIGHTER == 2 {
									power *= 1.2;
								}
								return 34.0 / power;
							}
						}
					}
					return 0.0;
				}
				else {
					original!()(boma, param_type, param_hash)
				}
			}
			else {
				original!()(boma, param_type, param_hash)
			}
		}
	}
}

#[skyline::hook(offset=0x3a3db0)]
pub unsafe fn get_param_helper_replace(
	arg1: u64,
	param_hash: u64,
	param_index: u32,
) -> u64 {
//	println!("Arg1: {:#X}", arg1);
//	println!("Param Hash: {:#X}", param_hash);	
//	println!("Param Index: {:#X}", param_index);	
	original!()(arg1, param_hash, param_index)
}

#[skyline::hook(offset=0x77d840)]
pub unsafe fn get_param_replace(
	param_accessor: u64,
	param_type: u64,
	param_hash: u64,
	param_index: u64
) -> u64 {
/*	if param_type == hash40("param_special_lw_command_data")
	|| param_hash == hash40("lot_prob") {
		println!("Command Data!");
		println!("Type: {:#X}", param_type);
		println!("Hash: {:#X}", param_hash);
		println!("Index: {}", param_index);
	}*/
	original!()(param_accessor, param_type, param_hash, param_index)
}

#[skyline::hook(replace=smash::app::lua_bind::WorkModule::get_param_float)]
pub unsafe fn get_param_float_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
param_type: u64,
param_hash: u64) -> f32 {
	let fighter_kind = app::utility::get_kind(module_accessor);
	let status_kind = StatusModule::status_kind(module_accessor);
	
	
	if param_hash == 0 {
		if INKLING_PRESENT {
			if param_type == hash40("run_speed_max")
			|| param_type == hash40("walk_speed_max") {
				let total_ink = (InkPaintModule::ink(module_accessor, app::Fighter::get_id_from_entry_id(0)) 
				+ InkPaintModule::ink(module_accessor, app::Fighter::get_id_from_entry_id(1)) 
				+ InkPaintModule::ink(module_accessor, app::Fighter::get_id_from_entry_id(2))
				+ InkPaintModule::ink(module_accessor, app::Fighter::get_id_from_entry_id(3))
				+ InkPaintModule::ink(module_accessor, app::Fighter::get_id_from_entry_id(4)) 
				+ InkPaintModule::ink(module_accessor, app::Fighter::get_id_from_entry_id(5))
				+ InkPaintModule::ink(module_accessor, app::Fighter::get_id_from_entry_id(6))
				+ InkPaintModule::ink(module_accessor, app::Fighter::get_id_from_entry_id(7))) / ((9 - TOTAL_FIGHTER) as f32);
				let mut ret = original!()(module_accessor, param_type, param_hash);
				if is_majin(module_accessor, fighter_kind) {
					if param_type == hash40("walk_speed_max") {
						ret = 0.75;
					}
					else if param_type == hash40("run_speed_max") {
						ret = 1.6;
					}	
				}
				if total_ink > 180.0 {
					return ret * 0.9;
				}
				else {
					return ret * (1.0 - (total_ink / 1800.0));
				}
			}
		}

		if status_kind > 21 && status_kind < 25 { //Status Kinds 22-24 are for Landing, Landing Light and Landing Attack Air respectively, so I can use this to reduce the number of checks
			if param_type == hash40("landing_attack_air_frame_n")
			|| param_type == hash40("landing_attack_air_frame_f")
			|| param_type == hash40("landing_attack_air_frame_b")
			|| param_type == hash40("landing_attack_air_frame_hi")
			|| param_type == hash40("landing_attack_air_frame_lw")
			|| param_type == hash40("landing_frame_light")
			|| param_type == hash40("landing_frame")
			|| param_type == hash40("landing_heavy_frame") {
				let mut ret = original!()(module_accessor, param_type, param_hash);
				if param_type != hash40("landing_frame_light")
				&& param_type != hash40("landing_frame")
				&& param_type != hash40("landing_heavy_frame") {
					if EXTRA_LAG[get_player_number(module_accessor)] > 1 {
						if !((fighter_kind == FIGHTER_KIND_GAMEWATCH && (param_type == hash40("lading_attack_air_frame_f") || param_type == hash40("landing_attack_air_frame_hi")))
						|| (fighter_kind == FIGHTER_KIND_ROCKMAN && (param_type == hash40("landing_attack_air_frame_n") || param_type == hash40("landing_attack_air_frame_lw")))
						|| (fighter_kind == FIGHTER_KIND_MIIGUNNER && param_type == hash40("landing_attack_air_frame_f"))
						|| (fighter_kind == FIGHTER_KIND_MURABITO && (param_type == hash40("landing_attack_air_frame_f") || param_type == hash40("landing_attack_air_frame_b")))
						|| (fighter_kind == FIGHTER_KIND_SHIZUE && (param_type == hash40("landing_attack_air_frame_f") || param_type == hash40("landing_attack_air_frame_b")))
						|| (fighter_kind == FIGHTER_KIND_PIKMIN && param_type != hash40("landing_attack_air_frame_n"))) {
							ret += 5.0;
						}
					}
					else if SPECIAL_SMASH_HEAD == 1 && EXTRA_LAG[get_player_number(module_accessor)] == 0 {
						ret = 1.0;
					}
					if SPECIAL_SMASH_SIZE == 1 {
						ret /= 1.7;
					}
					if SPECIAL_SMASH_SIZE == 2 {
						ret *= 2.0;
					}
				}
				if PARRIED_COUNT[get_player_number(module_accessor)] > 1 {
					ret += ((PARRIED_COUNT[get_player_number(module_accessor)] -1) * 3) as f32;
					PARRIED_COUNT[get_player_number(module_accessor)] = 0;
				}
				return ret;
			}
			else {
				original!()(module_accessor, param_type, param_hash)
			}
		}
		else if is_majin(module_accessor, fighter_kind) { 
			if param_type == hash40("walk_speed_max") {
				return 0.75;
			}
			else if param_type == hash40("run_speed_max") {
				return 1.6;
			}
			else if param_type == hash40("dive_speed_y") {
				return 2.464;
			}
			else {
				original!()(module_accessor, param_type, param_hash)
			}
		}
		else {
			original!()(module_accessor, param_type, param_hash)
		}
	}
	else {
		if fighter_kind != *FIGHTER_KIND_INKLING && !is_majin(module_accessor, fighter_kind) {
			original!()(module_accessor, param_type, param_hash)
		}
		else {
			if is_majin(module_accessor, fighter_kind) {
				if param_type == hash40("param_special_lw") {
					if param_hash == hash40("start_speed_x_mul") || param_hash == hash40("start_speed_y_mul") || param_hash == hash40("defense_mul") {
						return 1.0;
					}
					else if param_hash == hash40("endurance_min") || param_hash == hash40("endurance_max") || param_hash == hash40("air_step_add_speed_x") || param_hash == hash40("air_step_add_speed_y") {
						return 0.0;
					}
					else if param_hash == hash40("attack_invalid_frame") || param_hash == hash40("step_cancel_invalid_frame") {
						return 999.0;
					}
					else if param_hash == hash40("accel_y") {
						return 0.12;
					}
					else if param_hash == hash40("max_speed_y") {
						return 2.24;
					}
					else {
						original!()(module_accessor, param_type, param_hash)
					}
				}
				else if param_type == hash40("param_special_n") {
					if param_hash == hash40("air_speed_x_mul") {
						return 1.0;
					}
					else if param_hash == hash40("control_limit_mul_x") {
						return 1.0;
					} 
					else if param_hash == hash40("attack_acl_y") {
						original!()(module_accessor, param_type, param_hash)
					}
					else if param_hash == hash40("attack_max_y") {
						original!()(module_accessor, param_type, param_hash)
					}
					else {
						original!()(module_accessor, param_type, param_hash)
					}
				}
				else if param_type == hash40("param_special_s") {
					if param_hash == hash40("speed_y_mul") {
						return 1.0;
					}
					else if param_hash == hash40("air_add_speed_y") {
						return 0.0;
					}
					else if param_hash == hash40("end_air_accel_y") {
						return 0.12;
					}
					else if param_hash == hash40("air_accel_y") {
						return 0.12;
					}
					else if param_hash == hash40("air_max_speed_y") {
						return 1.76;
					}
					else if param_hash == hash40("end_air_max_speed_y") {
						return 1.76;
					}
					else {
						original!()(module_accessor, param_type, param_hash)
					}
				}
				else {
					original!()(module_accessor, param_type, param_hash)
				}
			}
			else if fighter_kind == FIGHTER_KIND_INKLING {
				if param_type == hash40("param_private") && param_hash == hash40("charge_ink") && INKED[get_player_number(module_accessor)] == 2 {
					return original!()(module_accessor, param_type, param_hash) * 2.0;
				}
				else {
					original!()(module_accessor, param_type, param_hash)
				}
			}
			else {
				original!()(module_accessor, param_type, param_hash)
			}
		}
	}
}

#[skyline::hook(offset=0x35d2f50)]
pub unsafe fn set_mesh_visibility_lvl_2_replace(
model_data: u64,
mesh: u64,
visibility: u8) -> u64 {
	let mut new_mesh = mesh;
	if sv_information::is_ready_go() {
		let mut player_number = 8;
		for i in 0..8 {
			if MODEL_DATA_POS[i] == model_data {
				player_number = i;
				break;
			}
		}
		if FIGHTER_NAME[player_number] == hash40("AKUMA") {
			if READY_GO[player_number] || (!READY_GO[player_number] && visibility == 1) {
				if mesh == hash40("gamemodel") {
					new_mesh = hash40("majin_gamemodel");
				}
				else {
					for i in 0..27 {
						if mesh == hash40(&("ryu_".to_owned() + MAJIN_MESH[i])) {
							new_mesh = hash40(&("majin_".to_owned() + MAJIN_MESH[i]));
							break;	
						}
					}
				}
			}
		}
	}
	original!()(model_data, new_mesh, visibility)
}

#[skyline::hook(replace = smash::app::sv_module_access::effect)]
pub unsafe fn sv_module_access_effect_replace(lua_state: u64) -> u64 {
	let mut l2c_agent = L2CAgent::new(lua_state);
	let module_accessor = app::sv_system::battle_object_module_accessor(lua_state);
	let fighter_kind = app::utility::get_kind(module_accessor);
	let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};

	if fighter_kind == *WEAPON_KIND_RYU_HADOKEN {
		let owner_module_accessor = &mut *app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		let HADOKEN_HIT_EFF = &mut FIGHTER_VEC3F_1[get_player_number(owner_module_accessor)];
		if is_majin(owner_module_accessor, fighter_kind) {
			let effect = l2c_agent.pop_lua_stack(2); //effect
			if effect.get_int() == hash40("ryu_hadoken_hit") || effect.get_int() == hash40("ryu_hadoken_hit2") {
				*HADOKEN_HIT_EFF = Vector3f{x: PostureModule::pos_x(module_accessor) - PostureModule::pos_x(owner_module_accessor), y: PostureModule::pos_y(module_accessor) - PostureModule::pos_y(owner_module_accessor), z: PostureModule::pos_z(module_accessor) - PostureModule::pos_z(owner_module_accessor)};
				return 0;
			}	
		}
	}

	original!()(lua_state)
}

#[skyline::hook(offset=0x4dc270)]
pub unsafe fn play_se_level_1_replace(
boma: u64,
se: u64,
arg3: u8,
arg4: u32,
arg5: i32,
arg6: u8) -> u64 {
	let mut new_se = se;
	let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
	let fighter_kind = app::utility::get_kind(module_accessor);
	if is_majin(module_accessor, fighter_kind) {
		for i in 0..55 {
			if se == hash40(&("vc_ryu_".to_owned() + MAJIN_SE[i])) {
				new_se = hash40(&("vc_majin_".to_owned() + MAJIN_SE[i]));
				break;
			}
		}
	}

	original!()(boma, new_se, arg3, arg4, arg5, arg6)
}

#[skyline::hook(replace = smash::app::sv_animcmd::PLAY_SEQUENCE)]
pub unsafe fn play_sequence_replace(lua_state: u64) -> u64 {
	let mut l2c_agent = L2CAgent::new(lua_state);
	let module_accessor = app::sv_system::battle_object_module_accessor(lua_state);
	let fighter_kind = app::utility::get_kind(module_accessor);
	if is_majin(module_accessor, fighter_kind) {
		let seq = l2c_agent.pop_lua_stack(1); //sound effect
		l2c_agent.clear_lua_stack();
		let mut new_seq = seq.get_int();
		for i in 0..8 {
			if seq.get_int() == hash40(&("seq_ryu_rnd_".to_owned() + MAJIN_SEQ[i])) {
				new_seq = hash40(&("seq_majin_rnd_".to_owned() + MAJIN_SEQ[i]));
				break;
			}
		}
		l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_seq));
	}
	original!()(lua_state)
}

#[skyline::hook(replace = smash::app::sv_animcmd::PLAY_FLY_VOICE)]
pub unsafe fn play_fly_voice_replace(lua_state: u64) -> u64 {
	let mut l2c_agent = L2CAgent::new(lua_state);
	let module_accessor = app::sv_system::battle_object_module_accessor(lua_state);
	let fighter_kind = app::utility::get_kind(module_accessor);
	if is_majin(module_accessor, fighter_kind) {
		let se = l2c_agent.pop_lua_stack(1); //sound effect
		l2c_agent.clear_lua_stack();
		let mut new_se = se.get_int();
		for i in 0..55 {
			if se.get_int() == hash40(&("vc_ryu_".to_owned() + MAJIN_SE[i])) {
				new_se = hash40(&("vc_majin_".to_owned() + MAJIN_SE[i]));
				break;
			}
		}
		l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_se));
	}

	original!()(lua_state)
}

#[skyline::hook(replace = smash::app::sv_animcmd::PLAY_STATUS)]
pub unsafe fn play_status_replace(lua_state: u64) -> u64 {
	let mut l2c_agent = L2CAgent::new(lua_state);
	let module_accessor = app::sv_system::battle_object_module_accessor(lua_state);
	let fighter_kind = app::utility::get_kind(module_accessor);
	if is_majin(module_accessor, fighter_kind) {
		let se = l2c_agent.pop_lua_stack(1); //sound effect
		l2c_agent.clear_lua_stack();
		let mut new_se = se.get_int();
		for i in 0..55 {
			if se.get_int() == hash40(&("vc_ryu_".to_owned() + MAJIN_SE[i])) {
				new_se = hash40(&("vc_majin_".to_owned() + MAJIN_SE[i]));
				break;
			}
		}
		l2c_agent.push_lua_stack(&mut L2CValue::new_int(new_se));
	}

	original!()(lua_state)
}

#[skyline::hook(replace = smash::app::lua_bind::ControlModule::get_attack_air_kind)]
pub unsafe fn get_attack_air_kind_replace(
module_accessor: &mut app::BattleObjectModuleAccessor) -> i32 {
	if AERIAL_KIND[get_player_number(module_accessor)] != 0 {
		return AERIAL_KIND[get_player_number(module_accessor)];
	}
	else {
		original!()(module_accessor)
	}
}

#[skyline::hook(replace = smash::app::lua_bind::ArticleModule::generate_article)]
pub unsafe fn generate_article_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
article: i32,
arg3: bool,
arg4: i32
) -> u64 {
	let fighter_kind = app::utility::get_kind(module_accessor);
	let motion_kind = MotionModule::motion_kind(module_accessor);

	if fighter_kind == FIGHTER_KIND_DAISY 
	&& article == *FIGHTER_DAISY_GENERATE_ARTICLE_KINOPIO
	&& motion_kind != hash40("special_n") 
	&& motion_kind != hash40("special_air_n") 
	&& motion_kind != hash40("special_n_hit") 
	&& motion_kind != hash40("special_air_n_hit") 
	&& motion_kind != hash40("special_n_turn") 
	&& motion_kind != hash40("special_air_n_turn") {
		return 0;
	}
	else if fighter_kind == FIGHTER_KIND_INKLING && article == *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID && INKED[get_player_number(module_accessor)] != 2 && (motion_kind == hash40("dash") || motion_kind == hash40("turn_dash")) {
		return 0;
	}
	else if fighter_kind == *FIGHTER_KIND_KOOPAG && article == *FIGHTER_KOOPA_GENERATE_ARTICLE_BREATH && !(motion_kind == hash40("special_n_start") || motion_kind == hash40("special_air_n_start")) {
		return 0;
	}
	else {
		original!()(module_accessor, article, arg3, arg4)
	}
}

#[skyline::hook(replace = smash::app::lua_bind::KineticModule::change_kinetic)]
pub unsafe fn change_kinetic_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
kinetic_type: i32) -> i32 {
	let motion_kind = MotionModule::motion_kind(module_accessor);
	let fighter_kind = app::utility::get_kind(module_accessor);
	if fighter_kind == FIGHTER_KIND_SONIC && kinetic_type == *FIGHTER_KINETIC_TYPE_SONIC_SPECIAL_S_AIR_DASH && (motion_kind == hash40("special_s_dash_lw") || motion_kind == hash40("special_s_dash_hi") || motion_kind == hash40("special_s_dash_hop")) {
		original!()(module_accessor, *FIGHTER_KINETIC_TYPE_SONIC_SPECIAL_S_DASH)
	}
	else {
		original!()(module_accessor, kinetic_type)
	}
}

#[skyline::hook(replace = smash::app::lua_bind::GrabModule::set_rebound)]
pub unsafe fn set_rebound_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
rebound: bool) -> u64 {
	let status_kind = StatusModule::status_kind(module_accessor);

	if status_kind != *FIGHTER_STATUS_KIND_CATCH 
	&& status_kind != *FIGHTER_STATUS_KIND_CATCH_DASH
	&& status_kind != *FIGHTER_STATUS_KIND_CATCH_TURN {
		original!()(module_accessor, false)
	}
	else {
		original!()(module_accessor, rebound)
	}
}

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::on_flag)]
pub unsafe fn on_flag_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
flag: i32) -> u64 {
	let motion_kind = MotionModule::motion_kind(module_accessor);
	let status_kind = StatusModule::status_kind(module_accessor);
	let fighter_kind = app::utility::get_kind(module_accessor);

	if flag == *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI {
		if (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_JUMP) || (ControlModule::is_enable_flick_jump(module_accessor) && ControlModule::get_stick_y(module_accessor) >= 0.7)) && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI) {
			return 0;
		}
		else {
			original!()(module_accessor, flag)
		}
	}
	else if fighter_kind == FIGHTER_KIND_SONIC {
		if flag == *FIGHTER_SONIC_STATUS_SPECIAL_S_HOLD_FLAG_MAX_CHARGE && (status_kind == FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH || status_kind == FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD) {
			return 0;
		}
		else {
			original!()(module_accessor, flag)
		}
	}
	else if fighter_kind == *FIGHTER_KIND_RYU || fighter_kind == *FIGHTER_KIND_KEN {
		let FINAL_SMASH_CANCEL = &mut FIGHTER_BOOL_9[get_player_number(module_accessor)];
		if flag == *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL && !*FINAL_SMASH_CANCEL {
			*FINAL_SMASH_CANCEL = true;
			return 0;
		}
		else {
			original!()(module_accessor, flag)
		}
	}
	else if fighter_kind == *FIGHTER_KIND_BAYONETTA {
		if flag == *FIGHTER_BAYONETTA_INSTANCE_WORK_ID_FLAG_SHOOTING_FORBID {
			return 0;
		}
		else {
			original!()(module_accessor, flag)
		}
	}
	else if fighter_kind == *FIGHTER_KIND_INKLING {
		if flag == *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID && (motion_kind == hash40("dash") || motion_kind == hash40("turn_dash")) && INKED[get_player_number(module_accessor)] != 2 {
			return 0;
		}
		else {
			original!()(module_accessor, flag)
		}
	}
	else {
		original!()(module_accessor, flag)
	}
}

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::off_flag)]
pub unsafe fn off_flag_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
flag: i32) -> u64 {
	let fighter_kind = app::utility::get_kind(module_accessor);

	if fighter_kind == *FIGHTER_KIND_RYU || fighter_kind == *FIGHTER_KIND_KEN {
		if flag == *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL {
			let FINAL_SMASH_CANCEL = &mut FIGHTER_BOOL_9[get_player_number(module_accessor)];
			*FINAL_SMASH_CANCEL = false;
			return 0;
		}
		else {
			original!()(module_accessor, flag)
		}
	}
	else {
		original!()(module_accessor, flag)
	}
}

#[skyline::hook(replace = smash::app::lua_bind::WorkModule::is_enable_transition_term)]
pub unsafe fn is_enable_transition_term_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
term: i32
) -> bool {
	let fighter_kind = app::utility::get_kind(module_accessor);
	let status_kind = StatusModule::status_kind(module_accessor);
	let situation_kind = StatusModule::situation_kind(module_accessor);
	let motion_kind = MotionModule::motion_kind(module_accessor);
	let ret = original!()(module_accessor, term);

	if app::utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
		if READY_GO_TIMER != 0 {
			return false;
		}

		if REWOUND[get_player_number(module_accessor)] != -1 || (REWIND_ENDING[get_player_number(module_accessor)] 
		&& term != *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR 
		&& term != *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL
		&& term != *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FALL	
		&& term != *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_D	
		&& term != *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_L	
		&& term != *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_R	
		&& term != *FIGHTER_STATUS_TRANSITION_TERM_ID_DAMAGE_FLY_REFLECT_U
		&& term != *FIGHTER_STATUS_TRANSITION_TERM_ID_DOWN){
			return false;
		}

		if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_FINAL {
			if fighter_kind == *FIGHTER_KIND_BAYONETTA {
				if FINAL_HIT[get_player_number(module_accessor)] {
					return true;
				}
				else {
					return false;
				}				
			}
			else {
				if START_FS[get_player_number(module_accessor)] {
					START_FS[get_player_number(module_accessor)] = false;
					return true;
				}
				else {
					return ret && (ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) 
					|| ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R))
					&& ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL);
				}	
			}
		}
		else if START_FS[get_player_number(module_accessor)] {
			return false;
		}

		if SPECIAL_SMASH_BODY == 3 {
			if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW 
			|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_DASH
			|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_GUARD
			|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE
			|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ITEM_THROW_FORCE_DASH {
				return ret && situation_kind == *SITUATION_KIND_AIR;
			}
		}

		if GIGA_GRABBED[get_attacker_number(module_accessor)] > 0 && motion_kind == hash40("capture_wait_hi") {
			return false;
		}

		if MAJIN_GRAB[get_attacker_number(module_accessor)] > 0 && motion_kind == hash40("capture_wait_hi") {
			return false;
		}
		

		if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH_TURN {
			if motion_kind == hash40("catch_dash") && MotionModule::frame(module_accessor) > 4.0 {
				return false;
			}
		}

		if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START {
			if status_kind == *FIGHTER_STATUS_KIND_JUMP_SQUAT && ControlModule::is_enable_flick_jump(module_accessor) == false && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
				return false;
			}
			else {
				return ret;
			}
		}

		if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N {
			if fighter_kind == FIGHTER_KIND_PICKEL {
				let STANDING_ON_TNT = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
				if *STANDING_ON_TNT {
					return false;
				}
				else {
					return ret;
				}
			}
		}

		if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI {
			if fighter_kind == FIGHTER_KIND_MEWTWO || fighter_kind == FIGHTER_KIND_PITB {
				if B_CHECK[get_player_number(module_accessor)] {
					return false;
				}
				else {
					return ret;
				}
			}
		}

		if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S {
			if fighter_kind == FIGHTER_KIND_SONIC {
				let BOOST_METER = &mut FIGHTER_INT_1[get_player_number(module_accessor)];
				if *BOOST_METER >= 10 {
					if ret {
						*BOOST_METER -= 10;
						return true;
					}
					else {
						return false;
					}
				}
				else {
					return false;
				}
			}
		}

		if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW {
			if fighter_kind == FIGHTER_KIND_SHEIK || fighter_kind == FIGHTER_KIND_SZEROSUIT || fighter_kind == FIGHTER_KIND_MIIFIGHTER || is_majin(module_accessor, fighter_kind) {
				if B_CHECK[get_player_number(module_accessor)] {
					return false;
				}
				else {
					if original!()(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI) && is_majin(module_accessor, fighter_kind) == false {
						return true;
					}
				}
			}
			if fighter_kind == FIGHTER_KIND_PZENIGAME {
				let SHELL_TIMER = &mut FIGHTER_INT_2[get_player_number(module_accessor)];
				return ret && *SHELL_TIMER == 0;
			} 
			if fighter_kind == *FIGHTER_KIND_MIIFIGHTER {
				let SMALL_STATE = &mut FIGHTER_INT_1[get_player_number(module_accessor)];
				return ret && *SMALL_STATE == 0;
			}
		}
		

		if fighter_kind == *FIGHTER_KIND_RYU || fighter_kind == *FIGHTER_KIND_KEN {
			if !is_majin(module_accessor, fighter_kind) {
				if motion_kind == hash40("attack_11_near_s") && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_ALL) {
					if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK	
					|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_100 {
						return CancelModule::is_enable_cancel(module_accessor);
					}
				}
			}
			else {
				let DEMON_FLIP_INPUT = &mut FIGHTER_BOOL_5[get_player_number(module_accessor)];
				let G_DEMON_FLIP = &mut FIGHTER_BOOL_3[get_player_number(module_accessor)];
				if motion_kind == hash40("attack_lw4")
				|| status_kind == *FIGHTER_STATUS_KIND_ATTACK_AIR {
					if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N
					|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND
					|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND
					|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S
					|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND
					|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI
					|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND
					|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
					|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND {
						return CancelModule::is_enable_cancel(module_accessor);
					}
				}
				if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N2_COMMAND && ret {
					*DEMON_FLIP_INPUT = true;
				}
				if *G_DEMON_FLIP {
					if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_AIR
					|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR
					|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL
					|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON
					|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N
					|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N_COMMAND
					|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S
					|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S_COMMAND
					|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI
					|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI_COMMAND
					|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW
					|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_LW_COMMAND {
						return false;
					}
					else {
						return ret;
					}
				}
			}	
		}		

		if fighter_kind == FIGHTER_KIND_PLIZARDON {
			let ROCK_SMASH = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
			if *ROCK_SMASH && MotionModule::frame(module_accessor) < 65.0 {
				return false;
			}
		}		

		if fighter_kind == FIGHTER_KIND_PZENIGAME || fighter_kind == FIGHTER_KIND_PFUSHIGISOU || fighter_kind == FIGHTER_KIND_PLIZARDON {
			let SWITCH_TIMER = &mut FIGHTER_INT_1[get_player_number(module_accessor)];
			let SHELL_TIMER = &mut FIGHTER_INT_2[get_player_number(module_accessor)];
			let PT_RIGHT = &mut FIGHTER_BOOL_2[get_player_number(module_accessor)];
			let PT_LEFT = &mut FIGHTER_BOOL_3[get_player_number(module_accessor)];
			let PT_SWITCH = &mut FIGHTER_BOOL_4[get_player_number(module_accessor)];
			if term == FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_APPEAL_S && *SWITCH_TIMER == 0 {
				if !(fighter_kind == FIGHTER_KIND_PZENIGAME && *SHELL_TIMER != 0) {
					if original!()(module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_HI) {
						*PT_SWITCH = true;
						if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
							*PT_LEFT = true;
						}
						else {
							*PT_RIGHT = true;
						}
					}
				}
				return false;
			}
		}

		if fighter_kind == FIGHTER_KIND_KOOPAG && 
		(motion_kind == hash40("catch") 
		|| motion_kind == hash40("catch_wait")
		|| motion_kind == hash40("catch_pull") 
		|| motion_kind == hash40("catch_cut") 
		|| motion_kind == hash40("catch_attack")
		|| motion_kind == hash40("throw_f")
		|| motion_kind == hash40("throw_b")
		|| motion_kind == hash40("throw_hi")
		|| motion_kind == hash40("throw_lw")
		|| motion_kind == hash40("throw_lw_2")) {
			return false;
		}

		if fighter_kind == *FIGHTER_KIND_TANTAN {
			let CAN_DOUBLE_ATTACK = &mut FIGHTER_BOOL_2[get_player_number(module_accessor)];
			if term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL	
			|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON
			|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING	
			|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_ATTACK_AIR	
			|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_FALL_SPECIAL	
			|| term == *FIGHTER_STATUS_TRANSITION_TERM_ID_LANDING_LIGHT {
				return ret;
			}
			else {
				return ret && *CAN_DOUBLE_ATTACK;
			}
		}

		return ret;
	}
		
	return ret;

}

#[skyline::hook(replace = smash::app::lua_bind::StatusModule::change_status_request_from_script)]
pub unsafe fn change_status_request_from_script_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
status_kind: i32,
arg3: bool) -> u64 {
	let owner_module_accessor = app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
	let fighter_kind = app::utility::get_kind(module_accessor);
	let situation_kind = StatusModule::situation_kind(module_accessor);
	let motion_kind = MotionModule::motion_kind(module_accessor);

	if app::utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER && REWOUND[get_player_number(module_accessor)] != -1 {
		if status_kind != *FIGHTER_STATUS_KIND_CAPTURE_BEETLE	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_BEITCRANE	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_BLACKHOLE	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_BOSSGALAGA	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_CLAPTRAP	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_DAMAGE_YOSHI	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_DRIVER	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_ITEM	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_JACK_WIRE
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_JUMP	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_KAWASAKI	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_MASTERHAND
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_MASTER_SWORD	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_MIMIKKYU	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_NABBIT	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_PULLED	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_PULLED_FISHINGROD	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_PULLED_OCTOPUS	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_PULLED_PICKEL	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_PULLED_YOSHI	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_WAIT	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_WAIT_OCTOPUS	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_WAIT_YOSHI	
		&& status_kind != *FIGHTER_STATUS_KIND_CAPTURE_YOSHI
		&& status_kind != *FIGHTER_STATUS_KIND_CATCHED_AIR_GANON	
		&& status_kind != *FIGHTER_STATUS_KIND_CATCHED_AIR_END_GANON	
		&& status_kind != *FIGHTER_STATUS_KIND_CATCHED_AIR_FALL_GANON	
		&& status_kind != *FIGHTER_STATUS_KIND_CATCHED_CUT_GANON	
		&& status_kind != *FIGHTER_STATUS_KIND_CATCHED_GANON	
		&& status_kind != *FIGHTER_STATUS_KIND_CATCHED_PICKEL_TROLLEY	
		&& status_kind != *FIGHTER_STATUS_KIND_CATCHED_REFLET	
		&& status_kind != *FIGHTER_STATUS_KIND_CATCHED_RIDLEY
		&& status_kind != *FIGHTER_STATUS_KIND_CLUNG_CAPTAIN
		&& status_kind != *FIGHTER_STATUS_KIND_CLUNG_GANON
		&& status_kind != *FIGHTER_STATUS_KIND_CLUNG_DAMAGE_DIDDY
		&& status_kind != *FIGHTER_STATUS_KIND_CLUNG_DIDDY
		&& status_kind != *FIGHTER_STATUS_KIND_CLUNG_THROWN_DIDDY
		&& status_kind != *FIGHTER_STATUS_KIND_CLUNG_THROWN_BLANK_DIDDY
		&& status_kind != *FIGHTER_STATUS_KIND_THROWN
		&& status_kind != *FIGHTER_STATUS_KIND_DAMAGE	
		&& status_kind != *FIGHTER_STATUS_KIND_DAMAGE_AIR	
		&& status_kind != *FIGHTER_STATUS_KIND_DAMAGE_FALL	
		&& status_kind != *FIGHTER_STATUS_KIND_DAMAGE_FLY	
		&& status_kind != *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR	
		&& status_kind != *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D	
		&& status_kind != *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD	
		&& status_kind != *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR	
		&& status_kind != *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U	
		&& status_kind != *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL	
		&& status_kind != *FIGHTER_STATUS_KIND_DAMAGE_SLEEP	
		&& status_kind != *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_END	
		&& status_kind != *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_FALL	
		&& status_kind != *FIGHTER_STATUS_KIND_DAMAGE_SLEEP_START	
		&& status_kind != *FIGHTER_STATUS_KIND_DAMAGE_SONG	
		&& status_kind != *FIGHTER_STATUS_KIND_DAMAGE_SONG_END	
		&& status_kind != *FIGHTER_STATUS_KIND_DAMAGE_SONG_FALL	
		&& status_kind != *FIGHTER_STATUS_KIND_DAMAGE_SONG_START
		&& status_kind != *FIGHTER_STATUS_KIND_DEAD {
			return 0;
		}
		else {
			original!()(module_accessor, status_kind, arg3)
		}
	}
	else if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_F || status_kind == *FIGHTER_STATUS_KIND_ESCAPE_B {
		let MAJIN_TELEPORT = &mut FIGHTER_FLOAT_2[get_player_number(module_accessor)];
		if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_RUN_BRAKE && ControlModule::get_flick_x(module_accessor) > 5 {
			return 0;
		}
		else if is_majin(module_accessor, fighter_kind) && *MAJIN_TELEPORT != 0.0 {
			return 0;
		}
		else {
			original!()(module_accessor, status_kind, arg3)
		}
	}
	else if fighter_kind == FIGHTER_KIND_ZELDA {
		if status_kind == FIGHTER_STATUS_KIND_SPECIAL_S {
			if ArticleModule::is_exist(module_accessor, *FIGHTER_ZELDA_GENERATE_ARTICLE_DEIN) || ArticleModule::is_exist(module_accessor, *FIGHTER_ZELDA_GENERATE_ARTICLE_DEIN_S) {
				original!()(module_accessor, *FIGHTER_ZELDA_STATUS_KIND_SPECIAL_S_END, arg3)
			}
			else {
				original!()(module_accessor, status_kind, arg3)
			}
		}
		else if status_kind == FIGHTER_STATUS_KIND_SPECIAL_LW && ArticleModule::is_exist(module_accessor, *FIGHTER_ZELDA_GENERATE_ARTICLE_PHANTOM) {
			return 0;
		}
		else {
			original!()(module_accessor, status_kind, arg3)
		}
	}
	else if fighter_kind == FIGHTER_KIND_PZENIGAME || fighter_kind == FIGHTER_KIND_PFUSHIGISOU || fighter_kind == FIGHTER_KIND_PLIZARDON {
		let PT_SPECIAL = &mut FIGHTER_BOOL_5[get_player_number(module_accessor)];
		let cat2 = ControlModule::get_command_flag_cat(module_accessor, 1);
		if ((cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_L) != 0 || (cat2 & *FIGHTER_PAD_CMD_CAT2_FLAG_APPEAL_S_R) != 0) 
		&& status_kind == *FIGHTER_STATUS_KIND_APPEAL
		&& !*PT_SPECIAL {
			return 0;
		}
		else {
			original!()(module_accessor, status_kind, arg3)
		}
	}
	else if fighter_kind == FIGHTER_KIND_SONIC {
		if (motion_kind == hash40("special_s_dash_hi") || motion_kind == hash40("special_s_dash_hop") || motion_kind == hash40("special_s_dash_lw")) && status_kind == FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_END {
			if situation_kind == SITUATION_KIND_GROUND {
				original!()(module_accessor, *FIGHTER_STATUS_KIND_RUN, arg3)
			}
			else {
				original!()(module_accessor, status_kind, arg3)
			}
		}
		else if (motion_kind == hash40("special_s_dash_hi") || motion_kind == hash40("special_s_dash_hop")) && (status_kind == FIGHTER_STATUS_KIND_JUMP_AERIAL || status_kind == FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_TURN){
			return 0;
		}
		else if status_kind == FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_HOLD && situation_kind == SITUATION_KIND_AIR {
			original!()(module_accessor, *FIGHTER_SONIC_STATUS_KIND_SPECIAL_S_DASH, arg3)
		}
		else {
			original!()(module_accessor, status_kind, arg3)
		}
	}
	else if fighter_kind == *FIGHTER_KIND_LITTLEMAC {
		if status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START && WorkModule::get_float(module_accessor, *FIGHTER_LITTLEMAC_INSTANCE_WORK_ID_FLOAT_KO_GAGE) > 30.0 {
			original!()(module_accessor, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2, arg3)
		}
		else if status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_FINAL_DASH && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) { 
			//If we were holding A, skip the dash and skip straight to the uppercut part
			original!()(module_accessor, *FIGHTER_LITTLEMAC_STATUS_KIND_FINAL_ATTACK, arg3)
		}
		else if status_kind == *FIGHTER_LITTLEMAC_STATUS_KIND_FINAL_HIT { 
			//If the game is trying to uppercut by itself, that means we hit the opponent, but since the dash itself now has the hitbox we can just go to the end status
			original!()(module_accessor, *FIGHTER_LITTLEMAC_STATUS_KIND_FINAL_DASH_END, arg3)
		}
		else {
			original!()(module_accessor, status_kind, arg3)
		}
	}
	else if fighter_kind == FIGHTER_KIND_MIIFIGHTER {
		if status_kind == FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_LANDING {
			original!()(module_accessor, *FIGHTER_STATUS_KIND_WAIT, arg3)
		}
		else if status_kind == FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW1_AIR {
			original!()(module_accessor, *FIGHTER_STATUS_KIND_FALL, arg3)
		}
		else if status_kind == FIGHTER_STATUS_KIND_SPECIAL_LW {
			let SMALL_STATE = &mut FIGHTER_INT_1[get_player_number(module_accessor)];
			if *SMALL_STATE != 0 {
				return 0;
			}
			else {
				original!()(module_accessor, status_kind, arg3)
			}
		}
		else {
			original!()(module_accessor, status_kind, arg3)
		}
	}
	else if fighter_kind == FIGHTER_KIND_RYU || fighter_kind == FIGHTER_KIND_KEN {
		if !is_majin(module_accessor, fighter_kind) {
			if status_kind == FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND && TOTAL_FIGHTER == 2 && StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_TURN_RUN {
				PostureModule::reverse_lr(module_accessor);
				original!()(module_accessor, status_kind, arg3)
			}
			else if ControlModule::get_stick_y(module_accessor) >= 0.0 && status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND {
				let cat4 = ControlModule::get_command_flag_cat(module_accessor, 3);
				if fighter_kind == *FIGHTER_KIND_RYU && (cat4 & *FIGHTER_PAD_CMD_CAT4_FLAG_SPECIAL_N2_COMMAND) != 0 {
					original!()(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND, arg3)
				}
				else {
					original!()(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND, arg3)
				}
			}
			else {
				original!()(module_accessor, status_kind, arg3)
			}
		}
		else {
			let SHAKU_INPUT = &mut FIGHTER_FLOAT_3[get_player_number(module_accessor)];
			let G_DEMON_FLIP = &mut FIGHTER_BOOL_3[get_player_number(module_accessor)];
			let RAGING_DEMON = &mut FIGHTER_BOOL_7[get_player_number(module_accessor)];
			let DEMON_FOLLOWUP = &mut FIGHTER_BOOL_6[get_player_number(module_accessor)];
			let DEMON_FLIP_INPUT = &mut FIGHTER_BOOL_5[get_player_number(module_accessor)];
			if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW || (status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND && *DEMON_FLIP_INPUT) {
				if situation_kind == *SITUATION_KIND_GROUND {
					*G_DEMON_FLIP = true;
					original!()(module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, arg3)
				}
				else {
					original!()(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, arg3)
				}
			}
			else if status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_HI_COMMAND {
				if ControlModule::get_stick_y(module_accessor) >= 0.0 {
					original!()(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND, arg3)
				}
				else {
					original!()(module_accessor, status_kind, arg3)
				}
			}
			else if status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_COMMAND  {
				if *SHAKU_INPUT != 0.0 && situation_kind == *SITUATION_KIND_GROUND {
					if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_TURN_RUN {
						*SHAKU_INPUT *= -1.0;
					}
					original!()(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND, arg3)	
				}
				else {
					if TOTAL_FIGHTER == 2 && StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_TURN_RUN {
						PostureModule::reverse_lr(module_accessor);
					}
					original!()(module_accessor, status_kind, arg3)
				}
			}
			else if status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N_COMMAND  {
				if *SHAKU_INPUT != 0.0 && situation_kind == *SITUATION_KIND_GROUND {
					original!()(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND, arg3)	
				}
				else {
					original!()(module_accessor, status_kind, arg3)
				}
			}
			else if status_kind == *FIGHTER_STATUS_KIND_LANDING && StatusModule::status_kind(module_accessor) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_B { //Landing Lag for Demon Flip Kick
				original!()(module_accessor, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, arg3)
			}
			else if status_kind == *FIGHTER_STATUS_KIND_LANDING && StatusModule::status_kind(module_accessor) == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_STEP_F { //Landing "Lag" for Demon Flip Throw
				MotionModule::change_motion(module_accessor, Hash40::new("special_lw_turn"), MotionModule::frame(module_accessor) - 100.0, 1.0, false, 0.0, false, false);
				return 0;
			 }
			 else if status_kind == *FIGHTER_STATUS_KIND_FALL && motion_kind == hash40("special_lw_turn") {
				return 0;
			}
			else if *G_DEMON_FLIP && status_kind == *FIGHTER_STATUS_KIND_JUMP {
				original!()(module_accessor, *FIGHTER_STATUS_KIND_SPECIAL_LW, arg3)
			}
			else if (status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK || status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_LW_ATTACK_TURN) && !*RAGING_DEMON && *DEMON_FOLLOWUP == false {
				original!()(module_accessor, *FIGHTER_STATUS_KIND_FALL, arg3)
			}
			else {
				original!()(module_accessor, status_kind, arg3)
			}
		}
	}
	else if fighter_kind == *FIGHTER_KIND_BAYONETTA {
		if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N && WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE) && 
		(ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) 
		|| ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)) 
		&& ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && !FINAL_WAIT[get_player_number(module_accessor)] {
			FINAL_WAIT[get_player_number(module_accessor)] = true;
			original!()(module_accessor, *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_FIRE, arg3)
		}
		else {
			original!()(module_accessor, status_kind, arg3)
		}
	}
	else if fighter_kind == *FIGHTER_KIND_DOLLY {
		if status_kind == *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_LW_COMMAND && ControlModule::get_stick_y(module_accessor) >= 0.0 {
			original!()(module_accessor, *FIGHTER_DOLLY_STATUS_KIND_SPECIAL_S_COMMAND, arg3)
		}
		else if (status_kind == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL || status_kind == *FIGHTER_DOLLY_STATUS_KIND_SUPER_SPECIAL2) && METER_ENABLED {
			if WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FINAL_AVAILABLE) {
				DRAIN_FULL_METER[get_player_number(module_accessor)] = true;
				SUB_METER[get_player_number(module_accessor)] = 100.0;
			}
			else {
				SUB_METER[get_player_number(module_accessor)] = -100.0;
			}
			FS_METER[get_player_number(module_accessor)] -= 100.0;
			original!()(module_accessor, status_kind, arg3)
		}
		else {
			original!()(module_accessor, status_kind, arg3)
		}
	}
	else if fighter_kind == *FIGHTER_KIND_BRAVE {
		if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
			let SPELL_1 = &mut FIGHTER_INT_2[get_player_number(module_accessor)];
			let SPELL_2 = &mut FIGHTER_INT_3[get_player_number(module_accessor)];
			let SPELL_3 = &mut FIGHTER_INT_4[get_player_number(module_accessor)];
			let SPELL_4 = &mut FIGHTER_INT_5[get_player_number(module_accessor)];
			let PREV_SPELL_1 = &mut FIGHTER_INT_6[get_player_number(module_accessor)];
			let PREV_SPELL_2 = &mut FIGHTER_INT_7[get_player_number(module_accessor)];
			let PREV_SPELL_3 = &mut FIGHTER_INT_8[get_player_number(module_accessor)];
			let PREV_SPELL_4 = &mut FIGHTER_INT_9[get_player_number(module_accessor)];
			let SPELL_MENU_TARGET = &mut FIGHTER_INT_11[get_player_number(module_accessor)];
			let SPELL_SEED_1 = &mut FIGHTER_U8_1[get_player_number(module_accessor)];
			let SPELL_SEED_2 = &mut FIGHTER_U8_2[get_player_number(module_accessor)];
			let SPELL_SEED_3 = &mut FIGHTER_U8_3[get_player_number(module_accessor)];
			let SPELL_SEED_4 = &mut FIGHTER_U8_4[get_player_number(module_accessor)];

			*SPELL_MENU_TARGET = 0;
			if spell_ok(get_player_number(module_accessor), *SPELL_SEED_1) {
				*SPELL_1 = spell_check(*SPELL_SEED_1);
			}
			else {
				while *SPELL_1 == *PREV_SPELL_1
				|| *SPELL_1 == *PREV_SPELL_2
				|| *SPELL_1 == *PREV_SPELL_3
				|| *SPELL_1 == *PREV_SPELL_4 
				|| *SPELL_1 == *SPELL_2 
				|| *SPELL_1 == *SPELL_3 
				|| *SPELL_1 == *SPELL_4 {
					println!("Shuffling 1...");
					*SPELL_1 = spell_check(sv_math::rand(hash40("fighter"), 256) as u8);
				}
			}
			if spell_ok(get_player_number(module_accessor), *SPELL_SEED_2) 
			&& spell_check(*SPELL_SEED_2) != *SPELL_1 {
				*SPELL_2 = spell_check(*SPELL_SEED_2);
			}
			else {
				while *SPELL_2 == *PREV_SPELL_1
				|| *SPELL_2 == *PREV_SPELL_2
				|| *SPELL_2 == *PREV_SPELL_3
				|| *SPELL_2 == *PREV_SPELL_4 
				|| *SPELL_2 == *SPELL_1
				|| *SPELL_2 == *SPELL_3
				|| *SPELL_2 == *SPELL_4 {
					println!("Shuffling 2...");
					*SPELL_2 = spell_check(sv_math::rand(hash40("fighter"), 256) as u8);
				}
			}
			if spell_ok(get_player_number(module_accessor), *SPELL_SEED_3) 
			&& spell_check(*SPELL_SEED_3) != *SPELL_1
			&& spell_check(*SPELL_SEED_3) != *SPELL_2 {
				*SPELL_3 = spell_check(*SPELL_SEED_3);
			}
			else {
				while *SPELL_3 == *PREV_SPELL_1
				|| *SPELL_3 == *PREV_SPELL_2
				|| *SPELL_3 == *PREV_SPELL_3
				|| *SPELL_3 == *PREV_SPELL_4 
				|| *SPELL_3 == *SPELL_1
				|| *SPELL_3 == *SPELL_2
				|| *SPELL_3 == *SPELL_4 {
					println!("Shuffling 3...");
					*SPELL_3 = spell_check(sv_math::rand(hash40("fighter"), 256) as u8);
				}
			}
			if spell_ok(get_player_number(module_accessor), *SPELL_SEED_4) 
			&& spell_check(*SPELL_SEED_4) != *SPELL_1
			&& spell_check(*SPELL_SEED_4) != *SPELL_2
			&& spell_check(*SPELL_SEED_4) != *SPELL_3 {
				*SPELL_4 = spell_check(*SPELL_SEED_4);
			}
			else {
				while *SPELL_4 == *PREV_SPELL_1
				|| *SPELL_4 == *PREV_SPELL_2
				|| *SPELL_4 == *PREV_SPELL_3
				|| *SPELL_4 == *PREV_SPELL_4 
				|| *SPELL_4 == *SPELL_1
				|| *SPELL_4 == *SPELL_2
				|| *SPELL_4 == *SPELL_3 {
					*SPELL_4 = spell_check(sv_math::rand(hash40("fighter"), 256) as u8);
					println!("Shuffling 4...");
				}
			}
			println!("Spell 1: {}", *SPELL_1);
			println!("Spell 2: {}", *SPELL_2);
			println!("Spell 3: {}", *SPELL_3);
			println!("Spell 4: {}", *SPELL_4);
		}
		original!()(module_accessor, status_kind, arg3)
	}
	else if fighter_kind == *FIGHTER_KIND_TANTAN {
		let CAN_ATTACK_JUMP = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
		if (status_kind == *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_SQUAT || status_kind == *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_AERIAL) 
		&& !*CAN_ATTACK_JUMP { //don't let minmin jump out of an arm attack if she's already used both arms
			return 0;
		}
		else {
			original!()(module_accessor, status_kind, arg3)
		}
	}
	else if fighter_kind == *FIGHTER_KIND_PICKEL {
		let PICKEL_JUMP_TIMER = &mut FIGHTER_INT_1[get_player_number(module_accessor)];
		if *PICKEL_JUMP_TIMER > 0 && status_kind == *FIGHTER_PICKEL_STATUS_KIND_SPECIAL_S_JUMP {
			return 0;
		}
		else {
			original!()(module_accessor, status_kind, arg3)
		}
	}
	else if fighter_kind == *FIGHTER_KIND_DEMON {
		if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3 && TOTAL_FIGHTER == 2 {
			let mut not_player = 1;
			if get_player_number(module_accessor) == 1 {
				not_player = 0;
			}
			let facing_right = PostureModule::lr(module_accessor) > 0.0;
			let opponent_is_on_right = PostureModule::pos_x(module_accessor) < PostureModule::pos_x(&mut *get_boma(not_player));
			if facing_right != opponent_is_on_right {
				PostureModule::reverse_lr(module_accessor); 
			}
		}
		original!()(module_accessor, status_kind, arg3)
	}
	else if fighter_kind == WEAPON_KIND_SHIZUE_CLAYROCKET {
		let LLOID_COUNTER = &mut FIGHTER_INT_1[get_player_number(&mut *owner_module_accessor)];
		if StatusModule::status_kind(module_accessor) == *WEAPON_SHIZUE_CLAYROCKET_STATUS_KIND_FLY && (status_kind == *WEAPON_SHIZUE_CLAYROCKET_STATUS_KIND_DISAPPEAR || status_kind == *WEAPON_SHIZUE_CLAYROCKET_STATUS_KIND_BURST) && (*LLOID_COUNTER < 80) {
			return 0;
		}
		else {
			original!()(module_accessor, status_kind, arg3)
		}
	}
	else {
		original!()(module_accessor, status_kind, arg3)
	}
} 

#[skyline::hook(replace = smash::app::lua_bind::MotionModule::add_motion_partial)]
pub unsafe fn add_motion_partial_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
motion_part: i32,
motion_kind: u64,
frame: f32,
rate: f32,
arg6: bool,
arg7: bool,
arg8: f32,
arg9: bool,
arg10: bool,
arg11: bool) -> u64 {
	let fighter_kind = app::utility::get_kind(module_accessor);
	if fighter_kind == *FIGHTER_KIND_TANTAN {
		let LAST_ARM = &mut FIGHTER_INT_3[get_player_number(module_accessor)];
		if WorkModule::is_flag(module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_LUA_TEMP_IS_ATTACK_BOTH) { //note: Check if this flag has been set by the time add_motion_partial is called.
			if motion_part == *FIGHTER_TANTAN_MOTION_PART_SET_KIND_UPPER_BODY_L { //If we're attacking with both arms and an arm wants to change animations, assume that whichever arm isn't changing was the one we sent first
				*LAST_ARM = 2;
			}
			if motion_part == *FIGHTER_TANTAN_MOTION_PART_SET_KIND_UPPER_BODY_R {
				*LAST_ARM = 1;
			}	
		}
		else {
			if motion_part == *FIGHTER_TANTAN_MOTION_PART_SET_KIND_UPPER_BODY_L { //If we're attacking with only one arm, then that's the arm that's changing animations
				*LAST_ARM = 1;
			}
			if motion_part == *FIGHTER_TANTAN_MOTION_PART_SET_KIND_UPPER_BODY_R {
				*LAST_ARM = 2;
			}
		}
	}

	original!()(module_accessor, motion_part, motion_kind, frame, rate, arg6, arg7, arg8, arg9, arg10, arg11)
}

#[skyline::hook(replace = smash::app::lua_bind::MotionModule::change_motion_inherit_frame)]
pub unsafe fn change_motion_inherit_frame_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
motion_kind: u64,
rate: f32,
arg4: f32,
arg5: f32,
arg6: bool,
arg7: bool) -> u64 {
	let fighter_kind = app::utility::get_kind(module_accessor);
	
	if fighter_kind != FIGHTER_KIND_PFUSHIGISOU && fighter_kind != *FIGHTER_KIND_PLIZARDON && !is_majin(module_accessor, fighter_kind) {
		original!()(module_accessor, motion_kind, rate, arg4, arg5, arg6, arg7)
	}
	else if fighter_kind == FIGHTER_KIND_PFUSHIGISOU {
		if motion_kind == hash40("special_s") && (MotionModule::motion_kind(module_accessor) == hash40("appeal_s_r") || MotionModule::motion_kind(module_accessor) == hash40("appeal_s_l")) {
			return 0;
		}
		else {
			original!()(module_accessor, motion_kind, rate, arg4, arg5, arg6, arg7)
		}
	}
	else if fighter_kind == FIGHTER_KIND_PLIZARDON {
		if MotionModule::motion_kind(module_accessor) == hash40("special_air_lw_rock") {
			original!()(module_accessor, hash40("special_lw_rock"), rate, arg4, arg5, arg6, arg7)
		}
		else {
			original!()(module_accessor, motion_kind, rate, arg4, arg5, arg6, arg7)
		}
	}
	else if is_majin(module_accessor, fighter_kind) {
		let MAJIN_ZANKU = &mut FIGHTER_BOOL_4[get_player_number(module_accessor)];
		if motion_kind == hash40("special_lw_turn") && MotionModule::motion_kind(module_accessor) == hash40("special_air_lw_turn") { //Landing Lag for Demon Flip Grab
			if !AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) {
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, true);				
			}
			return 0;
		}
		else if motion_kind == hash40("special_lw") && MotionModule::motion_kind(module_accessor) == hash40("special_air_lw") { //Landing Lag for Demon Flip Punch
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, true);
			return 0;
		}
		else if (motion_kind == hash40("special_n") || motion_kind == hash40("special_n_empty")) && *MAJIN_ZANKU { //Landing Lag for Zanku Hadoken
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, true);
			return 0;	
		}
		else if motion_kind == hash40("special_s") {
			if MotionModule::motion_kind(module_accessor) == hash40("special_air_s") {
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_RYU_STATUS_KIND_SPECIAL_S_END, true);
				return 0;
			}
			else {
				original!()(module_accessor, motion_kind, rate, arg4, arg5, arg6, arg7)
			}
		}
		else {
			original!()(module_accessor, motion_kind, rate, arg4, arg5, arg6, arg7)
		}
	}
	else if fighter_kind == *FIGHTER_KIND_KOOPAG {
		if motion_kind == hash40("special_n_start") {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
			return 0;
		}
		else if motion_kind == hash40("special_air_n_start") {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
			return 0;
		}
		else {
			original!()(module_accessor, motion_kind, rate, arg4, arg5, arg6, arg7)
		}
	}	
	else {
		original!()(module_accessor, motion_kind, rate, arg4, arg5, arg6, arg7)
	}
}

#[skyline::hook(replace = smash::app::lua_bind::MotionModule::change_motion)]
pub unsafe fn motionmodule_change_motion_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
motion_kind: u64,
entry_frame: f32,
rate: f32,
arg5: bool,
arg6: f32,
arg7: bool,
arg8: bool) -> u64 {
	let fighter_kind = app::utility::get_kind(module_accessor);
	let status_kind = StatusModule::status_kind(module_accessor);
	let situation_kind = StatusModule::situation_kind(module_accessor);
	let owner_module_accessor = &mut *app::sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);

	if ((MotionModule::motion_kind(module_accessor) == hash40("attack_11") 
	|| MotionModule::motion_kind(module_accessor) == hash40("attack_s3_s")
	|| MotionModule::motion_kind(module_accessor) == hash40("attack_s3_hi")
	|| MotionModule::motion_kind(module_accessor) == hash40("attack_s3_lw")
	|| MotionModule::motion_kind(module_accessor) == hash40("attack_hi3")
	|| MotionModule::motion_kind(module_accessor) == hash40("attack_lw3")
	|| MotionModule::motion_kind(module_accessor) == hash40("attack_dash")
	|| MotionModule::motion_kind(module_accessor) == hash40("attack_s4_s")
	|| MotionModule::motion_kind(module_accessor) == hash40("attack_s4_hi")
	|| MotionModule::motion_kind(module_accessor) == hash40("attack_s4_lw")) && motion_kind == hash40("jump_squat") && MotionModule::frame(module_accessor) < 2.0) && app::utility::get_category(module_accessor) == *BATTLE_OBJECT_CATEGORY_FIGHTER {
		ATTACK_CANCELED[get_player_number(module_accessor)] = true;
	}

	if fighter_kind == FIGHTER_KIND_PURIN || fighter_kind == FIGHTER_KIND_RIDLEY || fighter_kind == FIGHTER_KIND_BUDDY {
		if (fighter_kind == FIGHTER_KIND_PURIN && (motion_kind == hash40("jump_aerial_f6") || motion_kind == hash40("jump_aerial_f7")))
		|| (fighter_kind == FIGHTER_KIND_RIDLEY && motion_kind == hash40("jump_aerial_f3"))
		|| (fighter_kind == FIGHTER_KIND_BUDDY && (motion_kind == hash40("jump_aerial_f3") || motion_kind == hash40("jump_aerial_f4"))) {
			original!()(module_accessor, hash40("jump_aerial_f2"), entry_frame, rate, arg5, arg6, arg7, arg8)
		}
		else {
			original!()(module_accessor, motion_kind, entry_frame, rate, arg5, arg6, arg7, arg8)
		}
	}
	else if fighter_kind == FIGHTER_KIND_PFUSHIGISOU {
		if motion_kind == hash40("special_s") && (MotionModule::motion_kind(module_accessor) == hash40("appeal_s_r") || MotionModule::motion_kind(module_accessor) == hash40("appeal_s_l")) {
			return 0;
		}
		else {
			original!()(module_accessor, motion_kind, entry_frame, rate, arg5, arg6, arg7, arg8)
		}
	}
	else if fighter_kind == FIGHTER_KIND_SONIC {
		let mut new_motion = motion_kind;
		if motion_kind == hash40("special_air_s_end") && MotionModule::motion_kind(module_accessor) == hash40("special_air_s_end_loop") {
			CancelModule::enable_cancel(module_accessor);
			new_motion = hash40("fall");
		}
		if FINAL_TRANSFORM[get_player_number(module_accessor)] != 0 {
			ArticleModule::change_motion(module_accessor, *FIGHTER_SONIC_GENERATE_ARTICLE_SUPERSONIC, Hash40::new_raw(new_motion), arg5, entry_frame);
		}
		original!()(module_accessor, new_motion, entry_frame, rate, arg5, arg6, arg7, arg8)
	}
	else if fighter_kind == *FIGHTER_KIND_RYU || fighter_kind == *FIGHTER_KIND_KEN {
		let PJAB_CHECK = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
		let PJAB_FRAME = &mut FIGHTER_FLOAT_1[get_player_number(module_accessor)];
		let G_DEMON_FLIP = &mut FIGHTER_BOOL_3[get_player_number(module_accessor)];
		if !is_majin(module_accessor, fighter_kind) {
			let mut new_motion = motion_kind;
			if motion_kind == hash40("attack_11_s") && *PJAB_CHECK {
				new_motion = hash40("attack_11_near_s");
			}
			if motion_kind == hash40("attack_11_near_s") && !*PJAB_CHECK && *PJAB_FRAME == -1.0 {
				new_motion = hash40("attack_11_s");
			}
			original!()(module_accessor, new_motion, entry_frame, rate, arg5, arg6, arg7, arg8)
		}
		else {
			let mut new_motion = motion_kind;
			let mut new_entry_frame = entry_frame;
			if motion_kind == hash40("attack_11_s")
			|| motion_kind == hash40("attack_near_w")
			|| motion_kind == hash40("attack_11_near_s")
			|| motion_kind == hash40("attack_s3_s_s")
			|| motion_kind == hash40("attack_lw3_w")
			|| motion_kind == hash40("attack_lw3_s")
			|| motion_kind == hash40("attack_s4_s")
			|| motion_kind == hash40("attack_s4_hold")
			|| motion_kind == hash40("attack_lw4")
			|| motion_kind == hash40("attack_lw4_hold")
			|| motion_kind == hash40("attack_air_f")
			|| motion_kind == hash40("attack_air_lw")
			|| motion_kind == hash40("special_air_n")
			|| motion_kind == hash40("special_air_n_empty")
			|| status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND
			|| motion_kind == hash40("special_lw")
			|| motion_kind == hash40("special_lw_start")
			|| motion_kind == hash40("special_lw_turn")
			|| motion_kind == hash40("special_lw_step_f")
			|| motion_kind == hash40("special_lw_step_b")
			|| motion_kind == hash40("special_air_lw")
			|| motion_kind == hash40("special_air_lw_start")
			|| motion_kind == hash40("special_air_lw_turn")
			|| motion_kind == hash40("special_air_lw_step_f")
			|| motion_kind == hash40("special_air_lw_step_b")
			|| motion_kind == hash40("appeal_s_r")
			|| motion_kind == hash40("appeal_s_l")
			|| motion_kind == hash40("throw_f")
			|| motion_kind == hash40("final") { //Akuma's animations all start on f100 of Ryu's
				new_entry_frame += 100.0;
			}
			if motion_kind == hash40("final_air2") { //But Ryu's Shinku Hado animation is >100 frames so that move specifically has an exception
				new_entry_frame += 130.0;
			}
			if (motion_kind == hash40("special_lw") || motion_kind == hash40("special_lw_turn")) && (MotionModule::motion_kind(module_accessor) == hash40("special_lw_start") || MotionModule::motion_kind(module_accessor) == hash40("special_air_lw_start")) {
				StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, true);
				return 0;	
			}
			if motion_kind == hash40("attack_11_s") && *PJAB_CHECK {
				new_motion = hash40("attack_11_near_s");
			}
			if motion_kind == hash40("attack_11_near_s") && !*PJAB_CHECK {
				new_motion = hash40("attack_11_s");
			}
			if motion_kind == hash40("special_air_lw_start") && *G_DEMON_FLIP {
				*G_DEMON_FLIP = false;
				original!()(module_accessor, hash40("special_lw_start"), new_entry_frame, rate, arg5, arg6, arg7, arg8)
			}
			else if motion_kind == hash40("special_n") && status_kind == *FIGHTER_RYU_STATUS_KIND_SPECIAL_N2_COMMAND {
				original!()(module_accessor, hash40("special_n2"), new_entry_frame, rate, arg5, arg6, arg7, arg8)
			}
			else {
				original!()(module_accessor, new_motion, new_entry_frame, rate, arg5, arg6, arg7, arg8)
			}
		}
	}
	else if fighter_kind == *FIGHTER_KIND_BAYONETTA {
		if status_kind == *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_FIRE && StatusModule::prev_status_kind(module_accessor, 0) != *FIGHTER_BAYONETTA_STATUS_KIND_SPECIAL_N_CHARGE 
		&& StatusModule::prev_status_kind(module_accessor, 0) != *FIGHTER_STATUS_KIND_SPECIAL_N && FINAL_WAIT[get_player_number(module_accessor)] {
			if situation_kind == *SITUATION_KIND_GROUND {
				original!()(module_accessor, hash40("final_start"), entry_frame, rate * 2.0, arg5, arg6, arg7, arg8)
			}
			else {
				original!()(module_accessor, hash40("final_air_start"), entry_frame, rate * 2.0, arg5, arg6, arg7, arg8)
			}
		}
		else {
			original!()(module_accessor, motion_kind, entry_frame, rate, arg5, arg6, arg7, arg8)
		}
	}
	else if fighter_kind == *FIGHTER_KIND_EFLAME || fighter_kind == *FIGHTER_KIND_ELIGHT {
		let mut new_motion_kind = motion_kind;
		let mut new_entry_frame = entry_frame;
		let SWITCH_MOTION = &mut FIGHTER_U64_1[get_player_number(module_accessor)];
		let SWITCH_FRAME = &mut FIGHTER_FLOAT_1[get_player_number(module_accessor)];
		if motion_kind == hash40("special_lw_out") {
			if *SWITCH_MOTION != 0 {
				new_motion_kind = *SWITCH_MOTION;
				new_entry_frame = *SWITCH_FRAME;
			}
			*SWITCH_MOTION = 0;
			*SWITCH_FRAME = 0.0;
		}

		original!()(module_accessor, new_motion_kind, new_entry_frame, rate, arg5, arg6, arg7, arg8)
	}
	else if fighter_kind == *FIGHTER_KIND_KOOPAG {
		let GIGA_DTILT = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
		let GIGA_DASH_ATTACK = &mut FIGHTER_BOOL_2[get_player_number(module_accessor)];
		let DTILT_INPUT = &mut FIGHTER_BOOL_3[get_player_number(module_accessor)];
		let mut new_motion = motion_kind;
		if motion_kind == hash40("attack_s3_s") {
			if *GIGA_DASH_ATTACK {
				new_motion = hash40("attack_dash");
			}
			if *GIGA_DTILT {
				new_motion = hash40("attack_lw3");
			}
			if GIGA_GRAB[get_player_number(module_accessor)] == 1 {
				new_motion = hash40("catch_pull");
			}
			*DTILT_INPUT = false;
		}
		if motion_kind == hash40("special_n") || motion_kind == hash40("special_n_end") {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
			return 0;	
		}
		if motion_kind == hash40("special_air_n") || motion_kind == hash40("special_air_n_end") {
			StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
			return 0;
		}
		*GIGA_DASH_ATTACK = false;
		*GIGA_DTILT = false;
		original!()(module_accessor, new_motion, entry_frame, rate, arg5, arg6, arg7, arg8)
	}
	else if fighter_kind == WEAPON_KIND_INKLING_INKBULLET {
		if motion_kind == hash40("clash") && WorkModule::is_flag(module_accessor, *WEAPON_INKLING_INKBULLET_INSTANCE_WORK_ID_FLAG_HIT) == false {
			for i in 0..8 {
				if 	BULLET_POS[get_player_number(owner_module_accessor)][i].x + 1.0 < PostureModule::pos_x(module_accessor) && BULLET_POS[get_player_number(owner_module_accessor)][i].y - 1.0 > PostureModule::pos_x(module_accessor) {
					break;	
				}
				else {
					if i == 7 {
						BULLET_POS[get_slot(get_player_number(owner_module_accessor))][get_player_number(owner_module_accessor)].x = PostureModule::pos_x(module_accessor) - 8.0;
						BULLET_POS[get_slot(get_player_number(owner_module_accessor))][get_player_number(owner_module_accessor)].y = PostureModule::pos_x(module_accessor) + 8.0;
						BULLET_POS[get_slot(get_player_number(owner_module_accessor))][get_player_number(owner_module_accessor)].z = PostureModule::pos_y(module_accessor) - 2.0;
						BULLET_POS[get_slot(get_player_number(owner_module_accessor))][get_player_number(owner_module_accessor)].w = 480.0;
					}
				}
			}
		}
		original!()(module_accessor, motion_kind, entry_frame, rate, arg5, arg6, arg7, arg8)
	}
	else if fighter_kind == WEAPON_KIND_SHIZUE_CLAYROCKET {
		let LLOID_COUNTER = &mut FIGHTER_INT_1[get_player_number(&mut *owner_module_accessor)];
		if motion_kind == hash40("burst") && *LLOID_COUNTER < 80 {
			original!()(module_accessor, hash40("fly"), entry_frame, rate, arg5, arg6, arg7, arg8)
		}
		else {
			original!()(module_accessor, motion_kind, entry_frame, rate, arg5, arg6, arg7, arg8)
		}
	}
	else {
		original!()(module_accessor, motion_kind, entry_frame, rate, arg5, arg6, arg7, arg8)
	}
}


#[skyline::hook(replace = smash::app::lua_bind::ArticleModule::remove_exist)]
pub unsafe fn remove_exist_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
article: i32,
aot: i32) -> u64 {
	let fighter_kind = app::utility::get_kind(module_accessor);
	if fighter_kind == FIGHTER_KIND_SONIC && article == *FIGHTER_SONIC_GENERATE_ARTICLE_SUPERSONIC {
		if FINAL_TRANSFORM[get_player_number(module_accessor)] != 0 {
			return 0;
		}
	}	
	original!()(module_accessor, article, aot)
}

#[skyline::hook(replace = smash::app::lua_bind::ArticleModule::change_motion)]
pub unsafe fn articlemodule_change_motion_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
article: i32,
motion_kind: u64,
arg4: bool,
entry_frame: f32) -> u64 {
	let fighter_kind = app::utility::get_kind(module_accessor);
	
	if fighter_kind == FIGHTER_KIND_INKLING && article == *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID && INKED[get_player_number(module_accessor)] != 2 && 
	(motion_kind == hash40("turn_dash_r") 
	|| motion_kind == hash40("turn_dash_l")
	|| motion_kind == hash40("dash_r")
	|| motion_kind == hash40("dash_l")) {
		return 0;
	}
	else {
		original!()(module_accessor, article, motion_kind, arg4, entry_frame)
	}
}

#[skyline::hook(replace = smash::app::lua_bind::ArticleModule::set_frame)]
pub unsafe fn set_frame_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
article: i32,
frame: f32) -> u64 {
	let fighter_kind = app::utility::get_kind(module_accessor);
	let motion_kind = MotionModule::motion_kind(module_accessor);
	
	if fighter_kind == FIGHTER_KIND_INKLING && article == *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID && (motion_kind == hash40("dash") || motion_kind == hash40("turn_dash")) && INKED[get_player_number(module_accessor)] != 2 {
		return 0;
	}
	else {
		original!()(module_accessor, article, frame)
	}
} 

#[skyline::hook(replace = smash::app::lua_bind::ArticleModule::set_rate)]
pub unsafe fn set_rate_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
article: i32,
rate: f32) -> u64 {
	let fighter_kind = app::utility::get_kind(module_accessor);
	let motion_kind = MotionModule::motion_kind(module_accessor);
	
	if fighter_kind == FIGHTER_KIND_INKLING && article == *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID && (motion_kind == hash40("dash") || motion_kind == hash40("turn_dash")) && INKED[get_player_number(module_accessor)] != 2 {
		return 0;
	}
	else {
		original!()(module_accessor, article, rate)
	}
} 

#[skyline::hook(replace = smash::app::lua_bind::ArticleModule::set_visibility_whole)]
pub unsafe fn set_visibility_whole_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
article: i32,
visibility: bool,
aot: i32) -> u64 {
	let fighter_kind = app::utility::get_kind(module_accessor);
	let motion_kind = MotionModule::motion_kind(module_accessor);

	if fighter_kind == FIGHTER_KIND_INKLING && article == *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID && (motion_kind == hash40("dash") || motion_kind == hash40("turn_dash")) && INKED[get_player_number(module_accessor)] != 2 {
		return 0;
	}
/*	if fighter_kind == FIGHTER_KIND_TOONLINK && article == *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOWARROW && motion_kind != hash40("attack_s4_s") && visibility {
		return 0;

		//Toon Link Skull Hammer invisibility during Neutral B, WIP
	}*/
	else {
		original!()(module_accessor, article, visibility, aot)
	}
}

#[skyline::hook(replace = smash::app::lua_bind::VisibilityModule::set_whole)]
pub unsafe fn set_whole_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
visibility: bool) -> u64 {
	let fighter_kind = app::utility::get_kind(module_accessor);
	let motion_kind = MotionModule::motion_kind(module_accessor);

	if fighter_kind == FIGHTER_KIND_INKLING && (motion_kind == hash40("dash") || motion_kind == hash40("turn_dash")) && INKED[get_player_number(module_accessor)] != 2 {
		original!()(module_accessor, true)
	}
	else {
		original!()(module_accessor, visibility)
	}
}

#[skyline::hook(replace = smash::app::lua_bind::StatusModule::init_settings)]
pub unsafe fn init_settings_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
situation_kind: i32,
arg3: i32, 
arg4: u64,
ground_cliff_check_kind: u64,
arg6: bool,
arg7: i32,
arg8: i32,
arg9: i32,
arg10: i32) -> u64 {
	let status_kind = StatusModule::status_kind(module_accessor);
	let fighter_kind = app::utility::get_kind(module_accessor);

	if status_kind != *FIGHTER_STATUS_KIND_APPEAL
	&& status_kind != *FIGHTER_STATUS_KIND_DASH
	&& status_kind != *FIGHTER_STATUS_KIND_TURN
	&& status_kind != *FIGHTER_STATUS_KIND_TURN_DASH
	&& status_kind != *FIGHTER_STATUS_KIND_LANDING
	&& status_kind != *FIGHTER_STATUS_KIND_LANDING_LIGHT
	&& status_kind != *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR
	&& status_kind != *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL
	&& status_kind != *FIGHTER_STATUS_KIND_ESCAPE_AIR
	&& fighter_kind != *FIGHTER_KIND_BUDDY {
		original!()(module_accessor, situation_kind, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
	}
	else if status_kind == *FIGHTER_STATUS_KIND_APPEAL
	|| status_kind == *FIGHTER_STATUS_KIND_DASH
	|| status_kind == *FIGHTER_STATUS_KIND_TURN
	|| status_kind == *FIGHTER_STATUS_KIND_TURN_DASH
	|| status_kind == *FIGHTER_STATUS_KIND_LANDING
	|| status_kind == *FIGHTER_STATUS_KIND_LANDING_LIGHT
	|| status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR
	|| status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL {
		original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
	}
	else if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
		if fighter_kind == *FIGHTER_KIND_GEKKOUGA && FINAL_TRANSFORM[get_player_number(module_accessor)] != 0 {
			original!()(module_accessor, situation_kind, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
		}
		else {
			if ControlModule::get_stick_y(module_accessor) >= 0.66 {
				original!()(module_accessor, situation_kind, arg3, 1 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
			}
			else {
				original!()(module_accessor, situation_kind, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)			
			}
		}
	}
	else if fighter_kind == FIGHTER_KIND_BUDDY {
		if (status_kind == *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH || status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S) && situation_kind == SITUATION_KIND_GROUND {
			original!()(module_accessor, situation_kind, arg3, 7 as u64, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
		}
		else {
			original!()(module_accessor, situation_kind, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)			
		}
	}  
	else {
		original!()(module_accessor, situation_kind, arg3, arg4, ground_cliff_check_kind, arg6, arg7, arg8, arg9, arg10)
	}
}

#[skyline::hook(replace = smash::app::lua_bind::GroundModule::correct)]
pub unsafe fn correct_replace(
module_accessor: &mut app::BattleObjectModuleAccessor,
ground_correct_kind: u32) -> u64 {
	let status_kind = StatusModule::status_kind(module_accessor);
	let fighter_kind = app::utility::get_kind(module_accessor);

	if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR 
	|| status_kind == *FIGHTER_STATUS_KIND_LANDING
	|| status_kind == *FIGHTER_STATUS_KIND_TURN_DASH 
	|| status_kind == *FIGHTER_STATUS_KIND_DASH 
	|| status_kind == *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL {
		original!()(module_accessor, 1 as u32)
	}
	else if (fighter_kind == FIGHTER_KIND_PIKACHU && status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK)
	|| (fighter_kind == FIGHTER_KIND_PIKACHU && status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK)
	|| (fighter_kind == FIGHTER_KIND_PIKACHU && status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END)
	|| (fighter_kind == FIGHTER_KIND_PICHU && status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_WEAK)
	|| (fighter_kind == FIGHTER_KIND_PICHU && status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK)
	|| (fighter_kind == FIGHTER_KIND_PICHU && status_kind == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_END)
	|| (fighter_kind == FIGHTER_KIND_FOX && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S)
	|| (fighter_kind == FIGHTER_KIND_FALCO && status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S)
	|| (fighter_kind == FIGHTER_KIND_CAPTAIN && status_kind == *FIGHTER_CAPTAIN_STATUS_KIND_SPECIAL_LW_END)
	|| (fighter_kind == FIGHTER_KIND_MIIFIGHTER && status_kind == *FIGHTER_MIIFIGHTER_STATUS_KIND_SPECIAL_LW2_KICK_LANDING) {
		original!()(module_accessor, *GROUND_CORRECT_KIND_GROUND as u32)
	}
	else {
		original!()(module_accessor, ground_correct_kind)
	}
}

#[skyline::hook(replace = smash::app::WeaponSpecializer_PTrainerPTrainer::request_change_pokemon)]
pub unsafe fn request_change_pokemon_replace(
instance: &mut app::Fighter) -> u64 {
	let module_accessor = &mut *app::sv_battle_object::module_accessor(*(((&mut (instance.battle_object) as *mut app::BattleObject) as u64 + 8) as *mut u32));
	let PT_SWITCH = &mut FIGHTER_BOOL_4[get_player_number(module_accessor)];
	let PT_SPECIAL = &mut FIGHTER_BOOL_5[get_player_number(module_accessor)];

	if *PT_SWITCH == false {
		*PT_SPECIAL = true;
		return 0;
	}
	else {
		original!()(instance)
	}
}

#[skyline::hook(replace = smash::app::FighterSpecializer_Brave::get_special_lw_command_from_index)]
pub unsafe fn get_special_lw_command_from_index_replace(
instance: &mut app::Fighter,
index: i32) -> i32 {
	let module_accessor = app::sv_battle_object::module_accessor(*(((&mut (instance.battle_object) as *mut app::BattleObject) as u64 + 8) as *mut u32));
	let SEED = &mut FIGHTER_INT_1[get_player_number(&mut *module_accessor)];
	let SPELL_RNG = &mut FIGHTER_BOOL_2[get_player_number(&mut *module_accessor)];
	let SPELL_1 = &mut FIGHTER_INT_2[get_player_number(&mut *module_accessor)];
	let SPELL_2 = &mut FIGHTER_INT_3[get_player_number(&mut *module_accessor)];
	let SPELL_3 = &mut FIGHTER_INT_4[get_player_number(&mut *module_accessor)];
	let SPELL_4 = &mut FIGHTER_INT_5[get_player_number(&mut *module_accessor)];

	if *SPELL_RNG {
		if index == 0 {
			let ret = *SPELL_1;
			*SEED += *SPELL_1 + 1;
			return ret;
		}
		else if index == 1 {
			let ret = *SPELL_2;
			*SEED += 2 * (*SPELL_2 + 1);
			return ret;
		}
		else if index == 2 {
			let ret = *SPELL_3;
			*SEED += 3 * (*SPELL_3 + 1);
			return ret;
		}
		else {
			let ret = *SPELL_4;
			*SEED += 4 * (*SPELL_4 + 1);
			return ret;
		}
	}
	else {
		original!()(instance, index)
	}
}

#[skyline::hook(offset=0x3f0000)]
pub unsafe fn game_main_replace(
	arg1: u64
) {
	println!("Frame Counter: {}", FRAME_COUNTER);
	println!("Collision Processor Arg 1: {}", arg1);
	original!()(arg1)
}

#[skyline::hook(offset=0x3e65b0)]
pub unsafe fn collision_func_replace(
	p1: u64,
	p2: u64,
	arg3: u32,
) {
	println!("P1: {:#X}", p1);
	println!("P2: {:#X}", p2);
	println!("Arg3: {}", arg3);

	original!()(p1, p2, arg3)
}

#[skyline::hook(offset=0x857020)]
pub unsafe fn rng_modifier_replace(
	instance: &mut app::Fighter,
	some_module: u64,
	ret: &mut i32
) {
	let module_accessor = &mut *app::sv_battle_object::module_accessor(*(((&mut (instance.battle_object) as *mut app::BattleObject) as u64 + 8) as *mut u32));
	let SPELL_MENU_TARGET = &mut FIGHTER_INT_11[get_player_number(module_accessor)];
	*SPELL_MENU_TARGET+=1;
	original!()(instance, some_module, ret)
}

fn nro_main(nro: &skyline::nro::NroInfo) {	
	unsafe {
		HIGH_SPAWN_POS = Vector3f{x: 30.0, y: 0.0, z: 0.0};
		LOW_SPAWN_POS = Vector3f{x: -30.0, y: 0.0, z: 0.0};
		TRAINING_SPAWN = 0;
	}
	match nro.name {
		"common" => { 
			custom::install();
			mario::install();
			donkey::install();
			link::install(); 
			samus::install();
			samusd::install();
			yoshi::install();
			kirby::install();
			fox::install();
			pikachu::install();
			luigi::install();
			ness::install();
			captain::install();
			purin::install();
			peach::install();
			daisy::install();
			koopa::install();
			popo_nana::install();
			sheik::install();
			zelda::install();
			mariod::install();
			pichu::install();
			falco::install();
			marth::install();
			lucina::install();
			younglink::install();
			ganon::install();
			mewtwo::install();
			koopag::install();
			roy::install();
			chrom::install();
			gamewatch::install();
			metaknight::install();
			pit::install();
			pitb::install();
			szerosuit::install();
			wario::install();
			snake::install();
			ike::install();
			ptrainer::install();
			diddy::install();
			lucas::install();
			sonic::install();
			dedede::install();
			pikmin::install();
			lucario::install();
			robot::install();
			toonlink::install();
			wolf::install();
			murabito::install();
			rockman::install();
			wiifit::install();
			rosetta::install();
			littlemac::install();
			gekkouga::install();
			miifighter::install();
			miiswordsman::install();
			miigunner::install();
			palutena::install();
			pacman::install();
			reflet::install();
			shulk::install();
			koopajr::install();
			duckhunt::install();
			ryu::install();
			ken::install();
			cloud::install();
			kamui::install();
			bayonetta::install();
			inkling::install();
			ridley::install();
			simon::install();
			richter::install();
			krool::install();
			shizue::install();
			gaogaen::install();
			packun::install();
			jack::install();
			brave::install();
			buddy::install();
			dolly::install();
			master::install();
			tantan::install();
			pickel::install();
			edge::install();
			eflame::install();
			elight::install();
			demon::install();
		}
		_ => (),
	}
}

#[skyline::main(name = "ce")]
pub fn main() {
	skyline::nro::add_hook(nro_main).unwrap();
	skyline::install_hook!(offset_dump);
//	skyline::install_hook!(rng_modifier_replace);
//	skyline::install_hook!(game_main_replace);
//	skyline::install_hook!(collision_func_replace);
	skyline::install_hook!(is_valid_just_shield_reflector_replace);
	skyline::install_hook!(is_valid_just_shield_replace);
	skyline::install_hook!(request_change_pokemon_replace);
	skyline::install_hook!(sv_module_access_effect_replace);
	skyline::install_hook!(remove_exist_replace);
	skyline::install_hook!(play_sequence_replace);
	skyline::install_hook!(play_status_replace);
	skyline::install_hook!(play_fly_voice_replace);
	skyline::install_hook!(play_se_level_1_replace); 
	skyline::install_hook!(get_float_replace);
	skyline::install_hook!(get_int_replace);
//	skyline::install_hook!(get_param_helper_replace);
//	skyline::install_hook!(get_param_replace);
	skyline::install_hook!(get_param_float_replace);	
	skyline::install_hook!(get_param_float_lvl_1_replace); 
	skyline::install_hook!(get_param_int_replace);
	skyline::install_hook!(get_param_int_lvl_1_replace); 
	skyline::install_hook!(set_mesh_visibility_lvl_2_replace); 
	skyline::install_hook!(get_attack_air_kind_replace);
	skyline::install_hook!(generate_article_replace);
	skyline::install_hook!(set_rebound_replace);	
	skyline::install_hook!(on_flag_replace);
	skyline::install_hook!(off_flag_replace);
	skyline::install_hook!(init_settings_replace);
	skyline::install_hook!(is_enable_transition_term_replace);
	skyline::install_hook!(add_motion_partial_replace); 
	skyline::install_hook!(change_motion_inherit_frame_replace);
	skyline::install_hook!(motionmodule_change_motion_replace); 
	skyline::install_hook!(articlemodule_change_motion_replace);
	skyline::install_hook!(set_frame_replace);
	skyline::install_hook!(set_rate_replace);
	skyline::install_hook!(set_visibility_whole_replace);
	skyline::install_hook!(set_whole_replace);
	skyline::install_hook!(change_status_request_from_script_replace); 
	skyline::install_hook!(correct_replace);
	skyline::install_hook!(change_kinetic_replace);
	skyline::install_hook!(get_special_lw_command_from_index_replace);
	skyline::install_hook!(notify_log_event_collision_hit_replace);
}