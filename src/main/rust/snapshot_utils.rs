//============================================================================//
//                                                                            //
//                         Copyright © 2015 Sandpolis                         //
//                                                                            //
//  This source file is subject to the terms of the Mozilla Public License    //
//  version 2. You may not use this file except in compliance with the MPL    //
//  as published by the Mozilla Foundation.                                   //
//                                                                            //
//============================================================================//

use crate::core::net::message::MSG;
use crate::lib::connection::Connection;
use crate::plugin::snapshot::msg_snapshot::*;
use fasthash::murmur3;
use log::{debug, error};
use anyhow::Error;

pub struct BlockSnapshotMetadata {

}

pub struct BlockSnapshotter<'a> {
	pub block_size: usize,
	pub connection: &'a mut Connection,
}

impl BlockSnapshotter<'_> {

	pub fn read(&mut self, device_path: String) -> Result<(), Error> {

		let device = File::open(device_path)?;
		let data = unsafe { MmapOptions::new().map(&device)? };

		for i in (0_usize..device.metadata()?.len() as usize).step_by(self.block_size) {
			let block = &data[i..(i + self.block_size)];
			let hash = murmur3::hash128(&block);

			// TODO check with metadata before sending

			// Send update
			let mut ev_snapshot = EV_SnapshotDataBlock::new();
			ev_snapshot.data = block.to_vec();
			let ev = MSG::new();
			self.connection.send(&ev);
		}
		return Ok(())
	}
}
