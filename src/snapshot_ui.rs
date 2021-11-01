//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

use log::{debug, error};
use core::fmt::format;
use std::{thread, time};
use alloc::vec::Vec;

enum UiBlockState {

	/// The block has not been read
	Unseen,

	// The block is currently being read into memory and hashed
	Hashing,

	// The block has been hashed and can be removed from memory if it does not need to be synced
	Hashed,

	// The block is currently being transferred to or from a remote location
	Syncing,

	// The block has been synced successfully and can be removed from memory
	Synced,
}

/// UiBlock is a representation of a filesystem block in the UI.
struct UiBlock<'a> {

	/// The current state of this block
	pub state: UiBlockState,

	/// The block's local window
	pub win: &'a Window,

	/// The block's Y coordinate in the local window
	pub y: i32,

	/// The block's X coordinate in the local window
	pub x: i32,

	/// The block's global index
	pub index: i32,
}

impl UiBlock<'_> {

	pub fn refresh(&self) {
		
		// Determine the block's appearance according to state
		let ch = match self.state {
			UiBlockState::Unseen => ".",
			UiBlockState::Hashed => "▢",
			UiBlockState::Synced => "▣",
			_ => ".",
		};

		self.win.mvaddstr(self.y, self.x, &ch);
		self.win.refresh();
	}
}

/// The global state of the UI.
struct UiState<'a> {

	pub dialog_topleft_y: i32,
	pub dialog_topleft_x: i32,
	pub dialog_topright_y: i32,
	pub dialog_topright_x: i32,
	pub dialog_bottomleft_y: i32,
	pub dialog_bottomleft_x: i32,
	pub dialog_bottomright_y: i32,
	pub dialog_bottomright_x: i32,

	pub terminal_y: i32,
	pub terminal_x: i32,

	pub blocks_max: i32,

	pub win_stats: Window,
	pub win_header: Window,

	pub win_blocks_north: Window,
	pub win_blocks_west: Window,
	pub win_blocks_east: Window,
	pub win_blocks_south: Window,

	pub blocks: Vec<UiBlock<'a>>,
}

impl UiState<'_> {
	pub const TERM_WIDTH_MIN: i32 = 32;
	pub const TERM_HEIGHT_MIN: i32 = 16;

	pub const DIALOG_WIDTH: i32 = 89;
	pub const DIALOG_HEIGHT: i32 = 18;

	pub const HEADER_WIDTH: i32 = 89;
	pub const HEADER_HEIGHT: i32 = 9;

	pub const STATS_WIDTH: i32 = 89;
	pub const STATS_HEIGHT: i32 = 9;
}

fn init_win_header(state: &UiState) {

	// Write sixel image
	//if let Some(sixel) = BinaryAssets::get("sandpolis.sixel") {
	//	state.win_header.mv(0, 0);
	//	print!(String::from_utf8_lossy(&sixel));
	//}
}

// Initialize the stats window and static content
fn init_win_stats(state: &UiState) {

	state.win_stats.mvaddstr(0, 0, "Creating snapshot of device: /dev/sda1. Please do not interrupt this process.");

	state.win_stats.mvaddstr(2, 0, "Time remaining");
	state.win_stats.mvaddstr(3, 0, "Network upload");
	state.win_stats.mvaddstr(4, 0, "Network download");
	state.win_stats.mvaddstr(5, 0, "Disk Read");
	state.win_stats.mvaddstr(6, 0, "Disk Write");

	state.win_stats.refresh();
}

fn update_win_stats(state: &UiState) {

	let time_remaining = "";
	let net_upload = format!("↑ /s");
	let net_download = format!("↓ /s");
	let disk_read = format!("↑ /s");
	let disk_write = format!("↓ /s");

	state.win_stats.mvaddstr(2, UiState::DIALOG_WIDTH - time_remaining.chars().count() as i32, time_remaining);
	state.win_stats.mvaddstr(3, UiState::DIALOG_WIDTH - net_upload.chars().count() as i32, net_upload);
	state.win_stats.mvaddstr(4, UiState::DIALOG_WIDTH - net_download.chars().count() as i32, net_download);
	state.win_stats.mvaddstr(5, UiState::DIALOG_WIDTH - disk_read.chars().count() as i32, disk_read);
	state.win_stats.mvaddstr(6, UiState::DIALOG_WIDTH - disk_write.chars().count() as i32, disk_write);

	state.win_stats.refresh();
}

fn init_win_blocks(state: &UiState) {

	// Prepare an offset which will be used to create a rectangular gap
	let mut offset = 0;
	let mut i = -1;

	// Allocate blocks
	while i < 10 {
		i += 1;

		// Convert block index to absolute coordinates
		let mut y = (i + offset) / state.terminal_x;
		let mut x = (i + offset) % state.terminal_x;

		// If we're about to enter the dialog, add offset and retry this iteration
		if y >= state.dialog_topleft_y && y <= state.dialog_bottomleft_y && x == state.dialog_topleft_x {
			offset += UiState::DIALOG_WIDTH;
			i -= 1;
			continue;
		}

		// Convert absolute coordinates to relative and determine the window
		let local_window = 
			if y < state.dialog_topleft_y {
				&state.win_blocks_north
			} else if y >= state.dialog_topleft_y && y <= state.dialog_bottomleft_y && x < state.dialog_topleft_x {
				y -= state.dialog_topleft_y;
				&state.win_blocks_west
			} else if y >= state.dialog_topleft_y && y <= state.dialog_bottomleft_y && x > state.dialog_topright_x {
				y -= state.dialog_topleft_y;
				x -= state.dialog_topright_x;
				&state.win_blocks_east
			} else {
				y -= state.dialog_bottomleft_y;
				&state.win_blocks_south
			};

		// Create the block
		let blk = UiBlock {
			state: UiBlockState::Unseen,
			win: local_window,
			y: y,
			x: x,
			index: i,
		};
		//state.blocks.push(blk);
	}
}

/// Show the UI until the user quits.
pub fn start() {

	debug!("Initializing root window");
	let root = initscr();

	// Precondition check on terminal size
	let (term_y, term_x) = root.get_max_yx();
	debug!("Terminal size: ({} x {})", term_x, term_y);
	if term_y < UiState::TERM_HEIGHT_MIN || term_x < UiState::TERM_WIDTH_MIN {
		cleanup();
		error!("Terminal dimensions too small");
		std::process::exit(1);
	}

	if ! has_colors() {
		cleanup();
		error!("Terminal does not support colors");
		std::process::exit(1);
	}

	start_color();
	noecho();
	nonl();
	raw();
	cbreak();
	curs_set(0);

	init_color(COLOR_BLACK, 188, 188, 188);

	let dialog_topleft_y = (term_y - UiState::DIALOG_HEIGHT) / 2;
	let dialog_topleft_x = (term_x - UiState::DIALOG_WIDTH) / 2;
	let dialog_topright_y = (term_y - UiState::DIALOG_HEIGHT) / 2;
	let dialog_topright_x = (term_x + UiState::DIALOG_WIDTH) / 2;
	let dialog_bottomleft_y = (term_y + UiState::DIALOG_HEIGHT) / 2;
	let dialog_bottomleft_x = (term_x - UiState::DIALOG_WIDTH) / 2;
	let dialog_bottomright_y = (term_y + UiState::DIALOG_HEIGHT) / 2;
	let dialog_bottomright_x = (term_x + UiState::DIALOG_WIDTH) / 2;

	// Initialize UI state
	let state = UiState {
		dialog_topleft_y: dialog_topleft_y,
		dialog_topleft_x: dialog_topleft_x,
		dialog_topright_y: dialog_topright_y,
		dialog_topright_x: dialog_topright_x,
		dialog_bottomleft_y: dialog_bottomleft_y,
		dialog_bottomleft_x: dialog_bottomleft_x,
		dialog_bottomright_y: dialog_bottomright_y,
		dialog_bottomright_x: dialog_bottomright_x,

		terminal_y: term_y,
		terminal_x: term_x,

		blocks_max: (term_x * term_y) - (UiState::DIALOG_WIDTH * UiState::DIALOG_HEIGHT),

		win_stats: newwin(UiState::STATS_HEIGHT, UiState::STATS_WIDTH, dialog_topleft_y + UiState::HEADER_HEIGHT + 1, dialog_topleft_x),
		win_header: newwin(UiState::HEADER_HEIGHT, UiState::HEADER_WIDTH, dialog_topleft_y, dialog_topleft_x),

		win_blocks_north: newwin(dialog_topleft_y, term_x, 0, 0),
		win_blocks_west: newwin(UiState::DIALOG_HEIGHT, dialog_topleft_x - 1, dialog_topleft_y, 0),
		win_blocks_east: newwin(UiState::DIALOG_HEIGHT, term_x - dialog_topright_x + 1, dialog_topright_y, dialog_topright_x + 1),
		win_blocks_south: newwin(term_y - dialog_bottomleft_y, term_x, dialog_bottomleft_y - 1, 0),

		blocks: Vec::new(),
	};

	init_win_header(&state);
	init_win_stats(&state);
	init_win_blocks(&state);

	for block in state.blocks {
		block.refresh();
	}

	// Start UI update loop
	loop {
		thread::sleep(time::Duration::from_millis(200));
	}

	cleanup();
}

/// Reset the terminal to its previous state.
fn cleanup() {
	nocbreak();
	noraw();
	nl();
	echo();
	endwin();
}
